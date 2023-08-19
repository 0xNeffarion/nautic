use url::{Host, Url};

use crate::errors::NauticDnsError;

pub fn parse_domain(target: &Url) -> Result<String, NauticDnsError> {
    let target_host = target
        .host()
        .ok_or_else(|| NauticDnsError::InvalidTarget(target.to_string()))?;
    let target_host = match target_host {
        Host::Domain(domain) => domain.to_owned(),
        _ => target
            .host_str()
            .ok_or_else(|| NauticDnsError::InvalidTarget(target.to_string()))?
            .to_owned(),
    };

    Ok(target_host)
}
