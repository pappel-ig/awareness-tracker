mod track;
mod participants;
mod information;

use axum::Router;

pub fn router() -> Router {
    Router::new()
        .merge(participants::router())
        .merge(track::router())
        .merge(information::router())
}
