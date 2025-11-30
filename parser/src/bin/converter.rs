use parser::{FileType, from::FromFile, to::ToFile};

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    use std::{
        fs::File,
        io::{BufReader, BufWriter, Read},
    };
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
    fn test_convert_txt_to_csv() {
        let mut buf_r: BufReader<File> = BufReader::new(File::open(PATH_TXT).unwrap());
        let res = FromFile::operations(&mut buf_r, FileType::TXT);
        let operations = res.unwrap();

        let mut buf_w = BufWriter::new(Vec::new());
        let _ = ToFile::operations(&mut buf_w, &operations, FileType::CSV);

        let bytes = buf_w.into_inner().unwrap();
        let csv = String::from_utf8(bytes).unwrap();

        let mut answer = String::new();
        File::open(PATH_CSV)
            .unwrap()
            .read_to_string(&mut answer)
            .unwrap();

        // для быстрой проверки, если файлы не совпадают
        for (i, line) in csv.split('\n').enumerate() {
            assert_eq!(line, answer.split('\n').nth(i).unwrap());
        }
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

    #[test]
    fn test_convert_bin_to_csv() {
        let mut buf_r: BufReader<File> = BufReader::new(File::open(PATH_BIN).unwrap());
        let res = FromFile::operations(&mut buf_r, FileType::BIN);
        let operations = res.unwrap();

        let mut buf_w = BufWriter::new(Vec::new());
        let _ = ToFile::operations(&mut buf_w, &operations, FileType::CSV);

        let bytes = buf_w.into_inner().unwrap();
        let csv = String::from_utf8(bytes).unwrap();

        let mut answer = String::new();
        File::open(PATH_CSV)
            .unwrap()
            .read_to_string(&mut answer)
            .unwrap();

        assert_eq!(csv, answer);
    }
}
