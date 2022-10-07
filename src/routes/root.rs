use super::{auth_routes, email_routes, notes_routes, project_routes, todo_routes};
use axum::Router;

pub fn router() -> axum::Router {
    Router::new()
        .nest("/auth", auth_routes::routes())
        .nest("/emails", email_routes::routes())
        .nest("/projects", project_routes::routes())
        .nest("/notes", notes_routes::routes())
        .nest("/todo", todo_routes::routes())
}
