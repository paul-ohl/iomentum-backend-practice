use std::sync::Arc;

use warp::{reject::Rejection, reply::Reply};

use crate::{
    domain::{
        dtos::user_dtos::{UserInputDto, UserLoginInputDto},
        errors::Error,
        types::JwtClaims,
    },
    handlers::errors::result_to_warp_reply,
    models::users,
    AppState,
};

use super::password_hasher::verify;

type ReplyRes<T> = Result<T, Rejection>;

pub async fn get_all_users(app_state: Arc<AppState>) -> ReplyRes<impl Reply> {
    let users = users::get_all(&app_state.db_pool).await;
    result_to_warp_reply(users)
}

pub async fn get_user_by_id(id: uuid::Uuid, app_state: Arc<AppState>) -> ReplyRes<impl Reply> {
    let user = users::get_by_id(&app_state.db_pool, id).await;
    result_to_warp_reply(user)
}

pub async fn get_user_by_username(
    username: String,
    app_state: Arc<AppState>,
) -> ReplyRes<impl Reply> {
    let users = users::get_by_username(&app_state.db_pool, username).await;
    result_to_warp_reply(users)
}

pub async fn update_user(
    id: uuid::Uuid,
    user_input: UserInputDto,
    app_state: Arc<AppState>,
) -> ReplyRes<impl Reply> {
    match user_input.try_into() {
        Ok(user_modifications) => {
            result_to_warp_reply(users::update(&app_state.db_pool, id, user_modifications).await)
        }
        Err(e) => result_to_warp_reply(Err(e)),
    }
}

pub async fn delete_user(id: uuid::Uuid, app_state: Arc<AppState>) -> ReplyRes<impl Reply> {
    let user_id = users::delete(&app_state.db_pool, id).await;
    result_to_warp_reply(user_id)
}

pub async fn register_user(
    user_input: UserInputDto,
    app_state: Arc<AppState>,
) -> ReplyRes<impl Reply> {
    match user_input.try_into() {
        Ok(new_user) => result_to_warp_reply(users::create(&app_state.db_pool, new_user).await),
        Err(e) => result_to_warp_reply(Err(e)),
    }
}

pub async fn login_user(
    user_login_input: UserLoginInputDto,
    app_state: Arc<AppState>,
) -> ReplyRes<impl Reply> {
    let res = match users::get_by_username_for_verification(
        &app_state.db_pool,
        user_login_input.username,
    )
    .await
    {
        Ok(db_user) => {
            if verify(&user_login_input.password, &db_user.password_hash) {
                let claims = JwtClaims::new(db_user.username, db_user.id, db_user.role);
                app_state.jwt_handler.generate_token(claims)
            } else {
                Err(Error::InvalidPassword("Wrong password".to_string()))
            }
        }
        Err(e) => Err(e),
    };
    result_to_warp_reply(res)
}
