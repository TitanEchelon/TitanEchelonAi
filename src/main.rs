use reqwest;
use serde_json::Value;
use std::io;

#[tokio::main]
async fn main() {
    // Prompt user for a URL
    println!("Enter a URL to fetch:");
    let mut url = String::new();
    io::stdin().read_line(&mut url).expect("Failed to read line");
    let url = url.trim(); // Remove trailing newline

    // Fetch and handle errors
    match reqwest::get(url).await {
        Ok(response) => {
            if response.status().is_success() {
                let body = response.text().await.expect("Failed to read body");
                // Attempt to parse as JSON
                match serde_json::from_str::<Value>(&body) {
                    Ok(json) => println!("Parsed JSON: {:#?}", json),
                    Err(_) => println!("Response body (not JSON): {}", body),
                }
            } else {
                println!("Request failed with status: {}", response.status());
            }
        }
        Err(e) => {
            println!("Error occurred: {}", e);
        }
    }
}
