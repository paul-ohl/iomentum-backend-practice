use std::sync::Arc;

use warp::{reject::Rejection, reply::Reply};

use crate::{
    domain::{
        dtos::user_dtos::{NewUserDto, UserDto, UserLoginInputDto},
        errors::Error,
        types::JwtClaims,
    },
    handlers::errors::result_to_warp_reply,
    AppState,
};

use super::password_hasher::verify;

type ReplyRes<T> = Result<T, Rejection>;

pub async fn get_all_users(app_state: Arc<AppState>) -> ReplyRes<impl Reply> {
    let users_res = app_state.user_model.get_users().await;
    let users_res = match users_res {
        Ok(users) => Ok(users
            .into_iter()
            .map(|u| u.into())
            .collect::<Vec<UserDto>>()),
        Err(e) => Err(e),
    };
    result_to_warp_reply(users_res)
}

pub async fn get_user_by_id(id: uuid::Uuid, app_state: Arc<AppState>) -> ReplyRes<impl Reply> {
    let user = app_state.user_model.get_user(id).await;
    let user: Result<UserDto, Error> = match user {
        Ok(user) => Ok(user.into()),
        Err(e) => Err(e),
    };
    result_to_warp_reply(user)
}

pub async fn get_user_by_username(
    username: String,
    app_state: Arc<AppState>,
) -> ReplyRes<impl Reply> {
    let user = app_state.user_model.get_user_by_username(username).await;
    let user: Result<UserDto, Error> = match user {
        Ok(user) => Ok(user.into()),
        Err(e) => Err(e),
    };
    result_to_warp_reply(user)
}

pub async fn update_user(
    id: uuid::Uuid,
    user_input: NewUserDto,
    app_state: Arc<AppState>,
) -> ReplyRes<impl Reply> {
    let user_id = match user_input.try_into() {
        Ok(user_modifications) => {
            app_state
                .user_model
                .update_user(id, user_modifications)
                .await
        }
        Err(e) => Err(e),
    };
    result_to_warp_reply(user_id)
}

pub async fn delete_user(id: uuid::Uuid, app_state: Arc<AppState>) -> ReplyRes<impl Reply> {
    let user_id = app_state.user_model.delete_user(id).await;
    result_to_warp_reply(user_id)
}

pub async fn register_user(
    user_input: NewUserDto,
    app_state: Arc<AppState>,
) -> ReplyRes<impl Reply> {
    match user_input.try_into() {
        Ok(new_user) => result_to_warp_reply(app_state.user_model.create_user(new_user).await),
        Err(e) => result_to_warp_reply(Err(e)),
    }
}

pub async fn login_user(
    user_login_input: UserLoginInputDto,
    app_state: Arc<AppState>,
) -> ReplyRes<impl Reply> {
    let res = match app_state
        .user_model
        .get_user_by_username_interal(user_login_input.username)
        .await
    {
        Ok(db_user) => {
            if verify(
                &user_login_input.password,
                db_user.password_hash.expose_secret(),
            ) {
                let claims = JwtClaims::new(
                    db_user.username.as_ref().to_string(),
                    db_user.id,
                    db_user.role.as_ref().to_string(),
                );
                app_state.jwt_handler.generate_token(claims)
            } else {
                Err(Error::InvalidPassword("Wrong password".to_string()))
            }
        }
        Err(e) => Err(e),
    };
    result_to_warp_reply(res)
}
