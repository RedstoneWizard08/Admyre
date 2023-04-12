use std::str::FromStr;

use axum::http::{HeaderName, HeaderValue, Response};

use super::model::Artifact;

#[derive(Debug, Clone)]
pub struct MavenArtifact {
    pub jar: Vec<u8>,
    pub name: String,
    pub version: String,
    pub timestamp: u64,
}

impl Artifact<Vec<u8>> for MavenArtifact {
    fn get_data(&self) -> Vec<u8> {
        return self.jar.clone();
    }

    fn into_response(&self) -> Response<Vec<u8>> {
        let mut res = Response::new(self.jar.clone());

        res.headers_mut().append(
            HeaderName::from_str("Content-Type").unwrap(),
            HeaderValue::from_str("application/java-archive").unwrap(),
        );

        res.headers_mut().append(
            HeaderName::from_str("Content-Disposition").unwrap(),
            HeaderValue::from_str(
                format!("attachment; filename=\"{}\"", self.name.clone()).as_str(),
            )
            .unwrap(),
        );

        res.headers_mut().append(
            HeaderName::from_str("Content-Length").unwrap(),
            HeaderValue::from_str(self.jar.len().to_string().as_str()).unwrap(),
        );

        return res;
    }
}
