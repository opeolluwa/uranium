///authentication controllers 
/// it contain the following endpoints 
/// POST `/auth/sign-up` - sign up a user
/// POST `/auth/login` - login a user
/// GET `/auth/me` - get user profile
/// PUT `/auth/me` - update the user profile    
pub mod auth_controllers;
///email controllers : contains handlers (controllers) to send, receive emails, reply email e.t.
pub mod email_controllers;
/// note entries handlers
pub mod notes_controllers;
///project controllers
pub mod project_controllers;
///todo list handlers - create, delete, update, get paginated list of todo e.t.
pub mod todo_controllers;