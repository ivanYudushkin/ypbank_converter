mod formats; 
mod error;
mod converter;
mod comparer;
mod helpers;
mod transaction;

use std::io::{self, BufRead, Write};
use converter::converter;
use comparer::comparer;

use colored::*;


fn main() -> io::Result<()>  {
    //CLI
    println!("{}", "=== CLI ===".blue());
    println!("{}", "Команды:".blue());
    println!("{}", "ypbank_converter <input_file> <input_format> <output_format>     - конвертация файла в другой тип".blue());
    println!("{}", "ypbank_comparer <file1> <format1> <file2> <format2>              - проверка, что файлы одинаковые".blue());
    println!("{}", "Поддерживаемые форматы:".blue());
    println!("{}", "  -txt".blue());
    println!("{}", "  -csv".blue());
    println!("{}", "  -bin".blue());

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush().unwrap(); 

        let mut input = String::new();
        if stdin.lock().read_line(&mut input).unwrap() == 0 {
            break; 
        }

        let args: Vec<&str> = input.trim()
            .split_whitespace()
            .map(|s| s.trim_matches('"'))
            .collect();

        if args.is_empty() {
            continue;
        }

        //Команда converter
        match args[0] {
            "ypbank_converter" => {
                if args.len() == 4 {
                    match converter(args[1], args[2], args[3]) {
                        Ok(_) => println!("{}", "Конвертация прошла успешно".green()),
                        //Чтобы не было break в случае ошибки
                        Err(e) => eprintln!("{}", e)
                    }
                }
                //Обработка неправильного формата ввода
                else {
                    eprintln!("{}", "Использование:".yellow());
                    eprintln!("{}", "ypbank_converter <input_file> <input_format> <output_format>".yellow());
                }
            },
            //Команда comparer
            "ypbank_comparer" => {
                if args.len() == 5 {
                    match comparer(args[1], args[2], args[3], args[4]) {
                        Ok(_) => print!(""),
                        //Чтобы не было break в случае ошибки
                        Err(e) => eprintln!("{}", e)
                    }
                }
                //Обработка неправильного формата ввода
                else {
                    eprintln!("{}", "Использование:".yellow());
                    eprintln!("{}", "ypbank_comparer <file1> <format1> <file2> <format2>".yellow());
                }
            },
            "exit" => break,
            //Обработка ввода неправильной функции
            _ => {
                eprintln!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".red());
                eprintln!("{}", format!("Функция {} не поддерживается", args[0].red().bold()).red());
                eprintln!("{}", "Поддерживаемые функции:".red());
                eprintln!("{}", "ypbank_converter <input_file> <input_format> <output_format>".red());
                eprintln!("{}", "ypbank_comparer <file1> <format1> <file2> <format2>".red());
                eprintln!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".red());
            }
        }
        
        }

        Ok(())

    }
    // converter("C:/Users/ivany/Downloads/Примеры_файлов2/records_example1.csv", "csv", "bin")?;
    // comparer("C:/Users/ivany/Downloads/Примеры_файлов2/records_example.bin", "dasd", "C:/Users/ivany/Downloads/Примеры_файлов2/records_example1.bin", "bin")?
