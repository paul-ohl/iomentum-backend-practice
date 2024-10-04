use uuid::Uuid;

use crate::domain::{
    errors::Result,
    types::user_types::{InternalUseUser, NewUser, User},
};

#[async_trait::async_trait]
pub trait UsersModel: Send + Sync {
    async fn get_users(&self) -> Result<Vec<User>>;

    /// This function is used to get the user without the password hash
    async fn get_user(&self, id: Uuid) -> Result<User>;

    /// This function is used to get the user without the password hash
    async fn get_user_by_username(&self, username: String) -> Result<User>;

    /// This function is used internally to get the user with the password hash
    /// Do not expose this function to the outside world
    async fn get_user_by_username_interal(&self, username: String) -> Result<InternalUseUser>;

    async fn create_user(&self, user: NewUser) -> Result<Uuid>;

    async fn update_user(&self, id: Uuid, user: NewUser) -> Result<Uuid>;

    async fn delete_user(&self, id: Uuid) -> Result<()>;
}
