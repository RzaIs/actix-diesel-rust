use diesel::{QueryDsl, RunQueryDsl};
use diesel::{prelude::*};
use crate::entities::user::{CreateUser, User};
use crate::helpers::error::Error;
use crate::repositories::{Conn, Pool};

pub struct UserRepo {
    pub pool: Box<Pool>
}

pub trait UserRepoTrait {
    fn get_user_by_id(&self, id: i32) -> Result<User, Error>;
    fn get_user_by_username(&self, uname: String) -> Result<User, Error>;
    fn create_user(&self, data: CreateUser) -> Result<User, Error>;
}

impl UserRepoTrait for UserRepo {
    fn get_user_by_id(&self, id: i32) -> Result<User, Error> {
        use crate::schema::app_user::dsl::app_user;

        let conn: &mut Conn = &mut self.pool.get()?;

        let user: User = app_user.find(id)
            .first::<User>(conn)?;

        Ok(user)
    }

    fn get_user_by_username(&self, uname: String) -> Result<User, Error> {
        use crate::schema::app_user::dsl::{app_user, username};

        let conn: &mut Conn = &mut self.pool.get()?;

        let user: User = app_user.filter(username.eq(uname))
            .first(conn)?;

        Ok(user)
    }

    fn create_user(&self, data: CreateUser) -> Result<User, Error> {
        use crate::schema::app_user;

        let conn: &mut Conn = &mut self.pool.get()?;

        let user = diesel::insert_into(app_user::table)
            .values(&data)
            .get_result::<User>(conn)?;

        Ok(user)
    }
}
