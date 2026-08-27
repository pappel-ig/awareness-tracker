mod track;
mod participants;

use axum::Router;

pub fn router() -> Router {
    Router::new().merge(participants::router()).merge(track::router())
}
