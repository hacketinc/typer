use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sentence_uri = "https://typer-text.herokuapp.com/type-text/";

    let resp = reqwest::get(sentence_uri)
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    println!("{:#?}", resp.get("message").unwrap());
    Ok(())
}
