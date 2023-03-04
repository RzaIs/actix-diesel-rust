use diesel::{QueryDsl, RunQueryDsl};
use diesel::{prelude::*};
use crate::entities::article::{Article, CreateArticle};
use crate::helpers::error::Error;
use crate::repositories::{Conn, Pool};

pub struct ArticleRepo {
    pub pool: Box<Pool>
}

pub trait ArticleRepoTrait {
    fn get_all(&self) -> Result<Vec<Article>, Error>;
    fn create_article(&self, data: CreateArticle) -> Result<Article, Error>;
    fn get_by_id(&self, id: i32) -> Result<Article, Error>;
    fn update_article(&self, id: i32, data: CreateArticle) -> Result<Article, Error>;
    fn delete_article(&self, id: i32) -> Result<Article, Error>;
}

impl ArticleRepoTrait for ArticleRepo {
    fn get_all(&self) -> Result<Vec<Article>, Error> {
        use crate::schema::article::dsl::article;

        let conn: &mut Conn = &mut self.pool.get()?;

        let articles = article.load::<Article>(conn)?;

        Ok(articles)
    }

    fn create_article(&self, data: CreateArticle) -> Result<Article, Error> {
        use crate::schema::article;

        let conn: &mut Conn = &mut self.pool.get()?;

        let new_article = diesel::insert_into(article::table)
            .values(&data)
            .get_result::<Article>(conn)?;

        Ok(new_article)
    }

    fn get_by_id(&self, id: i32) -> Result<Article, Error> {
        use crate::schema::article::dsl::article;

        let conn: &mut Conn = &mut self.pool.get()?;

        let art = article.find(id)
            .first::<Article>(conn)?;

        Ok(art)
    }

    fn update_article(&self, id: i32, data: CreateArticle) -> Result<Article, Error> {
        use crate::schema::article::dsl::{article, title, content};

        let conn: &mut Conn = &mut self.pool.get()?;

        let art: Article = diesel::update(article.find(id))
            .set((
                title.eq(data.title),
                content.eq(data.content)
            ))
            .get_result::<Article>(conn)?;

        Ok(art)
    }

    fn delete_article(&self, id: i32) -> Result<Article, Error> {
        use crate::schema::article::dsl::article;

        let conn: &mut Conn = &mut self.pool.get()?;

        let art: Article = diesel::delete(article.find(id))
            .get_result::<Article>(conn)?;

        Ok(art)
    }
}
