use crate::entities::auth::RegisterDTO;
use crate::entities::user::CreateUser;

impl RegisterDTO {
    pub fn to_create_user(&self, hash: String) -> CreateUser {
        CreateUser {
            email: self.email.clone(),
            username: self.username.clone(),
            secret: hash.clone()
        }
    }
}
