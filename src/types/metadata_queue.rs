use std::fmt::Display;
use std::sync::Arc;
use serenity::all::CreateEmbed;
use songbird::input::AuxMetadata;

const ITEMS_PER_PAGE: usize = 10;

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

}

pub trait ToEmbed {
    fn to_embed(&self) -> CreateEmbed;
}

pub trait ToEmbedPageContent {
    fn page(&self, index: usize) -> String;
    fn to_paged_embed(&self) -> Vec<String>;
}

impl ToEmbed for MetadataObject {
    fn to_embed(&self) -> CreateEmbed {
        CreateEmbed::default()
            .title(self.title())
            .thumbnail(self.thumbnail_url().to_string())
            .description(self.channel())
            .url(self.source_url())
    }
}

impl ToEmbedPageContent for Vec<Arc<MetadataObject>> {
    fn page(&self, index: usize) -> String {
        let mut count = index * ITEMS_PER_PAGE;
        let ranges: (usize, usize) = (
            index*ITEMS_PER_PAGE,
            if self.len() < index*ITEMS_PER_PAGE + ITEMS_PER_PAGE {
                self.len()
            } else { index * ITEMS_PER_PAGE + ITEMS_PER_PAGE }
        );

        let mut description = String::new();
        for metadata in &self[ranges.0..ranges.1] {
            count+=1;
            description.push_str(&format!("{count}. {metadata}\n"));
        }

        description
    }
    fn to_paged_embed(&self) -> Vec<String> {
        let mut embeds: Vec<String> = Vec::new();

        let page_number = if self.len()%ITEMS_PER_PAGE > 0 { self.len() / ITEMS_PER_PAGE + 1 } else { self.len() / ITEMS_PER_PAGE };

        for page_nb in 0..page_number {
            embeds.push(self.page(page_nb));
        }

        embeds
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
            self.title(), self.source_url(), self.channel())
    }
}