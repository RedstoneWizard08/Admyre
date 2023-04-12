use async_trait::async_trait;

use crate::layout::model::RepositoryLayout;

pub trait RepositoryQueryBuilder: Send + Sync {
    fn new() -> Self;
}

#[async_trait]
pub trait RepositoryGenerator<T, Q, G = String>: Send + Sync
where
    Q: RepositoryQueryBuilder,
{
    async fn generate(&self, repo: Repository<T, Q>, params: String) -> G;
}

pub struct Repository<T, Q>
where
    Q: RepositoryQueryBuilder,
{
    pub generators: Vec<Box<dyn RepositoryGenerator<T, Q>>>,
    pub layout: RepositoryLayout<T, Q>,
}

impl<T, Q> Repository<T, Q>
where
    Q: RepositoryQueryBuilder,
{
    pub fn query(&self) -> Q {
        return Q::new();
    }
}
