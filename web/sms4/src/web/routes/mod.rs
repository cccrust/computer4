pub mod follow;
pub mod interest;
pub mod like;
pub mod message;
pub mod post;
pub mod profile;
pub mod user;

use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn build_routes() -> Router<crate::web::AppState> {
    Router::new()
        .route("/users", get(user::list).post(user::create))
        .route(
            "/users/{id}",
            get(user::get).put(user::update).delete(user::delete),
        )
        .route("/users/{id}/timeline", get(user::timeline))
        .route("/users/{id}/followers", get(follow::followers))
        .route("/users/{id}/following", get(follow::following))
        .route("/posts", get(post::list).post(post::create))
        .route("/posts/{id}", get(post::get).delete(post::delete))
        .route("/posts/{id}/reply", post(post::reply))
        .route("/follow", post(follow::follow).delete(follow::unfollow))
        .route("/likes", post(like::like).delete(like::unlike))
        .route("/messages/send", post(message::send))
        .route("/messages/{user_id}/conversations", get(message::conversations))
        .route("/messages/{user_id}/unread", get(message::unread))
        .route("/messages/{user_id}/{other_id}", get(message::messages))
        .route("/profiles/{user_id}", get(profile::get).put(profile::update))
        .route("/profiles/search", get(profile::search))
        .route("/interests", post(interest::add).delete(interest::remove))
        .route("/interests/{user_id}", get(interest::list))
}
