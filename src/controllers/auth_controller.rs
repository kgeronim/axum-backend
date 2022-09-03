use crate::{
    dto::{LoginDto, RegisterDto, AuthBodyDto},
    error,
    services::auth_service::AuthService,
    utils::jwt,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sqlx::PgPool;
use tracing::{event, instrument, Level};

#[instrument(skip_all)]
pub async fn login(
    State(pool): State<PgPool>,
    Json(login_dto): Json<LoginDto>,
) -> Result<impl IntoResponse, error::Error> {
    let user = AuthService::sign_in(login_dto, &pool).await.map_err(|e| {
        event!(Level::ERROR, "{e:?}");
        *e.current_context()
    })?;

    let token = jwt::sign(user.id).map_err(|e| {
        event!(Level::ERROR, "{e:?}");
        *e.current_context()
    })?;

    Ok(Json(AuthBodyDto::new(token)))
}

#[instrument(skip_all)]
pub async fn register(
    State(pool): State<PgPool>,
    Json(register_dto): Json<RegisterDto>,
) -> Result<impl IntoResponse, error::Error> {
    let user = AuthService::sign_up(register_dto, &pool)
        .await
        .map_err(|e| {
            event!(Level::ERROR, "{e:?}");
            *e.current_context()
        })?;

    Ok((StatusCode::OK, Json(user)))
}
