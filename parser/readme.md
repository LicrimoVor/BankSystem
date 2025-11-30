потом напишу


для тестов ;)
cargo run -p parser --bin converter -- -i ./parser/data/test.csv -o test.bin
cargo run -p parser --bin converter -- -i ./parser/data/test.txt -o test.csv --output-type bin
cargo run -p parser --bin converter -- -i ./parser/data/test.bin --input-type txt -o test.txt


cargo run -p parser --bin comparer -- --file1 ./parser/data/test.txt --file2 ./parser/data/test.bin