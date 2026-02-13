use super::types::UserCmd;
use crate::types::{Client, Error};
use std::sync::Arc;

pub async fn run(client: Arc<dyn Client>, cmd: UserCmd) -> Result<(), Error> {
    let mut user = client.user();

    match cmd {
        UserCmd::Me => {
            let me = user.me().await?;
            println!("{:#?}", me);
        }

        UserCmd::Update {
            username,
            email,
            password,
        } => {
            let updated = user
                .update(username.as_deref(), email.as_deref(), password.as_deref())
                .await?;
            println!("{:#?}", updated);
        }

        UserCmd::Delete => {
            user.delete().await?;
            println!("user deleted");
        }

        UserCmd::GetByEmail { email } => {
            let u = user.get_by_email(&email).await?;
            println!("{:#?}", u);
        }
    }

    Ok(())
}
