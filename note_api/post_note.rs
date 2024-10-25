use reqwest::Client;
use serde::Serialize;
use std::error::Error;
use std::io::stdin;

#[derive(Serialize)]
struct NewNote {
    title: String,
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a new HTTP client
    let client = Client::new();

    let title = prompt_user("Enter the title of the note:")?;
    let content = prompt_user("Enter the content of the note:")?;

    // Define the new note
    let new_note = NewNote {
        title,
        content,
    };

    // Send the POST request
    let response = client
        .post("http://127.0.0.1:8080/notes")
        .json(&new_note)
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        println!("Note created successfully!");
    } else {
        eprintln!("Failed to create note: {:?}", response.text().await?);
    }

    Ok(())
}

fn prompt_user(prompt: &str) -> Result<String, Box<dyn Error>> {
    println!("{}", prompt);
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
