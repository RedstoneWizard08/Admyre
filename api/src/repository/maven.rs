use std::{error::Error, time::Duration};

use async_trait::async_trait;

use crate::{
    artifact::maven::MavenArtifact, maven::metadata::generate_metadata,
    xml::maven::serialize_maven_metadata,
};

use super::model::{Repository, RepositoryGenerator, RepositoryQueryBuilder};

#[derive(Debug, Clone)]
pub struct MavenQueryBuilder {
    pub group: Option<String>,
    pub artifact: Option<String>,
    pub version: Option<String>,
}

impl MavenQueryBuilder {
    pub fn group(&mut self, group: String) -> &mut Self {
        self.group = Some(group);

        return self;
    }

    pub fn artifact(&mut self, artifact: String) -> &mut Self {
        self.artifact = Some(artifact);

        return self;
    }

    pub fn version(&mut self, version: String) -> &mut Self {
        self.version = Some(version);

        return self;
    }

    pub fn build(&self) -> MavenQuery {
        return MavenQuery {
            builder: self.clone(),
        };
    }
}

impl RepositoryQueryBuilder for MavenQueryBuilder {
    fn new() -> Self {
        return Self {
            group: None,
            artifact: None,
            version: None,
        };
    }
}

pub struct MavenQuery {
    pub builder: MavenQueryBuilder,
}

impl MavenQuery {
    pub async fn find_artifact(&self) -> Result<MavenArtifact, Box<dyn Error>> {
        todo!();
    }

    pub async fn find_artifacts(&self) -> Result<Vec<MavenArtifact>, Box<dyn Error>> {
        todo!();
    }

    pub async fn find_groups(&self) -> Result<Vec<String>, Box<dyn Error>> {
        todo!();
    }
}

pub struct MetadataGenerator;

#[async_trait]
impl RepositoryGenerator<Vec<u8>, MavenQueryBuilder, String> for MetadataGenerator {
    async fn generate(
        &self,
        repo: Repository<Vec<u8>, MavenQueryBuilder>,
        params: String,
    ) -> String {
        let mut params_split = params.split("::");

        let group = params_split.nth(0).unwrap().to_string();
        let artifact = params_split.nth(1).unwrap().to_string();

        let artifacts = repo
            .query()
            .group(group.clone())
            .artifact(artifact.clone())
            .build()
            .find_artifacts()
            .await
            .unwrap();

        let versions: Vec<&str> = artifacts.iter().map(|v| v.version.as_str()).collect();
        let mut latest_finder = artifacts.clone();

        latest_finder.sort_by(|a, b| {
            Duration::from_secs(a.timestamp).cmp(&Duration::from_secs(b.timestamp))
        });

        let latest = latest_finder.last().unwrap().clone();

        let metadata = generate_metadata(&group, &artifact, &latest.version, versions);

        return serialize_maven_metadata(metadata);
    }
}
