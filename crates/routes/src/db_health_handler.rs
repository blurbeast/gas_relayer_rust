use axum::extract::State;
use axum::http::StatusCode;
use db::db::DbState;
use crate::states::AppState;

pub async fn db_health_handler(state: State<AppState>) -> StatusCode {
    match DbState::ping_db(&state.db.pool).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}