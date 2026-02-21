use super::types::PostCmd;
use client::types::{Client, Error};
use std::sync::Arc;

pub async fn run(client: Arc<dyn Client>, cmd: PostCmd) -> Result<(), Error> {
    let mut post = client.post();

    match cmd {
        PostCmd::Create {
            title,
            content,
            img_base64,
        } => {
            let p = post.create(&title, &content, img_base64.as_deref()).await?;
            println!("{:#?}", p);
        }

        PostCmd::Update {
            post_id,
            title,
            content,
            img_base64,
        } => {
            let p = post
                .update(
                    &post_id,
                    title.as_deref(),
                    content.as_deref(),
                    img_base64.as_deref(),
                )
                .await?;
            println!("{:#?}", p);
        }

        PostCmd::Delete { post_id } => {
            post.delete(&post_id).await?;
            println!("post deleted");
        }

        PostCmd::Get { post_id } => {
            let p = post.get_by_id(&post_id).await?;
            println!("{:#?}", p);
        }

        PostCmd::My => {
            let posts = post.gets_me().await?;
            println!("{:#?}", posts);
        }

        PostCmd::ByAuthor { email } => {
            let posts = post.gets_by_author(&email).await?;
            println!("{:#?}", posts);
        }
    }

    Ok(())
}
