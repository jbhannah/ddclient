use reqwest::{get, Error};
use std::net::{AddrParseError, IpAddr};

pub enum IpError {
    Request(Error),
    Parse(AddrParseError),
}

impl From<Error> for IpError {
    fn from(err: Error) -> Self {
        Self::Request(err)
    }
}

impl From<AddrParseError> for IpError {
    fn from(err: AddrParseError) -> Self {
        Self::Parse(err)
    }
}

pub async fn get_addr(endpoint: &str) -> Result<IpAddr, IpError> {
    Ok(get(endpoint).await?.text().await?.parse()?)
}
