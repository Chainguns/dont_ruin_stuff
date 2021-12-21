use colored::*;
use hyper::{body, Body, Client, Method, Request};
use hyper_rustls::HttpsConnectorBuilder;
use std::fs::File;
use std::io::Read;

pub async fn get_access(action: &str) -> bool {
    let mut file = match File::open("token.txt") {
        Ok(f) => f,
        Err(_) => {
            println!(
                "{}",
                "file \"token.txt\" not found, make sure that you have it in this directory".red()
            );
            println!("{}", "to get your token go to your user details dashboard at https://www.blstsecurity.com/firecracker/UserDetails".purple().bold());
            return false;
        }
    };
    let mut token = String::new();
    match file.read_to_string(&mut token) {
        Ok(_) => (),
        Err(_) => {
            println!(
                "{}",
                "could not read the data from \"token.txt\", make sure the data is valid".red()
            );
            println!("{}", "to get your token go to your user details dashboard at https://www.blstsecurity.com/firecracker/UserDetails".purple().bold());
            return false;
        }
    }

    let connector = HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_only()
        .enable_http1()
        .enable_http2()
        .build();
    let client = Client::builder().build(connector);
    let req = Request::builder()
        .method(Method::POST)
        .uri("https://cli.blstsecurity.com/auth")
        .body(Body::from(format!(
            "{{\"client_token\":\"{}\",\"action\":\"{}\"}}",
            token, action
        )))
        .unwrap();

    let r = match client.request(req).await {
        Ok(r) => r,
        Err(_) => {
            println!("{}", "authentication request failed".red());
            return false;
        }
    };
    let txt = body::to_bytes(r.into_body()).await.unwrap();
    let json: serde_json::Value = match serde_json::from_slice(&txt) {
        Ok(j) => j,
        Err(_) => {
            println!("{}", "client_token not valid".red());
            println!("{}", "to get your token go to your user details dashboard at https://www.blstsecurity.com/firecracker/UserDetails".purple().bold());
            return false;
        }
    };
    match json["opt_in"].as_bool() {
        Some(b) => {
            if b {
                true
            } else {
                println!("{}", json["msg"].to_string().red());
                false
            }
        }
        None => {
            println!(
                "{}",
                "error while parsing the response from the authenticator".red()
            );
            false
        }
    }
}
