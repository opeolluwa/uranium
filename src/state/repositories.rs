use sea_orm::DatabaseConnection;

use crate::repositories::user_repository::UserRepository;

#[derive(Clone)]
pub struct Repositories {
    pub user: UserRepository,
}

impl Repositories {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self {
            user: UserRepository::init(db),
        }
    }
}
