use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::stdin;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewNote {
    pub title: String,
    pub content: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let id = prompt_user("Enter the ID of the note to update:")?.parse::<i64>()?;
    let title = prompt_user("Enter the new title:")?;
    let content = prompt_user("Enter the new content:")?;

    let new_note = NewNote { title, content };

    let response = client
        .put(format!("http://127.0.0.1:8080/notes/{}", id))
        .json(&new_note)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Note updated successfully!");
        let updated_note: NewNote = response.json().await?;
        println!("Updated note: {:?}", updated_note);
    } else {
        eprintln!("Failed to update note: {:?}", response.text().await?);
    }

    Ok(())
}

fn prompt_user(prompt: &str) -> Result<String, Box<dyn Error>> {
    println!("{}", prompt);
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
