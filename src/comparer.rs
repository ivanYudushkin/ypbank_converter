use crate::error::unsupported_file_type;
use crate::formats::{csv_format, txt_format, bin_format};

use std::io;
use std::fs::File;
use std::io::BufReader;

///Функция читает транзакции из обоих файлов в промежуточное представление `Vec<Transaction>` и сравнивает два вектора на идентичность
/// 
/// # Поддерживаемые типы:
/// 
/// * csv
/// * text
/// * bin
/// 
/// # Аргументы:
/// 
/// * `file1` - путь к первому файлу
/// * `format1` - формат первого файла
/// * `file2` - путь до второго файла
/// * `format2` - формат второго файла
/// 
/// # Возвращает
/// * `Ok(true)` - если файлы идентичны
/// * `Ok(false)` - если файлы разные
/// 
/// # Ошибки
/// 
/// Функция может вернуть ошибку в следующих случаях:
///
/// - Один из файлов не найден
/// - Неподдерживаемый формат файла
/// - Ошибка чтения файла
/// - Несоответствие формата файла указанному типу
pub fn comparer(file1: &str, format1: &str, file2: &str, format2: &str) -> io::Result<bool> {
    //Ридер для 1-го источника
    let f1 = File::open(file1)?;
    let mut reader1 = BufReader::new(f1);
    //Ридер для 2-го источника
    let f2 = File::open(file2)?;
    let mut reader2 = BufReader::new(f2);
    //Vec<Transaction> из 1-го источника в соответствии с форматом
    let read_transactions1 = match format1 {
        "csv" => csv_format::from_read(&mut reader1)?,
        "text" => txt_format::from_read(&mut reader1)?,
        "bin" => bin_format::from_read(&mut reader1)?,
        _ => return Err(unsupported_file_type(file1, format1))
    };
    //Vec<Transaction> из 2-го источника в соответствии с форматом
    let read_transactions2 = match format2 {
        "csv" => csv_format::from_read(&mut reader2)?,
        "text" => txt_format::from_read(&mut reader2)?,
        "bin" => bin_format::from_read(&mut reader2)?,
        _ => return Err(unsupported_file_type(file2, format2))
    };
    
    Ok(read_transactions1 == read_transactions2)


}

//ТЕСТЫ
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use std::fs::File;
    #[allow(unused_imports)]
    use std::io::{BufReader, BufWriter, Write};
    #[allow(unused_imports)]
    use crate::transaction::{Transaction, TransactionType, StatusType};
    #[test]
    fn test_comparer() {
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
        transaction2.to_user_id = Some(20);
        transaction2.amount = Some(2000);
        transaction2.timestamp = Some(1633036920000);
        transaction2.status = Some(StatusType::FAILURE);
        transaction2.description = Some("Test transaction 2".to_string());

        let mut transaction3 = Transaction::new();
        transaction3.tx_id = Some(100);
        transaction3.tx_type = Some(TransactionType::DEPOSIT);
        transaction3.from_user_id = Some(1);
        transaction3.to_user_id = Some(2);
        transaction3.amount = Some(1000);
        transaction3.timestamp = Some(1633036860000);
        transaction3.status = Some(StatusType::SUCCESS);
        transaction3.description = Some("Test transaction 1".to_string());

        {
            let file1 = File::create("test_comparer1.csv").unwrap();
            let mut writer1 = BufWriter::new(file1);
            csv_format::write_to(vec![transaction1], &mut writer1).unwrap();
            writer1.flush().unwrap();
        }

        {
            let file2 = File::create("test_comparer2.txt").unwrap();
            let mut writer2 = BufWriter::new(file2);
            txt_format::write_to(vec![transaction2], &mut writer2).unwrap();
            writer2.flush().unwrap();
        }

        {
            let file3 = File::create("test_comparer3.bin").unwrap();
            let mut writer3 = BufWriter::new(file3);
            bin_format::write_to(vec![transaction3], &mut writer3).unwrap();
            writer3.flush().unwrap();
        }


        let result = comparer("test_comparer1.csv", "csv", "test_comparer2.txt", "text");
        assert!(result.is_ok()); 

        let result = comparer("test_comparer2.txt", "text", "test_comparer3.bin", "bin");
        assert!(result.is_ok());
        
        let result = comparer("test_comparer1.csv", "csv", "test_comparer3.bin", "bin");
        assert!(result.is_ok()); 

        // Очистка
        let _ = std::fs::remove_file("test_comparer1.csv");
        let _ = std::fs::remove_file("test_comparer2.txt");
        let _ = std::fs::remove_file("test_comparer3.bin");
    }
}