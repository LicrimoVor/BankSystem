use super::types::AuthCmd;
use crate::types::{Client, Error};
use std::sync::Arc;

pub async fn run(client: Arc<dyn Client>, cmd: AuthCmd) -> Result<(), Error> {
    let mut auth = client.auth();

    match cmd {
        AuthCmd::Register {
            username,
            email,
            password,
        } => {
            let user = auth.register(&username, &email, &password).await?;
            println!("{:#?}", user);
        }
        AuthCmd::Login { email, password } => {
            let user = auth.login(&email, &password).await?;
            println!("{:#?}", user);
        }
        AuthCmd::Logout => {
            auth.logout().await?;
            println!("logged out");
        }
    }

    Ok(())
}
