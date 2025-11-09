use std::fmt::Display;
use songbird::input::AuxMetadata;

#[derive(Clone)]
pub struct MetadataObject {
    title: String,
    source_url: String,
    channel: String,
}

impl MetadataObject {
    pub fn title(&self) -> &str { &self.title }
    pub fn source_url(&self) -> &str { &self.source_url }
    pub fn channel(&self) -> &str { &self.channel }
}

impl From<AuxMetadata> for MetadataObject {
    fn from(metadata: AuxMetadata) -> Self {
        Self {
            title: metadata.title.unwrap_or("Unknown title".to_string()),
            source_url: metadata.source_url.unwrap_or("Unknown source".to_string()),
            channel: metadata.channel.unwrap_or("Unknown channel".to_string()),
        }
    }
}

impl Display for MetadataObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[**\"{}\"**]({}) from **{}**.",
            self.title, self.source_url, self.channel)
    }
}