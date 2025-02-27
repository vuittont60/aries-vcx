use std::{collections::HashMap, fmt};

use serde_json::value::Value;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    pub id: String,
    pub storage_type: Option<String>,
    pub storage_config: Option<Value>,
    pub cache: Option<CacheConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CachingAlgorithm {
    #[serde(rename = "lru")]
    LRU,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheConfig {
    #[serde(default = "default_cache_size")]
    pub size: usize,
    pub entities: Vec<String>,
    #[serde(default = "default_caching_algorithm")]
    pub algorithm: CachingAlgorithm,
}

pub const DEFAULT_CACHE_SIZE: usize = 10;

fn default_cache_size() -> usize {
    DEFAULT_CACHE_SIZE
}

fn default_caching_algorithm() -> CachingAlgorithm {
    CachingAlgorithm::LRU
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credentials {
    pub key: String,
    #[serde(default = "default_key_derivation_method")]
    pub key_derivation_method: KeyDerivationMethod,

    pub rekey: Option<String>,
    #[serde(default = "default_key_derivation_method")]
    pub rekey_derivation_method: KeyDerivationMethod,

    pub storage_credentials: Option<Value>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum KeyDerivationMethod {
    RAW,
    ARGON2I_MOD,
    ARGON2I_INT,
}

pub fn default_key_derivation_method() -> KeyDerivationMethod {
    KeyDerivationMethod::ARGON2I_MOD
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExportConfig {
    pub key: String,
    pub path: String,
    #[serde(default = "default_key_derivation_method")]
    pub key_derivation_method: KeyDerivationMethod,
}

#[derive(Debug, Deserialize)]
pub struct KeyConfig {
    pub seed: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Record {
    // Wallet record type
    #[serde(rename = "type")]
    pub type_: String,
    // Wallet record id
    pub id: String,
    // Wallet record value
    pub value: String,
    // Wallet record tags
    pub tags: HashMap<String, String>,
}

impl fmt::Debug for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Record")
            .field("type_", &self.type_)
            .field("id", &self.id)
            // Censor the value
            .field("value", &"******".to_string())
            .field("tags", &self.tags)
            .finish()
    }
}

pub type Tags = HashMap<String, String>;
