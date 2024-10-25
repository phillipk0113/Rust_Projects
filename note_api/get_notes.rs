use reqwest::Client;
use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: i64,
    pub title: String,
    pub content: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a new HTTP client
    let client = Client::new();


    // Send the POST request
    let response = client
        .get("http://127.0.0.1:8080/notes")
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        let notes: Vec<Note> = response.json().await?;
        println!("Notes fetched successfully!");
        
        for note in notes {
            println!("Note ID: {}, Title: {}, Content: {}", note.id, note.title, note.content);
        }
    } else {
        eprintln!("Failed to fetch notes: {:?}", response.text().await?);
    }

    Ok(())
}
