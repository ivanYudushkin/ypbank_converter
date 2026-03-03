use ypbank_converter::comparer;
use clap::Parser;
use colored::*;

#[derive(Parser, Debug)]
#[command(name = "comparer")]
#[command(about = "Сравнивает два файла транзакций на идентичность", long_about = None)]
struct Args {
    file1: String,

    #[arg(value_parser = ["csv", "text", "bin"])]
    format1: String,

    file2: String,
    
    #[arg(value_parser = ["csv", "text", "bin"])]
    format2: String,
}

fn main() {

    let args = Args::parse();

    match comparer(&args.file1, &args.format1, &args.file2, &args.format2) {
        Ok(true) => {
            println!("{}", "====================================================".blue());
            println!("{}", "Результат проверки идентичности двух файлов".blue());
            println!("{}", format!("Файл1: {}, Формат: {}", &args.file1.blue().bold(), &args.format1.blue().bold()).blue());
            println!("{}", format!("Файл2: {}, Формат: {}", &args.file2.blue().bold(), &args.format2.blue().bold()).blue());
            println!("{}", "TRUE".green().bold());
            println!("{}", format!("The transaction records in {} and {} are identical.", &args.file1.blue().bold(), &args.file2.blue().bold()).blue());
            println!("{}", "====================================================".blue());
        },
        //Чтобы не было break в случае ошибки
        Ok(false) => {
            println!("{}", "====================================================".blue());
            println!("Результат проверки идентичности двух файлов");
            println!("{}", format!("Файл1: {}, Формат: {}", &args.file1.blue().bold(), &args.format1.blue().bold()).blue());
            println!("{}", format!("Файл2: {}, Формат: {}", &args.file2.blue().bold(), &args.format2.blue().bold()).blue());
            println!("{}", "FALSE".red().bold());
            println!("{}", format!("The transaction records in {} and {} are not identical.", &args.file1.blue().bold(), &args.file2.blue().bold()).blue());
            println!("{}", "====================================================".blue());
        },
        Err(e) => {
            eprintln!("Ошибка: {}", e);
        }
    }

}