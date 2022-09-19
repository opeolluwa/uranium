use axum::response::IntoResponse;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use axum::Json;
///send email
/// receive the user email, subject, fullname and message
/// call on lettre to dispatch the mail to the user
pub async fn send_email() -> impl IntoResponse {
    // 
     let email = Message::builder()
        .from("Nitride <opeolluwa@nitride.tld>".parse().unwrap())
        .reply_to("Yuin <opeolluwa@gmail.com>".parse().unwrap())
        .to("Hei <adefem>".parse().unwrap())
        .subject("Happy new year")
        .body(String::from("Be happy!"))
        .unwrap();

    let creds = Credentials::new("".to_string(), "kfv".to_string());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    let res = match mailer.send(&email) {
        Ok(_) => format!("Email sent successfully!"),
        Err(e) => format!("Could not send email: {:?}", e),
    };

    Json(res)
}

///receive email being sent from the portfolio
/// store it in the database
pub async fn receive_email() -> impl IntoResponse {
    
}

///reply email
/// receive only the user email and subject and message
/// send the message to the user
pub async fn reply_email() -> impl IntoResponse {
    
}

///delete email
///receive the id of the mail to delete
///exec the query on the database
/// return result
pub async fn delete_email() -> impl IntoResponse {
    
}

///fetch email
/// retrieve an email from the data store
pub async fn fetch_email() -> impl IntoResponse {
    
}

///star email
/// mark email as important
pub async fn star_email() -> impl IntoResponse {
    
}
