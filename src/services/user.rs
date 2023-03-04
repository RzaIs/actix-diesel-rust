use crate::entities::user::User;
use crate::helpers::error::Error;
use crate::repositories::user::{UserRepoTrait};

pub struct UserService<Repo: UserRepoTrait> {
    pub repo: Repo
}

impl<Repo: UserRepoTrait> UserService<Repo> {
    pub fn get_user_by_id(&self, id: i32) -> Result<User, Error> {
        self.repo.get_user_by_id(id)
    }
}
