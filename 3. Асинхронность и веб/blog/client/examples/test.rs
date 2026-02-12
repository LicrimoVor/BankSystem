use client::{GrpcClient, HttpClient, types::Client};

const LOL: bool = false;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // такое решение ради CLI)
    let client: Box<dyn Client> = match LOL {
        true => Box::new(HttpClient::new("http://127.0.0.1:8001").await.unwrap()),
        false => Box::new(GrpcClient::new("http://127.0.0.1:50051").await.unwrap()),
    };
    let mut general = client.general();
    let mut auth = client.auth();
    let mut post = client.post();
    let mut user = client.user();

    let resp = general.health().await;
    println!("general.health -- {:#?}", resp);

    let resp = auth.login("lol32323@lol.ru", "1q2w3e4r5t").await;
    println!("auth.login -- {:#?}", resp);

    let resp = post.gets_me().await;
    println!("post.gets_me -- {:#?}", resp);

    let resp = post.create("From client HTTP", "content", None).await;
    println!("post.create -- {:#?}", resp);

    let resp = post
        .update(
            &resp.unwrap().id,
            Some("From client HTTP!!!!!!"),
            None,
            None,
        )
        .await;
    println!("post.update -- {:#?}", resp);

    let resp = post.delete(&resp.unwrap().id).await;
    println!("post.delete -- {:#?}", resp);

    let resp = auth.logout().await;
    println!("auth.logout -- {:#?}", resp);

    let resp = user.me().await;
    println!("user.me -- {:#?}", resp);

    let resp = auth
        .register("ImDeleted", "ImDelted@Del.Del", "DelDelDelDel")
        .await;
    println!("auth.register -- {:#?}", resp);

    let resp = user.delete().await;
    println!("user.delete -- {:#?}", resp);

    Ok(())
}
