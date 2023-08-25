use std::sync::Arc;

use tokio::net::UdpSocket;
use url::Url;

use crate::{errors::NauticDnsError, util::parse_domain};

pub struct DnsResolver {
    socket: Arc<UdpSocket>,
}

impl DnsResolver {
    pub async fn new(server: &str, port: u16) -> Result<Self, NauticDnsError> {
        let hostname = format!("{server}:{port}");
        let bind = UdpSocket::bind(&hostname).await?;

        Ok(Self {
            socket: Arc::new(bind),
        })
    }

    pub async fn resolve(
        &self,
        remote_resolver: &str,
        target: &Url,
    ) -> Result<String, NauticDnsError> {
        let domain = parse_domain(target)?;

        todo!()
    }
}
