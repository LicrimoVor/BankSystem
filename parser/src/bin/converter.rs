use clap::Parser;
use parser::{FileType, from::FromFile, to::ToFile};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Файл входа
    #[arg(short, long)]
    input: String,

    /// Тип входного файла
    #[arg(long)]
    input_type: Option<String>,

    /// Файл для конвертации
    #[arg(short, long)]
    output: String,

    /// Тип выходного файла
    #[arg(long)]
    output_type: Option<String>,
}

fn main() {
    tracing_subscriber::fmt::init();

    let Cli {
        input,
        input_type,
        output,
        output_type,
    } = Cli::parse();

    let input_type_default = input.split('.').last().unwrap().to_string();
    let output_type_default = output.split('.').last().unwrap().to_string();
    let input_type = input_type.unwrap_or(input_type_default.clone());
    let output_type = output_type.unwrap_or(output_type_default.clone());

    if input_type != input_type_default || output_type != output_type_default {
        println!("Типы файлов не соответствуют названию входного и выходного файлов");
        println!("Вы точно уверены? Y/N");
        let mut a = String::new();
        std::io::stdin().read_line(&mut a).unwrap();
        if a == "N" || a == "n" {
            return;
        }
    }

    let input_type = match input_type.as_str() {
        "txt" => FileType::TXT,
        "csv" => FileType::CSV,
        "bin" => FileType::BIN,
        _ => panic!("Unknown input type"),
    };

    let output_type = match output_type.as_str() {
        "txt" => FileType::TXT,
        "csv" => FileType::CSV,
        "bin" => FileType::BIN,
        _ => panic!("Unknown output type"),
    };

    let mut buf_r: BufReader<File> = BufReader::new(File::open(input).unwrap());
    let operations = FromFile::operations(&mut buf_r, input_type).unwrap();

    let mut buf_w = BufWriter::new(File::create(output).unwrap());
    ToFile::operations(&mut buf_w, &operations, output_type).unwrap();

    println!("Выполено успешно");
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Read;

    const PATH_TXT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/test.txt");
    const PATH_CSV: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/test.csv");
    const PATH_BIN: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/test.bin");

    #[test]
    fn test_convert_txt_to_bin() {
        let mut buf_r: BufReader<File> = BufReader::new(File::open(PATH_TXT).unwrap());
        let res = FromFile::operations(&mut buf_r, FileType::TXT);
        let operations = res.unwrap();

        let mut buf_w = BufWriter::new(Vec::new());
        let _ = ToFile::operations(&mut buf_w, &operations, FileType::BIN);

        let bytes = buf_w.into_inner().unwrap();

        let mut answer = Vec::new();
        File::open(PATH_BIN)
            .unwrap()
            .read_to_end(&mut answer)
            .unwrap();
        assert_eq!(bytes, answer);
    }

    #[test]
    fn test_convert_csv_to_bin() {
        let mut buf_r: BufReader<File> = BufReader::new(File::open(PATH_CSV).unwrap());
        let res = FromFile::operations(&mut buf_r, FileType::CSV);
        let operations = res.unwrap();

        let mut buf_w = BufWriter::new(Vec::new());
        let _ = ToFile::operations(&mut buf_w, &operations, FileType::BIN);

        let bytes = buf_w.into_inner().unwrap();

        let mut answer = Vec::new();
        File::open(PATH_BIN)
            .unwrap()
            .read_to_end(&mut answer)
            .unwrap();

        assert_eq!(bytes, answer);
    }

    //     #[test]
    //     fn test_convert_bin_to_csv() {
    //         let mut buf_r: BufReader<File> = BufReader::new(File::open(PATH_BIN).unwrap());
    //         let res = FromFile::operations(&mut buf_r, FileType::BIN);
    //         let operations = res.unwrap();

    //         let mut buf_w = BufWriter::new(Vec::new());
    //         let _ = ToFile::operations(&mut buf_w, &operations, FileType::CSV);

    //         let bytes = buf_w.into_inner().unwrap();
    //         let csv = String::from_utf8(bytes).unwrap();

    //         let mut answer = String::new();
    //         File::open(PATH_CSV)
    //             .unwrap()
    //             .read_to_string(&mut answer)
    //             .unwrap();

    //         for (i, line) in csv.split('\n').enumerate() {
    //             assert_eq!(line, answer.split('\n').nth(i).unwrap());
    //         }

    //         // assert_eq!(csv, answer);
    //     }
    //     #[test]
    //     fn test_convert_txt_to_csv() {
    //         let mut buf_r: BufReader<File> = BufReader::new(File::open(PATH_TXT).unwrap());
    //         let res = FromFile::operations(&mut buf_r, FileType::TXT);
    //         let operations = res.unwrap();

    //         let mut buf_w = BufWriter::new(Vec::new());
    //         let _ = ToFile::operations(&mut buf_w, &operations, FileType::CSV);

    //         let bytes = buf_w.into_inner().unwrap();
    //         let csv = String::from_utf8(bytes).unwrap();

    //         let mut answer = String::new();
    //         File::open(PATH_CSV)
    //             .unwrap()
    //             .read_to_string(&mut answer)
    //             .unwrap();

    //         // для быстрой проверки, если файлы не совпадают
    //         for (i, line) in csv.split('\n').enumerate() {
    //             assert_eq!(line, answer.split('\n').nth(i).unwrap());
    //         }
    //     }
}
