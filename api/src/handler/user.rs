use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use garde::Validate;
use kernel::model::{id::UserId, user::event::DeleteUser};
use registry::AppRegistry;
use shared::error::{AppError, AppResult};

use crate::{
    extractor::AuthorizedUser,
    model::{
        checkout::CheckoutsResponse,
        user::{
            CreateUserRequest, UpdateUserPasswordRequest, UpdateUserPasswordRequestWithUserId,
            UpdateUserRoleRequest, UpdateUserRoleRequestWithuserId, UserResponse, UsersResponse,
        },
    },
};

pub async fn register_user(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateUserRequest>,
) -> AppResult<Json<UserResponse>> {
    if !user.is_admin() {
        return Err(AppError::ForbiddenOperation);
    }

    req.validate(&())?;

    let registered_user = registry.user_repository().create(req.into()).await?;
    Ok(Json(registered_user.into()))
}

#[tracing::instrument(
    // drop argument from log, and add user_id field to log
    skip(_user, registry),
    fields(user_id = %_user.id().to_string(), test = "this is static ".to_string())
)]
pub async fn list_users(
    _user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<UsersResponse>> {
    let items = registry
        .user_repository()
        .find_all()
        .await?
        .into_iter()
        .map(UserResponse::from)
        .collect();

    Ok(Json(UsersResponse { items }))
}

pub async fn delete_user(
    user: AuthorizedUser,
    Path(user_id): Path<UserId>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    if !user.is_admin() {
        return Err(AppError::ForbiddenOperation);
    }

    registry
        .user_repository()
        .delete(DeleteUser { user_id })
        .await?;

    Ok(StatusCode::OK)
}

pub async fn change_role(
    user: AuthorizedUser,
    Path(user_id): Path<UserId>,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateUserRoleRequest>,
) -> AppResult<StatusCode> {
    if !user.is_admin() {
        return Err(AppError::ForbiddenOperation);
    }

    registry
        .user_repository()
        .update_role(UpdateUserRoleRequestWithuserId::new(user_id, req).into())
        .await?;
    Ok(StatusCode::OK)
}

pub async fn get_current_user(user: AuthorizedUser) -> Json<UserResponse> {
    Json(UserResponse::from(user.user))
}

pub async fn change_password(
    _user: AuthorizedUser,
    Path(user_id): Path<UserId>,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateUserPasswordRequest>,
) -> AppResult<StatusCode> {
    req.validate(&())?;

    registry
        .user_repository()
        .update_password(UpdateUserPasswordRequestWithUserId::new(user_id, req).into())
        .await?;
    Ok(StatusCode::OK)
}

pub async fn get_checkouts(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<CheckoutsResponse>> {
    registry
        .checkout_repository()
        .find_unreturned_by_user_id(user.id())
        .await
        .map(CheckoutsResponse::from)
        .map(Json)
}
