use std::fs::File;
use std::io;
use std::io::Write;
use reqwest::blocking::Client;
use serde::{Deserialize};

#[derive(Deserialize)]
struct ImgurData {
    link: String
}

#[derive(Deserialize)]
struct ImgurResponse {
    data: ImgurData,
    success: bool,
}

fn exit_with_message(message: String, exit_code: i32) {
    io::stdout().write_all(message.as_bytes()).unwrap();
    io::stdout().flush().unwrap();
    std::process::exit(exit_code);
}

fn main() {
    // Read the filepath from stdin
    let mut filepath = String::new();
    filepath = match io::stdin().read_line(&mut filepath) {
        Ok(..) => filepath.trim().into(),
        Err(error) => {
            return exit_with_message(format!("error: {}", error), 1);
        }
    };

    // Try to read the file given as input
    let file_res = File::open(filepath);
    let file = match file_res {
        Ok(file) => file,
        Err(error) => {
            return exit_with_message(format!("Error while reading file: {}", error), 1);
        }
    };

    let client_id: &'static str = env!("IMGUR_CLIENT_ID");
    let api_url: &'static str = "https://api.imgur.com/3/image";

    // Do the post request to the imgur api that uploads the image
    let client = Client::new();
    let res = client.post(api_url)
        .header("Authorization", format!("Client-ID {}", client_id))
        .body(file)
        .send();

    // Parse the json response
    let json = match res {
        Ok(t) => t.json::<ImgurResponse>(),
        Err(error) => {
            return exit_with_message(format!("Error while uploading to imgur: {}", error), 1);
        }
    };

    // Validate the response
    let result = match json {
        Ok(data) => data,
        Err(error) => {
            return exit_with_message(format!("Error while parsing imgur response: {}", error), 1);
        }
    };

    exit_with_message(result.data.link, 0)
}
