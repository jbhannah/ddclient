use failure::Error;
use reqwest::get;
use std::net::IpAddr;

pub async fn get_addr(endpoint: &str) -> Result<IpAddr, Error> {
    Ok(get(endpoint).await?.text().await?.parse()?)
}
