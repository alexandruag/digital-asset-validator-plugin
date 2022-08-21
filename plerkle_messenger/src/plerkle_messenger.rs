use crate::error::MessengerError;
#[cfg(feature = "pulsar")]
use crate::PulsarMessenger;
#[cfg(feature = "redis")]
use crate::RedisMessenger;
use async_trait::async_trait;
use figment::value::{Dict, Value};
use serde::Deserialize;

/// Some constants that can be used as stream key values.
pub const ACCOUNT_STREAM: &str = "ACC";
pub const SLOT_STREAM: &str = "SLT";
pub const TRANSACTION_STREAM: &str = "TXN";
pub const BLOCK_STREAM: &str = "BLK";

#[async_trait]
pub trait Messenger: Sync + Send {
    async fn new(config: MessengerConfig) -> Result<Self, MessengerError>
    where
        Self: Sized;

    async fn add_stream(&mut self, stream_key: &'static str) -> Result<(), MessengerError>;
    async fn set_buffer_size(&mut self, stream_key: &'static str, max_buffer_size: usize);
    async fn send(&mut self, stream_key: &'static str, bytes: &[u8]) -> Result<(), MessengerError>;
    async fn recv(&mut self, stream_key: &'static str)
        -> Result<Vec<(i64, &[u8])>, MessengerError>;
}

#[derive(Deserialize, Debug, Default, PartialEq)]
pub struct MessengerConfig {
    pub connection_config: Dict,
}

impl MessengerConfig {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.connection_config.get(key)
    }
}

#[cfg(test)]
mod tests {
    use crate::MessengerConfig;
    use figment::providers::Env;
    use figment::value::Dict;
    use figment::{Figment, Jail};
    use serde::Deserialize;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Container {
        messenger_config: MessengerConfig,
    }

    #[test]
    fn test_config_deser() {
        Jail::expect_with(|jail| {
            jail.set_env("MESSENGER_CONFIG.messenger_type", "Redis");
            jail.set_env(
                "MESSENGER_CONFIG.connection_config",
                r#"{redis_connection_str="redis://redis"}"#,
            );

            let config: Container = Figment::from(Env::raw()).extract()?;
            let mut expected_dict = Dict::new();
            expected_dict.insert("redis_connection_str".to_string(), "redis://redis".into());
            assert_eq!(
                config.messenger_config,
                MessengerConfig {
                    connection_config: expected_dict,
                }
            );
            Ok(())
        });
    }
}
