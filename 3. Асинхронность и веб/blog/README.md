# Blog - многопротокольная асинхронная система управления блогом

Полнофункциональная система управления блогом, написанная на Rust с поддержкой нескольких протоколов, баз данных и клиентов.

## 📦 Архитектура проекта

Проект состоит из трёх основных крейтов:

### Backend (`backend/`)
Серверная часть с поддержкой multiple backend'ов и протоколов.

**Особенности:**
- 📡 **Два протокола коммуникации:**
  - **HTTP API** (Axum) - REST API с OpenAPI документацией через Utoipa
  - **gRPC** (Tonic) - высокопроизводительный RPC через Protocol Buffers
  
- 💾 **Две системы хранения данных:**
  - **Memory State** - асинхронное хранилище в памяти (для разработки и тестирования)
  - **PostgreSQL** - релационная база данных (для production)
  
- 🏗️ **Чистая архитектура:**
  - Domain layer - бизнес-логика
  - Application layer - use cases и сервисы
  - Data layer - абстракции для работы с БД (в памяти и PostgreSQL)
  - Infrastructure layer - конфигурация, безопасность, обработка ошибок
  - Presentation layer - HTTP и gRPC handlers

### Client (`client/`)
Многопротокольный клиент для взаимодействия с backend'ом.

**Особенности:**
- 🌐 **Два клиента:**
  - **HTTP Client** - работает с REST API через HTTP
  - **gRPC Client** - работает с gRPC сервисом
  
- ⌨️ **CLI утилита** - командная строка для взаимодействия с API
  - Полный набор команд для управления пользователями, постами и аутентификацией
  - Удобная работа с API без необходимости писать свой код

- 📚 **Примеры использования** - готовые примеры для обоих протоколов

### Frontend (`frontend/`)
Веб-интерфейс приложения, написанный на WebAssembly.

**Особенности:**
- 🎨 **Технология:**
  - Чистый HTML и CSS (хотя очень хотелось React)
  - WebAssembly (WASM) модуль на Rust для логики UI
  - Асинхронный код для работы с API
  
- 🔄 **Интеграция:**
  - Взаимодействие с HTTP API backend'а
  - Полная функциональность блога в браузере

## 🛠️ Стэк технологий

### Backend
- **SeaORM** - ORM и миграции для работы с БД
- **Axum** - асинхронный HTTP framework
- **Tonic** - gRPC framework
- **PostgreSQL** + **tokio** - асинхронная работа с БД
- **jsonwebtoken** - JWT аутентификация
- **Utoipa** - автоматическая OpenAPI документация

### Client
- **Tonic** - gRPC клиент
- **reqwest** - HTTP клиент
- **tokio** - асинхронный runtime
- **clap** - парсинг CLI аргументов

### Frontend
- **wasm-bindgen** - привязки между Rust и JavaScript
- **web-sys** - Web API для работы с DOM
- **tokio** - асинхронный runtime в браузере

## 🔐 Авторизация

Используется **двухтокенная система** для безопасности:

```
┌─────────────┐
│   Клиент    │
└──────┬──────┘
       │ 1. POST /auth/login
       │    → username, password
       ▼
┌──────────────────────────────────────┐
│ Backend проверяет credentials        │
└──────┬───────────────────────────────┘
       │ 2. Возвращает:
       │    - JWT (in response body)
       │    - Refresh token (in http-only cookie)
       ▼
┌─────────────────────────────────────┐
│ Клиент хранит:                      │
│ - JWT в памяти (для каждого запроса)│
│ - Refresh token в куках (автоматом) │
└─────────────────────────────────────┘
       │
       │ 3. Если JWT истёк, отправляет refresh
       │    POST /auth/refresh
       │    (куки с refresh token отправляются автоматом)
       ▼
┌────────────────────────────────────┐
│ Backend выдаёт новый JWT           │
└────────────────────────────────────┘
```

**JWT** используется для:
- Аутентификации HTTP запросов в заголовке `Authorization: Bearer <token>`
- Аутентификации gRPC запросов в метаданных

**Refresh Token** используется для:
- Получения новых JWT токенов без повторного ввода пароля
- Хранится в http-only cookie (защита от XSS)

## 🚀 Быстрый старт

### Backend

```bash
cd backend

# С в памяти хранилищем
cargo run
```

### Client - CLI

```bash
cd client

cargo run
```

### Frontend

```bash
cd frontend


wasm-pack build
```

## 📖 API Документация

### HTTP API
OpenAPI документация доступна по адресу: `http://localhost:8001/api/redoc/`

### gRPC
Proto файлы находятся в `proto/` папке каждого крейта.

## 🗂️ Структура проекта

```
blog/
├── backend/
│   ├── src/
│   │   ├── domain/          # Бизнес-логика (User, Post, Auth)
│   │   ├── application/     # Use cases и сервисы
│   │   ├── data/            # Абстракции для работы с БД
│   │   │   ├── memory/      # В памяти реализация
│   │   │   └── postgres/    # PostgreSQL реализация
│   │   ├── infrastructure/  # Конфиг, безопасность, ошибки
│   │   ├── presentation/    # HTTP и gRPC handlers
│   │   │   ├── http/
│   │   │   └── grpc/
│   │   └── main.rs
│   ├── proto/               # Proto файлы для gRPC
│   └── Cargo.toml
│
├── client/
│   ├── src/
│   │   ├── http/            # HTTP клиент
│   │   ├── grpc/            # gRPC клиент
│   │   ├── cli/             # CLI интерфейс
│   │   └── types/           # Общие типы
│   ├── examples/
│   │   ├── cli.rs           # CLI пример
│   │   └── test.rs          # Тестовый пример
│   ├── proto/               # Proto файлы (синхронизированы с backend)
│   └── Cargo.toml
│
├── frontend/
│   ├── src/                 # Rust/WASM код
│   ├── static/
│   │   └── index.html       # HTML интерфейс
│   └── Cargo.toml
│
└── README.md
```

## 🔄 Основные endpointy

### Аутентификация
- `POST /api/auth/register` - регистрация
- `POST /api/auth/login` - вход
- `POST /api/auth/logout` - выход
- `POST /api/auth/refresh` - обновить JWT

### Пользователи
- `GET /api/user/me` - получить профиль (требует JWT)
- `GET /api/user/{email}` - получить пользователя по email
- `PATCH /api/user/me` - обновить профиль (требует JWT)
- `DELETE /api/user/me` - удалить аккаунт (требует JWT)

### Посты
- `POST /api/post/` - создать пост (требует JWT)
- `GET /api/post/{post_id}` - получить пост
- `GET /api/post/author/{email}` - получить посты автора
- `GET /api/post/me` - получить свои посты (требует JWT)
- `PATCH /api/post/{post_id}` - обновить пост (требует JWT)
- `DELETE /api/post/{post_id}` - удалить пост (требует JWT)

### Утилиты
- `GET /api/health` - проверка здоровья сервера
- `GET /api/ping` - пинг
- `GET /api/media/{filename}` - получить медиафайл

## 🎯 Особенности

✨ **Асинхронность** - весь код написан с использованием async/await
🔐 **Безопасность** - JWT токены, http-only cookies, защита паролей
📦 **Масштабируемость** - абстракции для разных источников данных
🔄 **Протоколы** - HTTP и gRPC для разных сценариев использования
📱 **Кроссплатформенность** - backend работает везде, frontend в браузере, client везде

## 📝 Лицензия

Этот проект создан в рамках учебного курса по асинхронному программированию на Rust и веб-разработке.