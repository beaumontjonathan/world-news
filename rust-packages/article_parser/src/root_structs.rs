use chrono::{DateTime, Utc};
use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Serialize)]
pub struct Image {
  pub alt: Option<String>,
  pub url: Option<String>,
  pub caption: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Video {
  pub alt: Option<String>,
  pub url: Option<String>,
  pub caption: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub enum Publisher {
  BBC,
  DailyMail,
  Independent,
  Metro,
  Mirror,
  Sky,
  Sun,
  Guardian,
}

use Publisher::*;

impl From<String> for Publisher {
  fn from(publisher: String) -> Self {
    match publisher.to_lowercase().as_str() {
      "bbc" => BBC,
      "dailymail" => DailyMail,
      "independent" => Independent,
      "metro" => Metro,
      "mirror" => Mirror,
      "sky" => Sky,
      "sun" => Sun,
      "theguardian" | "guardian" => Guardian,
      _ => panic!("Invalid publisher type"),
    }
  }
}

#[derive(Debug, Serialize)]
pub struct NewsArticleContent {
  pub headline: String,
  pub twitter_headline: Option<String>,
  pub description: String,
  pub thumbnail: Image,
  pub categories: Vec<String>,
  pub images: Vec<Image>,
  pub videos: Vec<Video>,
  pub body: String,
  pub date_updated: DateTime<Utc>,
  pub date_published: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct VideoArticleContent {
  pub something: String,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize)]
pub enum ParsedPageContent {
  NewsArticle(NewsArticleContent),
  VideoArticle(VideoArticleContent),
}

#[derive(Debug, Serialize)]
pub struct PageMeta {
  pub publisher: Publisher,
  pub url: String,
  pub date_parsed: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ParsedPage {
  pub meta: PageMeta,
  pub content: ParsedPageContent,
}
