use crate::errors::InternalParseError;
use crate::root_structs::{PageMeta, ParsedPage, Publisher};
use scraper::Html;

pub mod publishers;

use publishers::{guardian, independent, mirror};

pub fn parse_article_html(
  publisher: impl Into<Publisher>,
  url: String,
  html: &str,
) -> Result<ParsedPage, InternalParseError> {
  let document = Html::parse_document(html);
  let date_parsed = chrono::offset::Utc::now();

  let meta = PageMeta {
    publisher: publisher.into(),
    url,
    date_parsed,
  };

  let content = match meta.publisher {
    Publisher::Independent => independent::parse(document),
    Publisher::Mirror => mirror::parse(document),
    Publisher::Guardian => guardian::parse(document),
    _ => unimplemented!(),
  }?;

  Ok(ParsedPage { meta, content })
}
