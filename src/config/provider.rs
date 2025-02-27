//! Cryptographic service providers: signing backends


pub mod fortanixdsm;
use self::fortanixdsm::FortanixDsmConfig;
use serde::Deserialize;
use std::fmt;

/// Provider configuration
#[derive(Default, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProviderConfig {
    /// Fortanix DSM provider configurations    
    #[serde(default)]
    pub fortanixdsm: Vec<FortanixDsmConfig>,
}

/// Types of cryptographic keys
// TODO(tarcieri): move this into a provider-agnostic module
#[derive(Clone, Debug, Deserialize)]
pub enum KeyType {
    /// Account keys
    #[serde(rename = "account")]
    Account,

    /// Consensus keys
    #[serde(rename = "consensus")]
    Consensus,
}

impl Default for KeyType {
    /// Backwards compat for existing configuration files
    fn default() -> Self {
        KeyType::Consensus
    }
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyType::Account => f.write_str("account"),
            KeyType::Consensus => f.write_str("consensus"),
        }
    }
}
