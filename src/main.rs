use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
// use std::error::Error;

#[derive(Serialize, Deserialize)]
struct MinskyContentResponse {
    id: String,
    email: String,
}

const ENDPOINT: &str = "https://content.minsky.cc/potential-users";

const STRAPIENVVAR: &str = "STRAPI_KEY";

fn register_partner(email: &str) -> Result<MinskyContentResponse, Box<dyn std::error::Error>> {
    let payload = format!("{{\"email\": \"{:?}\"}}", email);
    let client = reqwest::blocking::Client::new();
    let res = client.post(ENDPOINT).body(payload).send()?;
    // Ok(format!("{:?}", res.json()?).to_string())
    println!("{}", res.status());
    let text = format!("{}", res.text()?);

    let m_res: MinskyContentResponse = serde_json::from_str(&text)?;
    Ok(m_res)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token: String = match env::var(STRAPIENVVAR) {
        Ok(var) => var,
        Err(_) => "".to_string(),
    };
    println!("strapi token: {}", token);

    let response: MinskyContentResponse = register_partner("bregymr001@gmail.com")?;

    println!("id: {}, email: {}", response.id, response.email);
    Ok(())
}
