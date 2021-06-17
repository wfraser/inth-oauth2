use inth_oauth2_async::Client;
use inth_oauth2_async::provider::GitHub;
use std::io;

#[tokio::main]
async fn main() {
    let http_client = reqwest::Client::new();

    let client = Client::new(
        GitHub,
        String::from("01774654cd9a6051e478"),
        String::from("9f14d16d95d605e715ec1a9aecec220d2565fd5c"),
        Some(String::from("https://cmcenroe.me/oauth2-paste/")),
    );

    let auth_uri = client.auth_uri(Some("user"), None);
    println!("{}", auth_uri);

    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();

    let token = client.request_token(&http_client, code.trim()).await.unwrap();
    println!("{:?}", token);
}
