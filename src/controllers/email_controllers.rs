use axum::response::IntoResponse;

///send email
/// receive the user email, subject, fullname and message
/// call on lettre to dispatch the mail to the user
pub async fn send_email() -> impl IntoResponse {
    todo!()
}

///receive email being sent from the portfolio
/// store it in the database
pub async fn receive_email() -> impl IntoResponse {
    todo!()
}

///reply email
/// receive only the user email and subject and message
/// send the message to the user
pub async fn reply_email() -> impl IntoResponse {
    todo!()
}

///delete email
///receive the id of the mail to delete
///exec the query on the database
/// return result
pub async fn delete_email() -> impl IntoResponse {
    todo!()
}

///fetch email
/// retrieve an email from the data store
pub async fn fetch_email() -> impl IntoResponse {
    todo!()
}

///star email
/// mark email as important
pub async fn star_email() -> impl IntoResponse {
    todo!()
}
