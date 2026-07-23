use serde::{Deserialize, Serialize};

/// Amazon Advertising marketplace with associated metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Marketplace {
    // North America
    US,
    CA,
    MX,
    BR,
    // Europe
    UK,
    DE,
    FR,
    IT,
    ES,
    NL,
    SE,
    PL,
    BE,
    TR,
    AE,
    SA,
    EG,
    IN,
    // Far East
    JP,
    AU,
    SG,
}

impl Marketplace {
    /// Returns the Amazon marketplace ID.
    pub fn id(&self) -> &'static str {
        match self {
            // North America
            Marketplace::US => "ATVPDKIKX0DER",
            Marketplace::CA => "A2EUQ1WTGCTBG2",
            Marketplace::MX => "A1AM78C64UM0Y8",
            Marketplace::BR => "A2Q3Y263D00KWC",
            // Europe
            Marketplace::UK => "A1F83G8C2ARO7P",
            Marketplace::DE => "A1PA6795UKMFR9",
            Marketplace::FR => "A13V1IB3VIYZZH",
            Marketplace::IT => "APJ6JRA9NG5V4",
            Marketplace::ES => "A1RKKUPIHCS9HS",
            Marketplace::NL => "A1805IZSGTT6HS",
            Marketplace::SE => "A2NODRKZP88ZB9",
            Marketplace::PL => "A1C3SOZRARQ6R3",
            Marketplace::BE => "AMEN7PMS3EDWL",
            Marketplace::TR => "A33AVAJ2PDY3EV",
            Marketplace::AE => "A2VIGQ35RCS4UG",
            Marketplace::SA => "A17E79C6D8DWNP",
            Marketplace::EG => "ARBP9OOSHTCHU",
            Marketplace::IN => "A21TJRUUN4KGV",
            // Far East
            Marketplace::JP => "A1VC38T7YXB528",
            Marketplace::AU => "A39IBJ37TRP1C6",
            Marketplace::SG => "A19VAU5U5O7RUS",
        }
    }

    /// Returns the ISO currency code for this marketplace.
    pub fn currency(&self) -> &'static str {
        match self {
            Marketplace::US => "USD",
            Marketplace::CA => "CAD",
            Marketplace::MX => "MXN",
            Marketplace::BR => "BRL",
            Marketplace::UK | Marketplace::DE | Marketplace::FR
            | Marketplace::IT | Marketplace::ES | Marketplace::NL
            | Marketplace::SE | Marketplace::PL | Marketplace::BE
            | Marketplace::TR | Marketplace::AE | Marketplace::SA
            | Marketplace::EG => "EUR",
            Marketplace::IN => "INR",
            Marketplace::JP => "JPY",
            Marketplace::AU => "AUD",
            Marketplace::SG => "SGD",
        }
    }

    /// Returns the locale code for this marketplace.
    pub fn locale(&self) -> &'static str {
        match self {
            Marketplace::US => "en_US",
            Marketplace::CA => "en_CA",
            Marketplace::MX => "es_MX",
            Marketplace::BR => "pt_BR",
            Marketplace::UK => "en_GB",
            Marketplace::DE => "de_DE",
            Marketplace::FR => "fr_FR",
            Marketplace::IT => "it_IT",
            Marketplace::ES => "es_ES",
            Marketplace::NL => "nl_NL",
            Marketplace::SE => "sv_SE",
            Marketplace::PL => "pl_PL",
            Marketplace::BE => "fr_BE",
            Marketplace::TR => "tr_TR",
            Marketplace::AE => "ar_AE",
            Marketplace::SA => "ar_SA",
            Marketplace::EG => "ar_EG",
            Marketplace::IN => "en_IN",
            Marketplace::JP => "ja_JP",
            Marketplace::AU => "en_AU",
            Marketplace::SG => "en_SG",
        }
    }

    /// Returns the advertising API region for this marketplace.
    #[cfg(feature = "client")]
    pub fn region(&self) -> crate::client::config::Region {
        match self {
            Marketplace::US | Marketplace::CA | Marketplace::MX | Marketplace::BR => {
                crate::client::config::Region::NorthAmerica
            }
            Marketplace::UK | Marketplace::DE | Marketplace::FR
            | Marketplace::IT | Marketplace::ES | Marketplace::NL
            | Marketplace::SE | Marketplace::PL | Marketplace::BE
            | Marketplace::TR | Marketplace::AE | Marketplace::SA
            | Marketplace::EG | Marketplace::IN => {
                crate::client::config::Region::Europe
            }
            Marketplace::JP | Marketplace::AU | Marketplace::SG => {
                crate::client::config::Region::FarEast
            }
        }
    }
}

/// Standard rate limits for Advertising API endpoints.
/// Values represent requests per second (rate) and burst capacity.
pub enum RateLimit {
    /// Standard API: 5 rps, burst 5
    Standard,
    /// Reports API: 1 rps, burst 1
    Reports,
    /// Snapshots API: 1 rps, burst 1
    Snapshots,
    /// Bid recommendations: 2 rps, burst 2
    BidRecommendations,
}

impl RateLimit {
    pub fn rate(&self) -> f64 {
        match self {
            RateLimit::Standard => 5.0,
            RateLimit::Reports => 1.0,
            RateLimit::Snapshots => 1.0,
            RateLimit::BidRecommendations => 2.0,
        }
    }

    pub fn burst(&self) -> u32 {
        match self {
            RateLimit::Standard => 5,
            RateLimit::Reports => 1,
            RateLimit::Snapshots => 1,
            RateLimit::BidRecommendations => 2,
        }
    }
}
