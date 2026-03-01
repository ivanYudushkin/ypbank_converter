use std::io::{BufReader,BufRead};
use crate::transaction::{Transaction, TransactionType, StatusType};
use crate::error::handle_read_error;
use crate::helpers::parse_u64;
use colored::*;


impl Transaction {

    //Метод структцры Transaction для записи транзакции в любой writer поддерживающий Write в формате YPBankText
    fn write_transaction_txt<W: std::io::Write>(&mut self, writer: &mut W) ->  std::io::Result<()> {

        if let Some(tx_id) = &self.tx_id {
            writeln!(writer, "TX_ID: {}", tx_id)?
        };
        if let Some(tx_type) = &self.tx_type {
            writeln!(writer, "TX_TYPE: {:?}", tx_type)?
        };
        if let Some(from_user_id) = &self.from_user_id {
            writeln!(writer, "FROM_USER_ID: {}", from_user_id)?
        };
        if let Some(to_user_id) = &self.to_user_id {
            writeln!(writer, "TO_USER_ID: {}", to_user_id)?
        };
        if let Some(amount) = &self.amount {
            writeln!(writer, "AMOUNT: {}", amount)?
        };
        if let Some(timestamp) = &self.timestamp {
            writeln!(writer, "TIMESTAMP: {}", timestamp)?
        };
        if let Some(status) = &self.status {
            writeln!(writer, "STATUS: {:?}", status)?
        };
        if let Some(description) = &self.description {
            writeln!(writer, "DESCRIPTION: {:?}", description)?
        };

        writeln!(writer)?;

        Ok(())

    }
    
}

//Чтение источника соответствующего формату YPBankText и приведение его к типу Vec<Transaction>
pub fn from_read<R: std::io::Read>(reader: &mut R) -> Result<Vec<Transaction>, std::io::Error> {
    //Инициализация ридера
    let buf_reader = BufReader::new(reader);
    //Инициализация транзакции
    let mut transaction = Transaction::new();
    //Инициализация Vec<Transaction> для накопления транзакий из источника
    let mut vec_transactions: Vec<Transaction> = Vec::new();

    //Счетчик транзакций для вывода предупреждений
    let mut num_transaction = 1;

    //Идем по каждой строке источника
    for line in buf_reader.lines() {
        //Пытаемся прочитать строку, если не получается, то выводим ошибку о некорректном формате источника
        let line = line
            .map_err(|e| handle_read_error("txt", e))?;

        //Пустая строка - разделитеель между транзакциями. Если встречаем пустую строку и у нас есть записанная транзакция (transaction.tx_id.is_some()), то добавляем ее в вектор
        if line.trim().is_empty() {
            if transaction.tx_id.is_some() {
                num_transaction += 1;
                vec_transactions.push(transaction);
                transaction = Transaction::new();
            }
            continue;
        }

        //Парсим строку по разделителю и пытаемся достать инфо о транзакции
        if let Some((key, value)) = line.split_once(":") {
            match key.to_lowercase().as_str().trim() {
                "tx_id" => {
                    transaction.tx_id = parse_u64(value)
                },
                "tx_type" => {
                    transaction.tx_type = match value.to_lowercase().trim() {
                        "deposit" => Some(TransactionType::DEPOSIT),
                        "transfer" => Some(TransactionType::TRANSFER),
                        "withdrawal" => Some(TransactionType::WITHDRAWAL),
                        //Выводим предупреждении о несуществующем типе
                        _ => {
                            eprintln!("{}", format!("Предупреждение: неизвестный тип транзакции '{}' в поле TX_TYPE. Номер транзакции: {}", value.to_lowercase().trim().yellow().bold(), num_transaction.to_string().yellow().bold()).yellow());
                            None
                        }
                    }
                },
                "from_user_id" => {
                    transaction.from_user_id = parse_u64(value)
                },
                "to_user_id" => {
                    transaction.to_user_id = parse_u64(value)
                },
                "amount" => {
                    transaction.amount = parse_u64(value)
                },
                "timestamp" => {
                    transaction.timestamp = parse_u64(value)
                },
                "status" => {
                    transaction.status = match value.to_lowercase().trim() {
                        "success" => Some(StatusType::SUCCESS),
                        "failure" => Some(StatusType::FAILURE),
                        "pending" => Some(StatusType::PENDING),
                        //Выводим предупреждении о несуществующем статусе
                        _ => {
                            eprintln!("{}", format!("Предупреждение: неизвестный статус транзакции '{}' в поле STATUS. Номер транзакции: {}", value.to_lowercase().trim().yellow().bold(), num_transaction.to_string().yellow().bold()).yellow());
                            None
                        }
                    }
                },
                "description" => {
                    transaction.description = Some(value.trim().trim_matches('"').to_string());
                },
                //Если обнаружился ключ, который не соответствует структуре
                _ => {
                    eprintln!("Предупреждение: Неизвестный атрибут у транзакции {}", num_transaction)
                }
            }
        }
    }

    //Запись последней транзакции перед закрытием ридера
    if transaction.tx_id.is_some() {
        vec_transactions.push(transaction);
    }

    Ok(vec_transactions)

}

