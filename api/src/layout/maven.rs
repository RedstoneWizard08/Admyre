// [root]
// + [group]
// ++ maven-metadata.xml
// ++ maven-metadata.xml.md5
// ++ maven-metadata.xml.sha1
// ++ [artifact]
// +++ maven-metadata.xml
// +++ maven-metadata.xml.md5
// +++ maven-metadata.xml.sha1
// +++ [version]
// ++++ [artifact]-[version].pom
// ++++ [artifact]-[version].pom.asc
// ++++ [artifact]-[version].pom.md5
// ++++ [artifact]-[version].pom.sha1
// ++++ [artifact]-[version].[extension]
// ++++ [artifact]-[version].[extension].asc
// ++++ [artifact]-[version].[extension].md5
// ++++ [artifact]-[version].[extension].sha1
// ++++ [artifact]-[version]-[classifier].[extension]
// ++++ [artifact]-[version]-[classifier].[extension].asc
// ++++ [artifact]-[version]-[classifier].[extension].md5
// ++++ [artifact]-[version]-[classifier].[extension].sha1

use liquid::{object, ParserBuilder};
use serde::{Deserialize, Serialize};

use crate::{artifact::maven::MavenArtifact, maven::MAVEN_INDEX_PAGE};

pub struct FileInfo {
    pub name: String,
    pub content: Vec<u8>,
}

pub enum FileKind {
    Maven(MavenArtifact),
    Basic(FileInfo),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
    pub path: String,
    pub name: String,
}

pub fn render_index(path: String, files: Vec<FileKind>) -> String {
    let mut all: Vec<File> = Vec::new();

    for file in files {
        match file {
            FileKind::Basic(data) => {
                all.push(File {
                    path: data.name.clone(),
                    name: data.name.to_string(),
                });
            }

            FileKind::Maven(data) => {
                all.push(File {
                    path: data.name.clone(),
                    name: data.name.to_string(),
                });
            }
        };
    }

    let globals = object!({
        "path": path,
        "files": all,
    });

    let template = ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(MAVEN_INDEX_PAGE)
        .unwrap();

    return template.render(&globals).unwrap();
}

pub fn make_test_data() -> Vec<FileKind> {
    return vec![
        FileKind::Basic(FileInfo {
            content: vec![],
            name: "maven-metadata.xml".to_string(),
        }),
        FileKind::Basic(FileInfo {
            content: vec![],
            name: "maven-metadata.xml.md5".to_string(),
        }),
        FileKind::Basic(FileInfo {
            content: vec![],
            name: "maven-metadata.xml.sha1".to_string(),
        }),
    ];
}
