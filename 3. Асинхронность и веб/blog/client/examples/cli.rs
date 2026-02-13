use anyhow::Result;
use clap::Parser;
use client::{
    GrpcClient, HttpClient,
    cli::{self, Cli, Command, Transport},
    types::{Client, Error},
};
#[cfg(not(feature = "cli"))]
compile_error!("feature 'cli' is not enabled");

pub async fn init_client(
    transport: &str,
    http_addr: &str,
    grpc_addr: &str,
) -> Result<Arc<dyn Client>, Error> {
    match transport {
        "http" => {
            let Ok(client) = HttpClient::new(http_addr).await else {
                return Err(Error::Inner("failed to create http client".into()));
            };
            Ok(Arc::new(client) as Arc<dyn Client>)
        }
        "grpc" => {
            let Ok(client) = GrpcClient::new(grpc_addr).await else {
                return Err(Error::Inner("failed to create grpc client".into()));
            };
            Ok(Arc::new(client) as Arc<dyn Client>)
        }
        _ => Err(Error::Inner("unknown transport".into())),
    }
}

/// Примитивненько но со вкусом)
#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let (type_client, addr) = match &cli.transport {
        Transport::Http => ("http", &cli.http_addr),
        Transport::Grpc => ("grpc", &cli.grpc_addr),
    };
    println!("Connect {}-client to {}", type_client, addr);

    let client = init_client(
        &format!("{:?}", cli.transport).to_lowercase(),
        &cli.http_addr,
        &cli.grpc_addr,
    )
    .await?;
    let mut general = client.general();
    println!("Success connect");

    loop {
        let line = readline()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        };
        let Ok(args) = shlex::split(line).ok_or("error: Invalid quoting") else {
            println!("Error shlex split");
            continue;
        };

        let command = match Command::try_parse_from(args) {
            Ok(command) => command,
            Err(e) => {
                println!("{}", e.to_string());
                continue;
            }
        };

        match command {
            Command::Exit => {
                break;
            }
            Command::Health => {
                println!("{}", general.health().await?);
            }
            Command::Ping => {
                println!("{}", general.ping().await?);
            }
            Command::Auth { cmd } => cli::auth::run(client.clone(), cmd).await?,
            Command::User { cmd } => cli::user::run(client.clone(), cmd).await?,
            Command::Post { cmd } => cli::post::run(client.clone(), cmd).await?,
        };
    }

    println!("Exit succsess");
    Ok(())
}
use std::{io::Write, sync::Arc};
fn readline() -> Result<String, Error> {
    write!(std::io::stdout(), "$ ").map_err(|e| Error::Inner(e.to_string()))?;
    std::io::stdout()
        .flush()
        .map_err(|e| Error::Inner(e.to_string()))?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| Error::Inner(e.to_string()))?;
    Ok(buffer)
}
