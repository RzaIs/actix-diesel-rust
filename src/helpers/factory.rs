use crate::helpers::config::Secrets;
use crate::repositories::article::{ArticleRepo};
use crate::repositories::Pool;
use crate::repositories::user::{UserRepo};
use crate::services::article::ArticleService;
use crate::services::auth::AuthService;
use crate::services::user::UserService;

#[derive(Clone)]
pub struct Factory {
    pool: Box<Pool>,
    secrets: Secrets
}

impl Factory {
    pub fn new(pool: Box<Pool>, secrets: Secrets) -> Self {
        Self { pool, secrets }
    }

    pub fn article_service(&self) -> ArticleService<ArticleRepo> {
        ArticleService { repo: self.article_repo() }
    }

    pub fn auth_service(&self) -> AuthService<UserRepo> {
        AuthService {
            user_repo: self.user_repo(),
            secrets: self.secrets.clone()
        }
    }

    pub fn user_service(&self) -> UserService<UserRepo> {
        UserService { repo: self.user_repo() }
    }

    fn article_repo(&self) -> ArticleRepo {
        ArticleRepo { pool: self.pool.clone() }
    }

    fn user_repo(&self) -> UserRepo {
        UserRepo { pool: self.pool.clone() }
    }
}
