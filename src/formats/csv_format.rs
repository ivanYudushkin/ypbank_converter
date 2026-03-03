use std::io::{BufReader,BufRead};
use crate::transaction::{Transaction, TransactionType, StatusType};
use crate::error::handle_read_error;
use colored::*;
use crate::helpers::parse_u64;

impl Transaction {

    //Метод структцры Transaction для записи транзакции в любой writer поддерживающий Write в формате YPBankCsv
    fn write_transaction_csv<W: std::io::Write>(&mut self, writer: &mut W) ->  std::io::Result<()> {

        if let Some(tx_id) = &self.tx_id {
            write!(writer, "{},", tx_id)?
        };
        if let Some(tx_type) = &self.tx_type {
            write!(writer, "{:?},", tx_type)?
        };
        if let Some(from_user_id) = &self.from_user_id {
            write!(writer, "{},", from_user_id)?
        };
        if let Some(to_user_id) = &self.to_user_id {
            write!(writer, "{},", to_user_id)?
        };
        if let Some(amount) = &self.amount {
            write!(writer, "{},", amount)?
        };
        if let Some(timestamp) = &self.timestamp {
            write!(writer, "{},", timestamp)?
        };
        if let Some(status) = &self.status {
            write!(writer, "{:?},", status)?
        };
        if let Some(description) = &self.description {
            write!(writer, "{:?}", description)?
        };

        writeln!(writer)?;

        Ok(())

    }
}

///Чтение источника соответствующего формату YPBankCsv и приведение его к типу `Vec<Transaction>`
/// 
/// # Аргументы:
/// 
/// * `reader` - читатель реализующий трейт `std::io::Read`
/// 
/// # Возвращает
/// * `Ok(Vec<Transaction>)` - вектор транзакций, прочитанных из источника
/// * `Err` - при ошибке чтения или парсинга
/// 
/// # Ошибки
/// 
/// Функция может вернуть ошибку в следующих случаях:
///
/// - Ошибка чтения данных из источника
/// - Несоответствие формата файла 
/// - Ошибка парсинга бинарных данных
pub fn from_read<R: std::io::Read>(reader: &mut R) -> Result<Vec<Transaction>, std::io::Error> {
    //Инициализация ридера
    let buf_reader = BufReader::new(reader);
    //Инициализация вектора транзакций
    let mut vec_transactions: Vec<Transaction> = Vec::new();
    //Счетсик транзакций для вывода предупреждений
    let mut num_transaction = 0;

    //Идем по каждой строчке источника
    for line in buf_reader.lines().skip(1) {

        //Пытаемся прочитать строку, если не получается, то выводим ошибку о некорректном формате источника
        let line = line
            .map_err(|e| handle_read_error("csv", e))?;

        //Если строка пустая - переходим к следующей строке
        if line.trim().is_empty() {
            continue;
        }
        num_transaction += 1;
        let mut transaction = Transaction::new();
        //Парсим по разделителю
        let parts:Vec<&str> = line.split(",").collect();

        //Пропускаю некорректные строки и вывожу предупреждение
        if parts.len() < 8 {
            eprintln!("{}", format!("Предупреждение: некорректная запись транзакции, длина строки: {}. Номер транзакции: {}", parts.len().to_string().yellow().bold(), num_transaction.to_string().yellow().bold()).yellow());
            continue;
        }
        //Пытаемся вытащить инфо о транзакции
        transaction.tx_id = parse_u64(parts[0]);
        transaction.tx_type = match parts[1].to_lowercase().trim() {
            "deposit" => Some(TransactionType::DEPOSIT),
            "transfer" => Some(TransactionType::TRANSFER),
            "withdrawal" => Some(TransactionType::WITHDRAWAL),
            _ => {
                eprintln!("{}", format!("Предупреждение: неизвестный тип транзакции '{}' в поле TX_TYPE. Номер транзакции: {}", parts[1].to_lowercase().trim().yellow().bold(), num_transaction.to_string().yellow().bold()).yellow());
                None
            }
        };
        transaction.from_user_id = parse_u64(parts[2]);
        transaction.to_user_id = parse_u64(parts[3]);
        transaction.amount = parse_u64(parts[4]);
        transaction.timestamp = parse_u64(parts[5]);
        transaction.status = match parts[6].to_lowercase().trim() {
            "success" => Some(StatusType::SUCCESS),
            "failure" => Some(StatusType::FAILURE),
            "pending" => Some(StatusType::PENDING),
            _ => {
                    eprintln!("{}", format!("Предупреждение: неизвестный статус транзакции '{}' в поле STATUS. Номер транзакции: {}", parts[6].to_lowercase().trim().yellow().bold(), num_transaction.to_string().yellow().bold()).yellow());
                    None
            }
        };
        transaction.description = Some(parts[7].trim().trim_matches('"').to_string());
        //Добавляем транзакцию к вектору
        vec_transactions.push(transaction);
    
    }

    Ok(vec_transactions)

}

