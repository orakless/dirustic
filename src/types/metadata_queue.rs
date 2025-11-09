use std::fmt::Display;
use serenity::all::{CreateEmbed, CreateEmbedFooter};
use songbird::input::AuxMetadata;

#[derive(Clone)]
pub struct MetadataObject {
    title: String,
    source_url: String,
    channel: String,
    thumbnail_url: String,
}

impl MetadataObject {
    pub fn title(&self) -> &str { &self.title }
    pub fn source_url(&self) -> &str { &self.source_url }
    pub fn channel(&self) -> &str { &self.channel }
    pub fn thumbnail_url(&self) -> &str { &self.thumbnail_url }
    pub fn to_embed(&self) -> CreateEmbed {
        CreateEmbed::default()
            .title(self.title.clone())
            .thumbnail(self.thumbnail_url.to_string())
            .description(self.channel.clone())
            .url(self.source_url.clone())
    }
}

impl From<AuxMetadata> for MetadataObject {
    fn from(metadata: AuxMetadata) -> Self {
        Self {
            title: metadata.title.unwrap_or("Unknown title".to_string()),
            source_url: metadata.source_url.unwrap_or("Unknown source".to_string()),
            channel: metadata.channel.unwrap_or("Unknown channel".to_string()),
            thumbnail_url: metadata.thumbnail.unwrap_or("".to_string()),
        }
    }
}


impl Display for MetadataObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[**\"{}\"**]({}) from **{}**.",
            self.title, self.source_url, self.channel)
    }
}