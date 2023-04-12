use crate::maven::metadata::Metadata;

pub fn serialize_maven_metadata(metadata: Metadata) -> String {
    let mut data = String::new();

    // Head
    data.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    data.push_str("<metadata>\n");

    // Group
    data.push_str("    <groupId>");
    data.push_str(&metadata.group_id);
    data.push_str("</groupId>\n");

    // Artifact
    data.push_str("    <artifactId>");
    data.push_str(&metadata.artifact_id);
    data.push_str("</artifactId>\n");

    // Versioning
    data.push_str("    <versioning>\n");

    // Latest
    data.push_str("        <latest>");
    data.push_str(&metadata.versioning.latest);
    data.push_str("</latest>\n");

    // Release
    data.push_str("        <release>");
    data.push_str(&metadata.versioning.release);
    data.push_str("</release>\n");

    // Versions
    data.push_str("        <versions>\n");

    // Each version
    for version in metadata.versioning.versions {
        data.push_str("            <version>");
        data.push_str(&version);
        data.push_str("</version>\n");
    }

    // Versions end
    data.push_str("        </versions>\n");

    // Last updated
    data.push_str("        <lastUpdated>");
    data.push_str(&metadata.versioning.last_updated);
    data.push_str("</lastUpdated>\n");

    // End
    data.push_str("    </versioning>\n");
    data.push_str("</metadata>\n");

    return data;
}
