use std::time::Duration;

use kafka::{client::RequiredAcks, producer::Producer};

use crate::config::CONFIG;

pub struct Kafka {}

impl Kafka {
    pub fn producer() -> Producer {
        Producer::from_hosts(vec![CONFIG.kafka_url.to_owned()])
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()
            .unwrap()
    }
}
