use dotenv::dotenv;
use std::env;

use std::collections::HashMap;
use reqwest::header::HeaderMap;

pub async fn run_post_call(url: &str, payload: HashMap<&str, &str>) -> Result<(), reqwest::Error> {
    //load env variables
    dotenv().ok();
    
    let admin_key = env::var("ADMIN_KEY").expect("Admin key should be set");
    let auth = format!("KakaoAK {admin_key}");

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        auth
            .parse()
            .unwrap(),
    );
    headers.insert(
        "Content-Type",
        "application/x-www-form-urlencoded;charset=utf-8"
            .parse()
            .unwrap(),
    );

    let response = client.post(url)
        .headers(headers)
        .form(&payload)
        .send()
        .await?;

    println!("Response: {:?}", response.text().await?);

    Ok(())
}
