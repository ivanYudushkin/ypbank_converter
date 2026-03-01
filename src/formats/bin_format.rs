use crate::transaction::{Transaction, TransactionType, StatusType};
use std::io::{Write}; 
use crate::error::handle_read_error;
use colored::*;

//Функция записывает u32 в big-endian
fn write_u32_be<W: Write>(writer: &mut W, value: u32) -> std::io::Result<()> {
    writer.write_all(&value.to_be_bytes())
}

//Функция записывает u64 в big-endian
fn write_u64_be<W: Write>(writer: &mut W, value: u64) -> std::io::Result<()> {
    writer.write_all(&value.to_be_bytes())
}

//Функция записывает i64 в big-endian
fn write_i64_be<W: Write>(writer: &mut W, value: i64) -> std::io::Result<()> {
    writer.write_all(&value.to_be_bytes())
}

impl Transaction {
    //Метод структцры Transaction для записи транзакции в любой writer поддерживающий Write в формате YPBankBin 
    fn write_transaction_bin<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        const MAGIC: [u8; 4] = [0x59, 0x50, 0x42, 0x4E];

        let desc_len = if let Some(description) = &self.description {
            description.len() as u32
        } else {
            0
        };

        let record_size = 8*5 + 1*2 + 4 + desc_len;

        writer.write_all(&MAGIC)?;
        write_u32_be(writer, record_size)?;

        match self.tx_id {
            Some(id) => write_u64_be(writer, id)?,
            None => write_u64_be(writer, 0)?,
        }

        let tx_type_byte = match self.tx_type {
            Some(TransactionType::DEPOSIT) => 0u8,
            Some(TransactionType::TRANSFER) => 1u8,
            Some(TransactionType::WITHDRAWAL) => 2u8,
            None => 0u8,
        };
        writer.write_all(&[tx_type_byte])?;

        match self.from_user_id {
            Some(id) => write_u64_be(writer, id)?,
            None => write_u64_be(writer, 0)?,
        }
        
        // TO_USER_ID
        match self.to_user_id {
            Some(id) => write_u64_be(writer, id)?,
            None => write_u64_be(writer, 0)?,
        }
        
        // AMOUNT (i64)
        match self.amount {
            Some(amount) => write_i64_be(writer, amount as i64)?,
            None => write_i64_be(writer, 0)?,
        }
        
        // TIMESTAMP
        match self.timestamp {
            Some(ts) => write_u64_be(writer, ts)?,
            None => write_u64_be(writer, 0)?,
        }
        
        // STATUS
        let status_byte = match self.status {
            Some(StatusType::SUCCESS) => 0u8,
            Some(StatusType::FAILURE) => 1u8,
            Some(StatusType::PENDING) => 2u8,
            None => 0u8,
        };

        writer.write_all(&[status_byte])?;
        
        // DESC_LEN
        write_u32_be(writer, desc_len)?;
        
        // DESCRIPTION
        if desc_len > 0 {
            if let Some(ref description) = self.description {
                writer.write_all(description.as_bytes())?;
            }
        }

        Ok(())
    }
}

