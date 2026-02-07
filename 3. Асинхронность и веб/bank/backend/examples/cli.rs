// src/main.rs

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bank", about = "Банковская CLI-утилита", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Показать баланс клиента
    Balance {
        #[arg(short, long)]
        user: String,
    },

    Deposit {
        #[arg(short, long)]
        user: String,
        #[arg(short, long)]
        amount: f64,
    },

    /// Перевести средства
    Transfer {
        #[arg(short, long)]
        from: String,
        #[arg(short, long)]
        to: String,
        #[arg(short, long)]
        amount: f64,
    },
    /// Показать историю транзакций
    History {
        #[arg(short, long)]
        user: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Balance { user } => {
            println!("Баланс пользователя {user}: 1000₽");
        }
        Commands::Transfer { from, to, amount } => {
            println!("Переводим {amount}₽ от {from} к {to}");
        }
        Commands::History { user } => match user {
            Some(u) => println!("История операций пользователя {u}: ..."),
            None => println!("История всех операций: ..."),
        },
        Commands::Deposit { user, amount } => {
            println!("Депозитируем {amount}₽ на счет пользователя {user}");
        }
    }
}
