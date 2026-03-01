use std::io;
use colored::*;

pub fn handle_read_error(format: &str, error: io::Error) -> io::Error {
    if error.kind() == io::ErrorKind::InvalidData {
        eprintln!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".red());
        eprintln!("{}", "ОШИБКА: Несоответствие формата файла".red());
        eprintln!("{}", format!("Причина: файл не может быть прочитан как {} формат", format.red().bold()).red());
        eprintln!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".red());
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Файл не может быть прочитан как {} формат", format)
        )
    } else {
        error
    }
}

pub fn unsupported_file_type(file_name: &str,file_format: &str) -> io::Error {
    eprintln!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".red());
    eprintln!("{}", "Ошибка: Неподдерживаемый тип файла".red());
    eprintln!("{}", format!("Файл: {}", file_name.red().bold()).red());
    eprintln!("{}", format!("Указанный формат: {}", file_format.red().bold()).red());
    eprintln!("{}", "Доступные форматы: csv, text, bin".red());
    eprintln!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".red());
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("Неподдерживаемый тип файла: {}", file_format))
}