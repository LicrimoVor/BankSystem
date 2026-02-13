use clap::CommandFactory;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "blog-cli")]
#[command(about = "CLI client for Blog API")]
pub struct Cli {
    #[arg(long, default_value = "http")]
    pub transport: Transport,

    #[arg(long, default_value = "http://127.0.0.1:8001")]
    pub http_addr: String,

    #[arg(long, default_value = "http://127.0.0.1:50051")]
    pub grpc_addr: String,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Transport {
    Http,
    Grpc,
}

#[command(multicall = true)]
#[derive(Parser)]
pub enum Command {
    Health,
    Ping,

    Auth {
        #[command(subcommand)]
        cmd: AuthCmd,
    },

    User {
        #[command(subcommand)]
        cmd: UserCmd,
    },

    Post {
        #[command(subcommand)]
        cmd: PostCmd,
    },

    Exit,
}

impl Command {
    pub fn print_help() {
        let mut cmd = Command::command();
        cmd.print_help().unwrap();
        println!();
    }
}

#[derive(Subcommand)]
pub enum AuthCmd {
    Register {
        username: String,
        email: String,
        password: String,
    },
    Login {
        email: String,
        password: String,
    },
    Logout,
}

#[derive(Subcommand)]
pub enum UserCmd {
    Me,
    Update {
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        email: Option<String>,
        #[arg(long)]
        password: Option<String>,
    },
    Delete,
    GetByEmail {
        email: String,
    },
}

#[derive(Subcommand)]
pub enum PostCmd {
    Create {
        title: String,
        content: String,
        #[arg(long)]
        img_base64: Option<String>,
    },
    Update {
        post_id: String,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        content: Option<String>,
        #[arg(long)]
        img_base64: Option<String>,
    },
    Delete {
        post_id: String,
    },
    Get {
        post_id: String,
    },
    My,
    ByAuthor {
        email: String,
    },
}
