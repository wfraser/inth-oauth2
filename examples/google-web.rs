use inth_oauth2_async::Client;
use inth_oauth2_async::provider::google::Web;
use std::io;

#[tokio::main]
async fn main() {
    let http_client = reqwest::Client::new();

    let client = Client::new(
        Web,
        String::from("143225766783-0h4h5ktpvhc7kqp6ohbpd2sssqrap57n.apps.googleusercontent.com"),
        String::from("7Xjn-vRN-8qsz3Zh9zZGkHsM"),
        Some(String::from("https://cmcenroe.me/oauth2-paste/")),
    );

    let auth_uri = client.auth_uri(Some("https://www.googleapis.com/auth/userinfo.email"), None);
    println!("{}", auth_uri);

    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();

    let token = client.request_token(&http_client, code.trim()).await.unwrap();
    println!("{:?}", token);
}
