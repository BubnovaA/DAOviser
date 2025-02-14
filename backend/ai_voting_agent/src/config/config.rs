use std::env;
use std::error::Error;
use std::fmt;

#[derive(Clone)]
pub struct Config {
    pub openai_api_key: String,
    pub arbitrum_rpc_url: String,
    pub dao_contract_address: String,
    pub safe_wallet_address: String,
    pub safe_wallet_private_key: String,
    pub pg_user: String,
    pub pg_pass: String,
    pub pg_host: String,
    pub pg_db: String,
}

#[derive(Debug)]
struct ConfigError(String);

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Configurations error: {}", self.0)
    }
}

impl Error for ConfigError {}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        dotenv::dotenv().ok();
        let openai_api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| ConfigError("OPENAI_API_KEY не установлен".into()))?;
        let arbitrum_rpc_url = env::var("ARBITRUM_RPC_URL")
            .map_err(|_| ConfigError("ARBITRUM_RPC_URL не установлен".into()))?;
        let dao_contract_address = env::var("DAO_CONTRACT_ADDRESS")
            .map_err(|_| ConfigError("DAO_CONTRACT_ADDRESS не установлен".into()))?;
        let safe_wallet_address = env::var("SAFE_WALLET_ADDRESS")
            .map_err(|_| ConfigError("SAFE_WALLET_ADDRESS не установлен".into()))?;
        let safe_wallet_private_key = env::var("SAFE_WALLET_PRIVATE_KEY")
            .map_err(|_| ConfigError("SAFE_WALLET_PRIVATE_KEY не установлен".into()))?; 
        let pg_user = env::var("PG_USER").unwrap_or_else(|_| "".to_string());
        let pg_pass = env::var("PG_PASS").unwrap_or_default();
        let pg_host = env::var("PG_HOST").unwrap_or_else(|_| "172.17.0.1".to_string());
        let pg_db = env::var("PG_DB").unwrap_or_else(|_| "".to_string());

        Ok(Config {
            openai_api_key,
            arbitrum_rpc_url,
            dao_contract_address,
            safe_wallet_address,
            safe_wallet_private_key,
            pg_user,
            pg_pass,
            pg_host,
            pg_db
        })
    }

    pub fn to_pg_connection_string(&self) -> String {
        match self.pg_pass.is_empty() {
            true => format!(
                "postgres://{}@{}:5432/{}",
                self.pg_user, self.pg_host, self.pg_db
            ),
            false => format!(
                "postgres://{}:{}@{}:5432/{}",
                self.pg_user, self.pg_pass, self.pg_host, self.pg_db
            ),
        }
    }
}