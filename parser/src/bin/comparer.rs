use anyhow::Result;
use clap::Parser;
use parser::{from::FromFile, types::FileType};
use std::{fs::File, io::BufReader};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Файл 1
    #[arg(long)]
    file1: String,

    /// Тип файла 1
    #[arg(long)]
    file1_type: Option<FileType>,

    /// Файл 2
    #[arg(long)]
    file2: String,

    /// Тип файла 2
    #[arg(long)]
    file2_type: Option<FileType>,
}

fn main() -> Result<()> {
    let Cli {
        file1: f1,
        file1_type: f1_tp,
        file2: f2,
        file2_type: f2_tp,
    } = Cli::parse();

    let f1_tp_default = FileType::try_from(f1.split('.').last().unwrap().to_string()).ok();
    let f2_tp_default = FileType::try_from(f2.split('.').last().unwrap().to_string()).ok();

    if f1_tp.is_none() && f1_tp_default.is_none() {
        println!("Типы входных файлов не указаны");
        return;
    } else if f2_tp.is_none() || f2_tp_default.is_none() {
        println!("Тип выходного файла не указан");
        return;
    }

    if file1_type != file1_type_default || file2_type != file2_type_default {
        println!("Типы не соответствуют названию первого и второго файлов");
        println!("Вы точно уверены? Y/N");
        let mut a = String::new();
        std::io::stdin().read_line(&mut a).unwrap();
        let a = a.trim().to_lowercase();
        if a == "n\n" || a == "n" {
            return;
        }
    }

    let mut buf_r: BufReader<File> = BufReader::new(File::open(file1).unwrap());
    let operations1 = FromFile::operations(&mut buf_r, file1_type).unwrap();

    let mut buf_r: BufReader<File> = BufReader::new(File::open(file2).unwrap());
    let operations2 = FromFile::operations(&mut buf_r, file2_type).unwrap();

    if operations1 != operations2 {
        println!("Файлы не равны");
    }

    for (op1, op2) in operations1.iter().zip(operations2.iter()) {
        if op1 != op2 {
            println!("{:?} != {:?}", op1, op2);
        }
    }
    println!("Сравнение окончено");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const PATH_TXT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/test.txt");
    const PATH_CSV: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/test.csv");
    const PATH_BIN: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/test.bin");

    #[test]
    fn text_compare_bin_txt_csv() {
        let mut buf_r: BufReader<File> = BufReader::new(File::open(PATH_CSV).unwrap());
        let res_csv = FromFile::operations(&mut buf_r, FileType::CSV).unwrap();

        let mut buf_r: BufReader<File> = BufReader::new(File::open(PATH_BIN).unwrap());
        let res_bin = FromFile::operations(&mut buf_r, FileType::BIN).unwrap();

        let mut buf_r: BufReader<File> = BufReader::new(File::open(PATH_TXT).unwrap());
        let res_txt = FromFile::operations(&mut buf_r, FileType::TXT).unwrap();

        for i in 0..res_csv.len() {
            assert_eq!(res_csv[i], res_txt[i]);
            assert_eq!(res_csv[i], res_bin[i]);
        }
    }
}
