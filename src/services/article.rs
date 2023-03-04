use crate::entities::article::{Article, CreateArticle};
use crate::repositories::article::{ArticleRepoTrait};
use crate::helpers::error::Error;

pub struct ArticleService<Repo: ArticleRepoTrait> {
    pub repo: Repo
}

impl<Repo: ArticleRepoTrait> ArticleService<Repo> {
    pub fn get_articles(&self) -> Result<Vec<Article>, Error> {
        self.repo.get_all()
    }

    pub fn create_article(&self, data: CreateArticle) -> Result<Article, Error> {
        self.repo.create_article(data)
    }

    pub fn get_article_by_id(&self, id: i32) -> Result<Article, Error> {
        self.repo.get_by_id(id)
    }

    pub fn update_article(&self, id: i32, data: CreateArticle) -> Result<Article, Error> {
        self.repo.update_article(id, data)
    }

    pub fn delete_article(&self, id: i32) -> Result<Article, Error> {
        self.repo.delete_article(id)
    }
}
