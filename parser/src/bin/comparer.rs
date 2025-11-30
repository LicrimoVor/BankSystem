use parser::{FileType, from::FromFile, to::ToFile};
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};
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
            assert_eq!(res_csv[i], res_bin[i]);
            assert_eq!(res_csv[i], res_txt[i]);
        }
    }
}
