use client::grpc::GrpcClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = GrpcClient::new("http://127.0.0.1:50051").await.unwrap();
    let mut general = client.general();
    let mut auth = client.auth();
    let mut post = client.post();
    let mut user = client.user();

    let resp = general.health().await;
    println!("{:#?}", resp);

    let resp = auth
        .login("lol32323@lol.ru".to_string(), "1q2w3e4r5t".to_string())
        .await;
    println!("{:#?}", resp);

    let resp = post.gets_me().await;
    println!("{:#?}", resp);

    // let resp = post
    //     .create("From client grpc".to_string(), "content".to_string(), None)
    //     .await;
    // println!("{:#?}", resp);

    let reps = auth.logout().await;
    println!("{:#?}", reps);

    let reps = user.me().await;
    println!("{:#?}", reps);

    Ok(())
}
