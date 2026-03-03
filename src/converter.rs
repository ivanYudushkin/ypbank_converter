use crate::error::unsupported_file_type;
use crate::formats::{csv_format, txt_format, bin_format};

use std::io;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

///Функция читает данные из файла в `Vec<Transaction>`, после чего записывает каждую транзакцию в файл с новым форматом
/// 
/// # Поддерживаемые типы:
/// 
/// * csv
/// * text
/// * bin
/// 
/// # Аргументы:
/// 
/// * `input_file` - путь к файлу
/// * `input_format` - формат исходного файла
/// * `output_format` - формат целевого файла
/// 
/// # Возвращает
/// * `Ok(())` - после конвертации
/// * `Err` - при ошибке смены формата
/// 
/// # Ошибки
/// 
/// Функция может вернуть ошибку в следующих случаях:
///
/// - Исходный файл не найден
/// - Неподдерживаемый формат входного или выходного файла
/// - Ошибка чтения или записи файла
/// - Несоответствие формата файла указанному типу
pub fn converter(input_file: &str, input_format: &str, output_format: &str)  -> io::Result<()> {
    //Ридер для источника
    let f = File::open(input_file)?;
    let mut reader = BufReader::new(f);

    //Проверка исходного формата. Если формат не поддерживается - выводит ошибку
    let read_transactions = match input_format {
        "csv" => csv_format::from_read(&mut reader)?,
        "text" => txt_format::from_read(&mut reader)?,
        "bin" => bin_format::from_read(&mut reader)?,
        _ => return Err(unsupported_file_type(input_file, input_format))
    };
    
    //Проверка целевого формата. Если формат не поддерживается - выводит ошибку
    let output_path = Path::new(input_file)
    .with_extension(match output_format {
        "csv" => "csv",
        "text" => "txt",
        "bin" => "bin",
        _ => return Err(unsupported_file_type(input_file, output_format))
    });

    //Создание целевого врайтера
    let output_file = File::create(&output_path)?;
    let mut writer = BufWriter::new(output_file);

    //Запись в целевой формат
    match output_format {
        "csv" => csv_format::write_to(read_transactions,&mut writer)?,
        "text" => txt_format::write_to(read_transactions,&mut writer)?,
        "bin" => bin_format::write_to(read_transactions,&mut writer)?,
        _ => return Err(unsupported_file_type(input_file, output_format))
    };

    Ok(())
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
    #[allow(unused_imports)]
    use crate::comparer;
    #[test]
    fn test_converter() {
        let mut transaction1 = Transaction::new();
        transaction1.tx_id = Some(100);
        transaction1.tx_type = Some(TransactionType::DEPOSIT);
        transaction1.from_user_id = Some(1);
        transaction1.to_user_id = Some(2);
        transaction1.amount = Some(1000);
        transaction1.timestamp = Some(1633036860000);
        transaction1.status = Some(StatusType::SUCCESS);
        transaction1.description = Some("Test transaction 1".to_string());

        {
            let file1 = File::create("test_converter.txt").unwrap();
            let mut writer1 = BufWriter::new(file1);
            txt_format::write_to(vec![transaction1], &mut writer1).unwrap();
            writer1.flush().unwrap();
        }

        converter("test_converter.txt", "text", "csv").unwrap();
        let result = comparer("test_converter.txt", "text", "test_converter.csv", "csv");
        assert!(result.is_ok()); 

        converter("test_converter.csv", "csv", "bin").unwrap();
        let result = comparer("test_converter.csv", "csv", "test_converter.bin", "bin");
        assert!(result.is_ok()); 

        converter("test_converter.bin", "bin", "text").unwrap();
        let result = comparer("test_converter.bin", "bin", "test_converter.txt", "text");
        assert!(result.is_ok()); 

        let _ = std::fs::remove_file("test_converter.txt");
        let _ = std::fs::remove_file("test_converter.csv");
        let _ = std::fs::remove_file("test_converter.bin");
    }
}