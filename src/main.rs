use reqwest;
use serde::{Deserialize, Serialize};
use std::io;

mod tests;

#[derive(Debug, Serialize, Deserialize)]
struct APIResponse {
    login: String,
    url: String,
    name: String,
    followers: i16,
    following: i16
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Enter the user you want to query:");
    let mut user = String::new();
    io::stdin().read_line(&mut user).expect("Error typing username");

    println!("---------------------------------");

    let url = format!(
        "https://api.github.com/users/{set_user}",
        set_user = user
    );

    let response = reqwest::Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, "MyInfo")
        .send()
        .await?;

    if response.status().is_success() {
        let response: APIResponse = response.json().await?;
        setup_response(&response)
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}

fn setup_response(response: &APIResponse) {
    println!("Login: {}", response.login);
    println!("URL: {}", response.url);
    println!("Name: {}", response.name);
    println!("Followers: {}", response.followers);
    println!("Following: {}", response.following);
}