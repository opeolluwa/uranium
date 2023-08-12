pub struct UserInformationHandler;

impl UserInformationHandler {
    pub async fn get_user_information() -> String {
        let user_information = "User Information";
        user_information.to_string()
    }

    pub async fn get_user_information_by_id() -> String {
        let id = 5;
        let user_information = format!("User Information by id {}", id);
        user_information.to_string()
    }
}