///Пишет данные в формат YPBankCsv
/// 
/// # Аргументы:
/// 
/// * `bin_transactions` - вектор транзакций для записи
/// * `writer` - писатель реализующий трейт `std::io::Write``
/// 
/// # Возвращает
/// * `Ok(())` - при успешной записи всех транзакций
/// * `Err` - при ошибке записи данных
/// 
/// # Ошибки
/// 
/// Функция может вернуть ошибку в следующих случаях:
///
/// - Ошибка записи данных в целевой источник
pub fn write_to<W: std::io::Write>(csv_transactions: Vec<Transaction>,mut writer: &mut W) -> std::io::Result<()> {
    //Шапка CSV
    writeln!(writer, "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION")?;
    //Пишем каждую транзакцию
    for mut trans in csv_transactions {
        //Проверка, что у транзакции есть все атрибуты
        if trans.is_complete() {
            trans.write_transaction_csv(&mut writer)?
        }
        else {
            eprintln!("{}", "Предупреждение: Транзакция не имеет всех атрибутов. Запись пропущена".yellow());
            continue;
        }
    }

    Ok(())
}


//ТЕСТЫ

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_read() {
        let input = 
    "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
    1000000000000000,DEPOSIT,0,9223372036854775807,100,1633036860000,FAILURE,\"Record number 1\"
    1000000000000001,TRANSFER,9223372036854775807,9223372036854775807,200,1633036920000,PENDING,\"Record number 2\"



    1000000000000002,WITHDRAWAL,599094029349995112,0,300,1633036980000,TEST,\"Record number 3\"";

        let mut reader = std::io::Cursor::new(input);
        let vec_transaction = from_read(&mut reader).unwrap();

        assert_eq!(vec_transaction.len(), 3);

        assert_eq!(vec_transaction[0].tx_id, Some(1000000000000000));
        assert_eq!(vec_transaction[0].tx_type, Some(TransactionType::DEPOSIT));
        assert_eq!(vec_transaction[0].from_user_id, Some(0));
        assert_eq!(vec_transaction[0].to_user_id, Some(9223372036854775807));
        assert_eq!(vec_transaction[0].amount, Some(100));
        assert_eq!(vec_transaction[0].timestamp, Some(1633036860000));
        assert_eq!(vec_transaction[0].status, Some(StatusType::FAILURE));
        assert_eq!(vec_transaction[0].description, Some("Record number 1".to_string()));

        assert_eq!(vec_transaction[1].tx_id, Some(1000000000000001));
        assert_eq!(vec_transaction[1].tx_type, Some(TransactionType::TRANSFER));
        assert_eq!(vec_transaction[1].from_user_id, Some(9223372036854775807));
        assert_eq!(vec_transaction[1].to_user_id, Some(9223372036854775807));
        assert_eq!(vec_transaction[1].amount, Some(200));
        assert_eq!(vec_transaction[1].timestamp, Some(1633036920000));
        assert_eq!(vec_transaction[1].status, Some(StatusType::PENDING));
        assert_eq!(vec_transaction[1].description, Some("Record number 2".to_string()));

        assert_eq!(vec_transaction[2].tx_id, Some(1000000000000002));
        assert_eq!(vec_transaction[2].tx_type, Some(TransactionType::WITHDRAWAL));
        assert_eq!(vec_transaction[2].from_user_id, Some(599094029349995112));
        assert_eq!(vec_transaction[2].to_user_id, Some(0));
        assert_eq!(vec_transaction[2].amount, Some(300));
        assert_eq!(vec_transaction[2].timestamp, Some(1633036980000));
        assert_eq!(vec_transaction[2].status, None);
        assert_eq!(vec_transaction[2].description, Some("Record number 3".to_string()));

    }
    #[test]
    fn test_write_to() {
        let mut transaction1 = Transaction::new();
        transaction1.tx_id = Some(100);
        transaction1.tx_type = Some(TransactionType::DEPOSIT);
        transaction1.from_user_id = Some(1);
        transaction1.to_user_id = Some(2);
        transaction1.amount = Some(1000);
        transaction1.timestamp = Some(1633036860000);
        transaction1.status = Some(StatusType::SUCCESS);
        transaction1.description = Some("Test transaction 1".to_string());

        let mut transaction2 = Transaction::new();
        transaction2.tx_id = Some(200);
        transaction2.tx_type = Some(TransactionType::WITHDRAWAL);
        transaction2.from_user_id = Some(10);
        transaction2.to_user_id = Some(0);
        transaction2.amount = Some(200);
        transaction2.timestamp = Some(163303698012312);
        transaction2.status = Some(StatusType::FAILURE);
        transaction2.description = Some("Test transaction 2".to_string());

        let mut buffer = Vec::new();
        write_to(vec![transaction1, transaction2], &mut buffer).unwrap();

        let output = String::from_utf8(buffer).unwrap();
        
        // Проверяем, что все поля записаны
        assert!(output.contains("100"));
        assert!(output.contains("DEPOSIT"));
        assert!(output.contains("1"));
        assert!(output.contains("2"));
        assert!(output.contains("1000"));
        assert!(output.contains("1633036860000"));
        assert!(output.contains("SUCCESS"));
        assert!(output.contains("\"Test transaction 1\""));

        assert!(output.contains("200"));
        assert!(output.contains("WITHDRAWAL"));
        assert!(output.contains("10"));
        assert!(output.contains("0"));
        assert!(output.contains("200"));
        assert!(output.contains("163303698012312"));
        assert!(output.contains("FAILURE"));
        assert!(output.contains("\"Test transaction 2\""));
    }

    #[test]
    fn test_from_read_with_bad_record() {
        let input = 
    "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
    1000000000000000,DEPOSIT,0,9223372036854775807,100,1633036860000,FAILURE,\"Record number 1\"
    1000000000000001,TRANSFER,9223372036854775807,200,1633036920000,PENDING,\"Record number 2\"



    1000000000000002,WITHDRAWAL,599094029349995112,0,300,1633036980000,TEST,\"Record number 3\"";

        let mut reader = std::io::Cursor::new(input);
        let vec_transaction = from_read(&mut reader).unwrap();

        assert_eq!(vec_transaction.len(), 2);

    }

    #[test]
    fn test_write_to_bad_record() {
        let mut transaction1 = Transaction::new();
        transaction1.tx_id = Some(100);
        transaction1.tx_type = Some(TransactionType::DEPOSIT);
        transaction1.from_user_id = None;
        transaction1.to_user_id = Some(2);
        transaction1.amount = Some(1000);
        transaction1.timestamp = Some(1633036860000);
        transaction1.status = Some(StatusType::SUCCESS);
        transaction1.description = Some("Test transaction 1".to_string());

        let mut buffer = Vec::new();
        write_to(vec![transaction1], &mut buffer).unwrap();

        let output = String::from_utf8(buffer).unwrap();

        let mut reader = std::io::Cursor::new(output);
        let vec_result = from_read(&mut reader).unwrap();

        assert!(vec_result.is_empty());
    }
}
