# Parser

Этот крейт предоставляет функциональность для парсинга и преобразования банковских записей различного формата. Он включает следующие основные модули:

- `errors`: Определяет пользовательские типы ошибок для крейта.
- `from`: Содержит модули для парсинга записей из различных входных форматов.
- `to`: Содержит модули для преобразования записей в различные выходные форматы.
- `types`: Определяет типы, используемые в парсере.

## Использование

Добавьте следующую информацию в ваш `Cargo.toml`, чтобы включить `parser` в ваш проект:

```toml
[dependencies]
parser = "0.7.0"
```

## CLI инструменты
Крейт parser также включает CLI инструменты для упрощения работы с парсером. Вот некоторые из них:

**parser-converter**: CLI инструмент для преобразования записей из одного формата в другой. Для использования:
```bash
parser --bin converter -- -i <input_file> -o <output_file> -f <input_format> -t <output_format>
```
Где:
* ```<input_file>``` - путь к входному файлу.
* ```<output_file>``` - путь к выходному файлу.
* ```<input_format>``` - формат входных данных (csv, txt, bin).
* ```<output_format>``` - формат выходных данных (csv, txt, bin).

---

**parser-comparer**: CLI инструмент для сравнения двух файлов на равенство. Для использования:
```bash
parser --bin comparer -- --file1 <file1> --file2 <file2>
```
Где:
* ```<file1>``` - путь к первому файлу.
* ```<file2>``` - путь ко второму файлу.



#### Для тестов
```bash
cargo run -p parser --bin converter -- -i ./parser/data/test.csv -o test.bin
```
```bash
cargo run -p parser --bin converter -- -i ./parser/data/test.txt -o test.csv --output-type bin
```
```bash
cargo run -p parser --bin converter -- -i ./parser/data/test.bin --input-type txt -o test.txt
```
```bash
cargo run -p parser --bin comparer -- --file1 ./parser/data/test.txt --file2 ./parser/data/test.bin
```
