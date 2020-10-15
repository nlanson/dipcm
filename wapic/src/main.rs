extern crate reqwest;
use text_io::read;
use serde::{Serialize, Deserialize}; //serde is used to convert json into structs and vice versa

#[derive(Serialize, Deserialize, Debug)]
struct Status {
    deprecated: bool,
    host: String,
    isitdown: bool,
    response_code: i32
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Enter site to check:");
    let site: String = read!();
    let api_addr: String = format!("https://isitdown.site/api/v3/{}", site);

    let client = reqwest::Client::new();
    let res = client.get(&site).send().await?;
    let res_stat = res.text().await?;

    let status: Status = serde_json::from_str(&res_stat).unwrap(); //convert res_stat (L18) to Struct Status (L5)

    println!("Is {} down? \n  {:?} \n  Response code {}", status.host, status.isitdown, status.response_code);

    Ok(())


}

