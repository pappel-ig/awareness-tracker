use std::env;
use std::net::IpAddr;

use anyhow::{Context, Result};
use maxminddb::{geoip2, Reader};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IpInformation {
    pub city: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub asn: Option<u32>,
    pub isp: Option<String>,
}

pub struct IpInformationService {
    city_db: Reader<Vec<u8>>,
    asn_db: Reader<Vec<u8>>,
}

impl IpInformationService {
    pub fn new() -> Result<Self> {
        let city_path = env::var("GEOIP_CITY_DB").unwrap_or_else(|_| "GeoLite2-City.mmdb".to_string());
        let asn_path = env::var("GEOIP_ASN_DB").unwrap_or_else(|_| "GeoLite2-ASN.mmdb".to_string());

        let city_db = Reader::open_readfile(&city_path)
            .with_context(|| format!("Unable to open GeoIP city database at {city_path}"))?;
        let asn_db = Reader::open_readfile(&asn_path)
            .with_context(|| format!("Unable to open GeoIP ASN database at {asn_path}"))?;

        Ok(Self { city_db, asn_db })
    }

    pub fn lookup(&self, ip: IpAddr) -> IpInformation {
        let city = self
            .city_db
            .lookup(ip)
            .ok()
            .and_then(|r| r.decode::<geoip2::City>().ok().flatten());
        let asn = self
            .asn_db
            .lookup(ip)
            .ok()
            .and_then(|r| r.decode::<geoip2::Asn>().ok().flatten());

        IpInformation {
            city: city
                .as_ref()
                .and_then(|c| c.city.names.english)
                .map(str::to_string),
            country: city
                .as_ref()
                .and_then(|c| c.country.names.english)
                .map(str::to_string),
            country_code: city
                .as_ref()
                .and_then(|c| c.country.iso_code)
                .map(str::to_string),
            asn: asn.as_ref().and_then(|a| a.autonomous_system_number),
            isp: asn
                .as_ref()
                .and_then(|a| a.autonomous_system_organization)
                .map(str::to_string),
        }
    }
}
