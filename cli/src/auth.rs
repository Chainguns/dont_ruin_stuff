use colored::*;
use hyper::{body, Body, Client, Method, Request};
use hyper_rustls::HttpsConnectorBuilder;
use std::fs::File;
use std::io::{Read,Write};
use std::env::set_current_dir;

const TOKEN_FILE:&str = ".cherrybomb/token.txt";
async fn sign_up()->bool{
    match set_current_dir(dirs::home_dir().unwrap()){
        Ok(_)=>(),
        Err(e)=>{
            println!("{:?}",e);
            panic!("Could not generate a CLI token, please contact BLST at support@blstsecurity.com");
        }
    };
    let mut file = match File::create(TOKEN_FILE) {
        Ok(f) => f,
        Err(e) => {
            println!("{:?}",e);
            panic!("Could not generate a CLI token, please contact BLST at support@blstsecurity.com");
        }
    };
    let res = match reqwest::get("https://cherrybomb.blstsecurity.com/token").await{
        Ok(r)=>{
            match r.text().await{
                Ok(t)=>t,
                Err(_)=>{
                    panic!("Could not generate a CLI token, please contact BLST at support@blstsecurity.com");
                }
            }
        },
        Err(e)=>{
            println!("{:?}",e);
            panic!("Could not generate a CLI token, please contact BLST at support@blstsecurity.com");
        }
    };
    let json: serde_json::Value = match serde_json::from_str(&res) {
        Ok(j) => j,
        Err(_) => {
            panic!("Could not generate a CLI token, please contact BLST at support@blstsecurity.com");
        }
    };
    match file.write_all(json["client_token"].to_string().as_bytes()){
        Ok(_)=>(),
        Err(_)=>{
            panic!("Could not generate a CLI token, please contact BLST at support@blstsecurity.com");
        }
    }
    true
}
pub async fn get_access(action: &str) -> bool {
    let mut file = match File::open(TOKEN_FILE) {
        Ok(f) => f,
        Err(_) => {
            if sign_up().await{
            match File::open(TOKEN_FILE) {
                Ok(f)=>f,
                Err(_)=>{
                    panic!("Could not validate the CLI token, please contact BLST at support@blstsecurity.com");
                }
            }
            }else{
                    panic!("Could not validate the CLI token, please contact BLST at support@blstsecurity.com");
            }
            /*
            println!(
                "{}",
                "file \"token.txt\" not found, make sure that you have it in this directory".red()
            );
            println!("{}", "to get your token go to your user details dashboard at https://www.blstsecurity.com/cherrybomb/UserDetails".purple().bold());
            return false;
            */
        }
    };
    let mut token = String::new();
    match file.read_to_string(&mut token) {
        Ok(_) => (),
        Err(_) => {
            panic!("Could not validate the CLI token, please contact BLST at support@blstsecurity.com");
            /*
            println!(
                "{}",
                "could not read the data from \"token.txt\", make sure the data is valid".red()
            );
            println!("{}", "to get your token go to your user details dashboard at https://www.blstsecurity.com/cherrybomb/UserDetails".purple().bold());*/
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
        .uri("https://cherrybomb.blstsecurity.com/auth")
        .body(Body::from(format!(
            "{{\"client_token\":{},\"action\":\"{}\"}}",
            token, action
        ).replace("\n","")))
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
            panic!("Invalid CLI token, please contact BLST at support@blstsecurity.com");
            /*
            println!("{}", "client_token not valid".red());
            println!("{}", "to get your token go to your user details dashboard at https://www.blstsecurity.com/cherrybomb/UserDetails".purple().bold());*/
        }
    };
    match json["opt_in"].as_bool() {
        Some(b) => {
            if b {
                true
            } else {
                panic!("Invalid CLI token, please contact BLST at support@blstsecurity.com");
                //println!("{}", json["msg"].to_string().red());
                //false
            }
        }
        None => {
            panic!("Invalid CLI token, please contact BLST at support@blstsecurity.com");
            /*
            println!(
                "{}",
                "error while parsing the response from the authenticator".red()
            );
            false*/
        }
    }
}
