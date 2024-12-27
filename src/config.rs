use serde::{Deserialize, Serialize};

lazy_static::lazy_static! {
  pub static ref  CONFIG : Config  = Config::parse();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
    pub jwt_signing_key: String,
    pub database_connection_string: String,
    pub database_connection_retries: u16,
    // pub kafka_url: String,
}

impl Config {
    pub fn parse() -> Self {
        let port = std::env::var("PORT")
            .expect("unable to parse port")
            .parse::<u16>()
            .expect("error parsing port");
        let jwt_signing_key =
            std::env::var("JWT_SIGNING_KEY").expect("Couldn't parse JWT_SIGNING_KEY env");
        let database_connection_string =
            std::env::var("DATABASE_URL").expect("Couldn't parse DATABASE_URL env");
        // let kafka_url = std::env::var("KAFKA_URL").expect("Couldn't parse KAFKA_URL env");

        Self {
            port,
            jwt_signing_key,
            database_connection_string,
            // kafka_url,
            ..Default::default()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 50051,
            jwt_signing_key: "".to_string(),
            database_connection_string: "".to_string(),
            database_connection_retries: 5,
            // kafka_url: "".to_string(),
        }
    }
}
