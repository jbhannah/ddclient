pub mod cf;

use async_trait::async_trait;
use failure::Error;
use std::net::IpAddr;

#[async_trait]
pub trait Client {
    async fn check(&mut self, domain: &str) -> Result<(), Error>;
    async fn update(&self, domain: &str, addr: IpAddr) -> Result<(), Error>;
}
