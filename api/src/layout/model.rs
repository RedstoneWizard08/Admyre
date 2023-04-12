use std::future::Future;

use crate::{
    artifact::model::Artifact,
    repository::model::{Repository, RepositoryQueryBuilder},
};

pub type RepositoryRetriever<T, Q> =
    fn(path: String, repo: Repository<T, Q>) -> dyn Future<Output = Box<dyn Artifact<T>>>;

pub struct RepositoryRoute<T, Q>
where
    Q: RepositoryQueryBuilder,
{
    pub path: String,
    pub retrieve: RepositoryRetriever<T, Q>,
}

pub struct RepositoryLayout<T, Q>
where
    Q: RepositoryQueryBuilder,
{
    pub routes: Vec<RepositoryRoute<T, Q>>,
}
