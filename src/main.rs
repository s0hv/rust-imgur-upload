use std::fs::File;
use std::io;
use std::io::Write;

use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json;

#[derive(Deserialize)]
struct ImgurData {
    link: String,
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
    let res = client
        .post(api_url)
        .header("Authorization", format!("Client-ID {}", client_id))
        .body(file)
        .send();

    // Read the response body as text so we can print it in case of an error
    let text_result = match res {
        Ok(t) => t.text(),
        Err(error) => {
            return exit_with_message(format!("Error while uploading to imgur: {}", error), 1);
        }
    };

    let text = match text_result {
        Ok(t) => t,
        Err(error) => {
            return exit_with_message(format!("Error while uploading to imgur: {}", error), 1);
        }
    };

    // Parse the json
    let json = serde_json::from_str::<ImgurResponse>(&*text);
    let result = match json {
        Ok(data) => data,
        Err(error) => {
            return exit_with_message(
                format!(
                    "Error while parsing imgur response from {}: {}",
                    text, error
                ),
                1,
            );
        }
    };

    exit_with_message(result.data.link, 0)
}