//Пишем данные в формат YPBankText
pub fn write_to<W: std::io::Write>(txt_transactions: Vec<Transaction>,mut writer: &mut W) -> std::io::Result<()> {
    //Для каждой транзакции в векторе вызываем write_transaction_txt
    for mut trans in txt_transactions {
        trans.write_transaction_txt(&mut writer)?
    }

    Ok(())
}


//ТЕСТЫ
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_read() {
        let input = "TX_ID: 1
                TX_TYPE: DEPOSIT
                FROM_USER_ID: 0
                TO_USER_ID: 9223372036854775807
                AMOUNT: 100
                TIMESTAMP: 1633036860000
                STATUS: FAILURE
                DESCRIPTION: \"Record number 1\"
                
                TX_ID: 2
                TX_TYPE: WITHDRAWAL
                FROM_USER_ID: 599094029349995112
                TO_USER_ID: 0
                AMOUNT: 300
                TIMESTAMP: 1633036980000
                STATUS: SUCCESS
                DESCRIPTION: \"Record number 3\"








                TX_ID: 10
                TX_TYPE: nonetype
                FROM_USER_ID: dmitry
                TO_USER_ID: alex
                AMOUNT: not
                TIMESTAMP: today
                STATUS: well done
                DESCRIPTION: 1000



                TX_ID: too_bad_transaction
                TX_TYPE: nonetype
                FROM_USER_ID: dmitry
                DESCRIPTION: 1000
    ";

        let mut reader = std::io::Cursor::new(input);
        let vec_transaction = from_read(&mut reader).unwrap();

        assert_eq!(vec_transaction.len(), 3);

        assert_eq!(vec_transaction[0].tx_id, Some(1));
        assert_eq!(vec_transaction[0].tx_type, Some(TransactionType::DEPOSIT));
        assert_eq!(vec_transaction[0].from_user_id, Some(0));
        assert_eq!(vec_transaction[0].to_user_id, Some(9223372036854775807));
        assert_eq!(vec_transaction[0].amount, Some(100));
        assert_eq!(vec_transaction[0].timestamp, Some(1633036860000));
        assert_eq!(vec_transaction[0].status, Some(StatusType::FAILURE));
        assert_eq!(vec_transaction[0].description, Some("Record number 1".to_string()));

        assert_eq!(vec_transaction[1].tx_id, Some(2));
        assert_eq!(vec_transaction[1].tx_type, Some(TransactionType::WITHDRAWAL));
        assert_eq!(vec_transaction[1].from_user_id, Some(599094029349995112));
        assert_eq!(vec_transaction[1].to_user_id, Some(0));
        assert_eq!(vec_transaction[1].amount, Some(300));
        assert_eq!(vec_transaction[1].timestamp, Some(1633036980000));
        assert_eq!(vec_transaction[1].status, Some(StatusType::SUCCESS));
        assert_eq!(vec_transaction[1].description, Some("Record number 3".to_string()));

        assert_eq!(vec_transaction[2].tx_id, Some(10));
        assert_eq!(vec_transaction[2].tx_type, None);
        assert_eq!(vec_transaction[2].from_user_id, None);
        assert_eq!(vec_transaction[2].to_user_id, None);
        assert_eq!(vec_transaction[2].amount, None);
        assert_eq!(vec_transaction[2].timestamp, None);
        assert_eq!(vec_transaction[2].status, None);
        assert_eq!(vec_transaction[2].description, Some(1000.to_string()));

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
        assert!(output.contains("TX_ID: 100"));
        assert!(output.contains("TX_TYPE: DEPOSIT"));
        assert!(output.contains("FROM_USER_ID: 1"));
        assert!(output.contains("TO_USER_ID: 2"));
        assert!(output.contains("AMOUNT: 1000"));
        assert!(output.contains("TIMESTAMP: 1633036860000"));
        assert!(output.contains("STATUS: SUCCESS"));
        assert!(output.contains("DESCRIPTION: \"Test transaction 1\""));

        assert!(output.contains("TX_ID: 200"));
        assert!(output.contains("TX_TYPE: WITHDRAWAL"));
        assert!(output.contains("FROM_USER_ID: 10"));
        assert!(output.contains("TO_USER_ID: 0"));
        assert!(output.contains("AMOUNT: 200"));
        assert!(output.contains("TIMESTAMP: 163303698012312"));
        assert!(output.contains("STATUS: FAILURE"));
        assert!(output.contains("DESCRIPTION: \"Test transaction 2\""));
    }
}
