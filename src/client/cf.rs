use crate::client::Client;
use async_trait::async_trait;
use cloudflare::endpoints::{dns, zone};
use cloudflare::framework::async_api::{self, ApiClient};
use cloudflare::framework::{auth, Environment, HttpApiClientConfig};
use failure::Error;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;

pub struct Cloudflare {
    client: async_api::Client,
    zones: HashMap<String, String>,
}

fn zone_from_domain(domain: &str) -> String {
    domain
        .rsplitn(3, '.')
        .take(2)
        .collect::<Vec<_>>()
        .iter()
        .rev()
        .map(|s| s.to_owned())
        .collect::<Vec<_>>()
        .join(".")
}

#[async_trait]
impl Client for Cloudflare {
    async fn check(&mut self, domain: &str) -> Result<(), Error> {
        let name = zone_from_domain(domain);

        if self.zones.contains_key(&name) {
            Ok(())
        } else {
            let zone = self.get_zone_by_name(&name).await?;
            self.zones.insert(name.to_owned(), (*zone.id).to_string());
            println!("{:#?}", self.zones);
            Ok(())
        }
    }

    async fn update(&self, domain: &str, addr: IpAddr) -> Result<(), Error> {
        let name = zone_from_domain(domain);
        let zone_identifier = self.zones.get(&name).expect("unknown zone ID");

        let content = match addr {
            IpAddr::V4(content) => dns::DnsContent::A { content },
            IpAddr::V6(content) => dns::DnsContent::AAAA { content },
        };

        let mut records = self.get_dns_records(zone_identifier, domain).await?;
        let result = if records.is_empty() {
            println!("no record, creating");

            let params = dns::CreateDnsRecordParams {
                ttl: Some(1),
                priority: None,
                proxied: None,
                name: domain,
                content,
            };

            self.client
                .request(&dns::CreateDnsRecord {
                    zone_identifier,
                    params,
                })
                .await?
                .result
        } else {
            let record = records.pop().unwrap();

            if record.content == content {
                println!("record matches, skipping update");

                record
            } else {
                println!("updating");

                let params = dns::UpdateDnsRecordParams {
                    ttl: Some(1),
                    proxied: None,
                    name: domain,
                    content,
                };

                let identifier = &record.id;

                self.client
                    .request(&dns::UpdateDnsRecord {
                        zone_identifier,
                        identifier,
                        params,
                    })
                    .await?
                    .result
            }
        };

        println!("{:#?}", result);
        Ok(())
    }
}

impl Cloudflare {
    pub fn new(email: &str, key: &str) -> Result<Self, Error> {
        let credentials = auth::Credentials::UserAuthKey {
            email: email.to_owned(),
            key: key.to_owned(),
        };
        let client = async_api::Client::new(
            credentials,
            HttpApiClientConfig::default(),
            Environment::Production,
        )?;

        Ok(Self {
            client,
            zones: HashMap::new(),
        })
    }

    async fn get_dns_records(
        &self,
        zone_identifier: &str,
        name: &str,
    ) -> Result<Vec<dns::DnsRecord>, Error> {
        let mut params = dns::ListDnsRecordsParams::default();
        params.name = Some(name.to_owned());

        let endpoint = &dns::ListDnsRecords {
            zone_identifier,
            params,
        };

        Ok(self.client.request(endpoint).await?.result)
    }

    async fn get_zone_by_name(&self, name: &str) -> Result<Arc<zone::Zone>, Error> {
        let mut params = zone::ListZonesParams::default();
        params.name = Some(name.to_owned());

        let endpoint = &zone::ListZones { params };
        if let Some(zone) = self.client.request(endpoint).await?.result.pop() {
            Ok(Arc::from(zone))
        } else {
            Err(failure::err_msg("no zones found"))
        }
    }
}
