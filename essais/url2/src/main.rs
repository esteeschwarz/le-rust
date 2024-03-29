use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "https://ada-sub.dh-index.org/es/locale.txt";

    // Make an HTTP GET request
    let response = reqwest::get(url).await?;

    // Read the response body as a string
    let body = response.text().await?;
    println!("Response body:\n{}", body);

    Ok(())
}
