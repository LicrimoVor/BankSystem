# Image CLI Processor (FFI Plugins)

В репозитории два процессора:

- Rust
- Python (через `ctypes`)

---

## Что делает приложение

1. Загружает изображение
2. Приводит к формату `Rgba8`
3. Получает `width`, `height`, `Vec<u8>` (RGBA)
4. Загружает динамическую библиотеку
5. Передаёт данные плагину
6. Сохраняет результат

---

## CLI аргументы

- `-i, --input` — входное изображение
- `-o, --output` — выходное изображение
- `--plugin` — имя плагина (без расширения)
- `--params` — путь к файлу параметров
- `--plugin-path` — путь к директории плагинов (по умолчанию `target/debug`)

---

## Сигнатура плагина

```rust
#[no_mangle]
pub extern "C" fn process(
    width: u32,
    height: u32,
    data: *mut u8,
    params: *const c_char,
) -> i32
```

- `data` — RGBA-буфер длиной `width * height * 4`
- обработка происходит в переданном массиве
- память остаётся у процессора

---

## Сборка и запуск (Rust)

Собрать плагин:

```bash
cargo build
```

Запустить процессор:

```bash
cargo run -- \
  -i ./assets/ava.jpg \
  -o ava2.png \
  --plugin plugin_blur \
  --params ./plugin_blur/params.json
```

## Запуск (Python)

Установить все зависимости

```bash
cd py_processor
pip install -r requirements.txt
```

Запустить процессор:

```bash
python main.py
```
