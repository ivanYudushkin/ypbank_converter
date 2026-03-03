use ypbank_converter::converter;
use clap::Parser;
use colored::*;

#[derive(Parser, Debug)]
#[command(name = "comparer")]
#[command(about = "Сравнивает два файла транзакций на идентичность", long_about = None)]
struct Args {
    input_file: String,

    #[arg(value_parser = ["csv", "text", "bin"])]
    input_format: String,

    #[arg(value_parser = ["csv", "text", "bin"])]
    output_format: String
}

fn main() {

    let args = Args::parse();

    match converter(&args.input_file, &args.input_format, &args.output_format) {
        Ok(_) => println!("{}", "Конвертация прошла успешно".green()),
        //Чтобы не было break в случае ошибки
        Err(e) => eprintln!("{}", e)
    }

}