use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MavenVersioning {
    pub release: String,
    pub latest: String,
    pub versions: Vec<String>,
    pub last_updated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Metadata {
    pub group_id: String,
    pub artifact_id: String,
    pub versioning: MavenVersioning,
}

pub fn generate_metadata(
    group: &str,
    artifact: &str,
    latest: &str,
    versions: Vec<&str>,
) -> Metadata {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let versions = versions.iter().map(|v| v.to_string()).collect();

    let versioning = MavenVersioning {
        last_updated: timestamp.to_string(),
        release: latest.to_string(),
        latest: latest.to_string(),
        versions,
    };

    let metadata = Metadata {
        group_id: group.to_string(),
        artifact_id: artifact.to_string(),
        versioning,
    };

    return metadata;
}

pub fn make_example_metadata() -> Metadata {
    return generate_metadata(
        "com.example",
        "example-lib",
        "1.0.0",
        vec![
            "0.1.0", "0.2.0", "0.3.0", "0.4.0", "0.5.0", "0.6.0", "0.7.0", "0.8.0", "0.9.0",
            "0.10.0", "1.0.0",
        ],
    );
}