//Чтение источника соответствующего формату YPBankBin и приведение его к типу Vec<Transaction>
pub fn from_read<R: std::io::Read>(reader: &mut R) -> Result<Vec<Transaction>, std::io::Error> {
    //Инициализация вектора транзакций
    let mut vec_transactions: Vec<Transaction> = Vec::new();
    const MAGIC: [u8; 4] = [0x59, 0x50, 0x42, 0x4E];
    //Счетчик транзакций для вывода предупреждений
    let mut num_transaction = 0;
    //Читаем файл по байтам
    loop {
        num_transaction += 1;
        let mut magic = [0u8; 4];
        //Читаем магическое слово
        match reader.read_exact(&mut magic) {
            Ok(_) => {
                if magic != MAGIC {
                    //Если не получается, то выводим ошибку о несоотвествии формата
                    return Err(handle_read_error("bin", std::io::Error::new(std::io::ErrorKind::InvalidData,"")));
                }
            }
            //Выход из цикла
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                break;  
            }
            Err(e) => return Err(e),
        }
    
        //Обрабатываем инфо о размере записи
        let mut record_size = [0u8; 4];
        reader.read_exact(&mut record_size)?;
    
        let mut transaction = Transaction::new();

        //Задаем размеры в байтах
        let mut bytes = [0u8; 8];
        let mut byte = [0u8; 1];
    
        //Читаем по срезам и вытаскиваем инфо о транзакциях 

        reader.read_exact(&mut bytes)?;
        transaction.tx_id = Some(u64::from_be_bytes(bytes));
    
        reader.read_exact(&mut byte)?;
        transaction.tx_type = match byte[0] {
            0 => Some(TransactionType::DEPOSIT),
            1 => Some(TransactionType::TRANSFER),
            2 => Some(TransactionType::WITHDRAWAL),
            _ => {
                eprintln!("{}", format!("Предупреждение: неизвестный тип транзакции '{}' в поле TX_TYPE. Номер транзакции: {}", byte[0].to_string().yellow().bold(), num_transaction.to_string().yellow().bold()).yellow());
                None
            },
        };
    
        reader.read_exact(&mut bytes)?;
        transaction.from_user_id = Some(u64::from_be_bytes(bytes));
    
        reader.read_exact(&mut bytes)?;
        transaction.to_user_id = Some(u64::from_be_bytes(bytes));
    
        reader.read_exact(&mut bytes)?;
        transaction.amount = Some(u64::from_be_bytes(bytes));
    
        reader.read_exact(&mut bytes)?;
        transaction.timestamp = Some(u64::from_be_bytes(bytes));
    
        reader.read_exact(&mut byte)?;
        transaction.status = match byte[0] {
            0 => Some(StatusType::SUCCESS),
            1 => Some(StatusType::FAILURE),
            2 => Some(StatusType::PENDING),
            _ => {
                eprintln!("{}", format!("Предупреждение: неизвестный статус транзакции '{}' в поле STATUS. Номер транзакции: {}", byte[0].to_string().yellow().bold(), num_transaction.to_string().yellow().bold()));
                None
            },
        };
    
        let mut len_bytes = [0u8; 4];
        reader.read_exact(&mut len_bytes)?;
        let desc_len = u32::from_be_bytes(len_bytes);
    
        if desc_len > 0 {
            let mut desc_bytes = vec![0u8; desc_len as usize];
            reader.read_exact(&mut desc_bytes)?;
            transaction.description = Some(
                String::from_utf8(desc_bytes)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?
                .trim()
                .trim_matches('"')
                .to_string()
            );
        }
    
        vec_transactions.push(transaction);
    }

    Ok(vec_transactions)

}

//Пишем данные в формат YPBankBin
pub fn write_to<W: std::io::Write>(bin_transactions: Vec<Transaction>,writer: &mut W) -> std::io::Result<()> {
    //Для каждой транзакции в векторе вызываем write_transaction_bin
    for trans in bin_transactions {
        trans.write_transaction_bin(writer)?
    }

    Ok(())
}


//ТЕСТЫ

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufReader, BufWriter};

    #[test]
    fn test_write_read() {
        let mut transaction1 = Transaction::new();
            transaction1.tx_id = Some(1000000000000000);
            transaction1.tx_type = Some(TransactionType::DEPOSIT);
            transaction1.from_user_id = Some(0);
            transaction1.to_user_id = Some(9223372036854775807);
            transaction1.amount = Some(100);
            transaction1.timestamp = Some(1633036860000);
            transaction1.status = Some(StatusType::FAILURE);
            transaction1.description = Some("Record number 1".to_string());

        let mut transaction2 = Transaction::new();
            transaction2.tx_id = Some(1000000000000001);
            transaction2.tx_type = Some(TransactionType::TRANSFER);
            transaction2.from_user_id = Some(9223372036854775807);
            transaction2.to_user_id = Some(9223372036854775807);
            transaction2.amount = Some(200);
            transaction2.timestamp = Some(1633036920000);
            transaction2.status = Some(StatusType::PENDING);
            transaction2.description = Some("Record number 2".to_string());

        let vec_transaction = vec![transaction1.clone(), transaction2.clone()];

        {
            let output_file = File::create("test_bin.bin").unwrap();
            let mut writer = BufWriter::new(output_file);
            write_to(vec_transaction, &mut writer).unwrap();
            writer.flush().unwrap(); // Явно сбрасываем буфер
        };

        let f = File::open("test_bin.bin").unwrap();
        let mut reader = BufReader::new(f);

        let vec_transaction_result = from_read(&mut reader).unwrap();

        assert_eq!(vec_transaction_result[0], transaction1);
        assert_eq!(vec_transaction_result[1], transaction2);

        let _ = std::fs::remove_file("test_bin.bin");
    }
}
