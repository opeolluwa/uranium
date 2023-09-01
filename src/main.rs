use app::Uranium;

/// Uranium is an authentication server for microservices
/// it is built primarily on gRPC transport layer
/// it also implement other protocol for communication of different layers
///
/// SMTP - for email transporter
/// AMQP - for message queue  system
/// HTTPS - the HTTP is used for

#[tokio::main]
async fn main() {
    // run the server
    Uranium::run().await;
}
