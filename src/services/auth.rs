use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use crate::entities::auth::{AuthResponse, Claims, LoginDTO, RegisterDTO, Tokens};
use crate::helpers::config::Secrets;
use crate::helpers::error::Error;
use crate::repositories::user::{UserRepoTrait};

pub struct AuthService<Repo: UserRepoTrait> {
    pub user_repo: Repo,
    pub secrets: Secrets
}

impl<Repo: UserRepoTrait> AuthService<Repo> {
    pub fn register(&self, data: RegisterDTO) -> Result<AuthResponse, Error> {
        let password_hash = self.hash(&data.secret)?;

        let user = self.user_repo
            .create_user(data.to_create_user(password_hash))?;

        let tokens = self.generate_tokens(user.id, )
            .ok_or(Error::tokens())?;

        Ok(AuthResponse { user, tokens })
    }

    pub fn login(&self, data: LoginDTO) -> Result<AuthResponse, Error> {
        let password_hash = self.hash(&data.secret)?;
        let user = self.user_repo
            .get_user_by_username(data.username.clone())?;

        if self.verify(user.secret.as_bytes(), &password_hash) {
            return Err(Error::login());
        }

        let tokens = self.generate_tokens(user.id, )
            .ok_or(Error::tokens())?;

        Ok(AuthResponse { user, tokens })
    }

    fn hash(&self, secret: &String) -> Result<String, Error> {
        let salt = SaltString::new(self.secrets.argon_salt.as_str())?;

        let argon2 = Argon2::default();

        Ok(
            argon2.hash_password(
                secret.as_bytes(), &salt
            )?.to_string()
        )
    }

    fn verify(&self, pwd_bytes: &[u8], hash: &String) -> bool {
        match PasswordHash::new(hash) {
            Err(_) => false,
            Ok(parsed_hash) => {
                Argon2::default()
                    .verify_password(pwd_bytes, &parsed_hash)
                    .is_ok()
            }
        }
    }

    fn generate_access_token(&self, id: i32) -> Option<String> {
        let secret = self.secrets.jwt_access_secret.as_bytes();
        let exp = (Utc::now() + Duration::days(30)).timestamp() as usize;
        let claims = Claims { id, exp };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret)
        ).ok()
    }

    fn generate_refresh_token(&self, id: i32) -> Option<String> {
        let secret = self.secrets.jwt_refresh_secret.as_bytes();
        let exp = (Utc::now() + Duration::weeks(24)).timestamp() as usize;
        let claims = Claims { id, exp };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret)
        ).ok()
    }
    fn generate_tokens(
        &self,
        user_id: i32,
    ) -> Option<Tokens> {
        let access_token = self.generate_access_token(user_id)?;
        let refresh_token = self.generate_refresh_token(user_id)?;

        Some(Tokens {
            access_token,
            refresh_token
        })
    }
}
