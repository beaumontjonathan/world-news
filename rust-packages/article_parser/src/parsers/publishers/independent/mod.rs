use crate::errors::InternalParseError;
use crate::helpers::{
  AttrValueOrNone, FullFixedHtmlText, JoinWithNewline, ParseResult, SelectHelpers,
};
use crate::my_selectors;
use crate::root_structs::{Image, NewsArticleContent, ParsedPageContent, Video};
use chrono::DateTime;
use htmlescape::decode_html;
use scraper::{Html, Selector};

my_selectors! {
  ARTICLE_SELECTOR => "article",
  HEADLINE_SELECTOR => ".headline",
  TWITTER_TITLE_SELECTOR => "meta[name=\"twitter:title\"]",
  DESCRIPTION_SELECTOR => ".sub-headline",
  HERO_IMAGE_SELECTOR => ".hero-image",
  IMAGE_WRAPPER_CHILD_SELECTOR => ":scope > .image-wrapper",
  META_OG_IMAGE => "meta[property=\"og:image\"]",
  SECTION_SELECTOR => "meta[name=\"article:section\"]",
  SUBSECTION_SELECTOR => "meta[name=\"article:subsection\"]",
  STORY_BODY_SELECTOR => "div.main-content > div.body-content",
  STORY_BODY_IMAGE_SELECTOR => ":scope > figure:not(.i-gallery):not(.video)",
  AMP_IMG_SELECTOR => "amp-img",
  FIGCAPTION_SELECTOR => "figcaption",
  FIGCAPTION_CAPTION_SELECTOR => "figcaption.caption",
  FIGURE_VIDEO_SELECTOR => ":scope > figure.video",
  P_SELECTOR => ":scope > p",
  DATE_UPDATED => "meta[property=\"article:modified_time\"]",
  DATE_PUBLISHED => "meta[property=\"article:published_time\"]",
}

const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%z";

pub fn parse(document: Html) -> ParseResult {
  // DOCUMENTS
  let story_body = document.select_unique(&STORY_BODY_SELECTOR)?;

  // HEADLINE
  let headline = document
    .select_unique(&HEADLINE_SELECTOR)?
    .full_fixed_html_text();

  // TWITTER HEADLINE
  let twitter_headline = document
    .select_unique_attr(&TWITTER_TITLE_SELECTOR, "content")
    .ok();

  // DESCRIPTION
  let description =
    document
      .select_unique(&DESCRIPTION_SELECTOR)
      .map_or(Ok(String::new()), |el| {
        decode_html(&el.full_fixed_html_text())
          .map_err(|_| InternalParseError::unknown("Unknown error: html parsing"))
      })?;

  // THUMBNAIL
  let thumbnail = {
    if let Some(hero_image) = document.select(&HERO_IMAGE_SELECTOR).next() {
      let hero_image_or_optional_wrapper = hero_image
        .select(&IMAGE_WRAPPER_CHILD_SELECTOR)
        .next()
        .unwrap_or(hero_image);

      let amp_img = hero_image_or_optional_wrapper.select_unique(&AMP_IMG_SELECTOR)?;

      let alt = amp_img.attr_value_or_none("alt");
      let url = amp_img.attr_value_or_none("src");
      let caption = amp_img.attr_value_or_none("title");

      Image { alt, url, caption }
    } else {
      let url = document.select_unique_attr(&META_OG_IMAGE, "content").ok();
      Image {
        alt: None,
        url,
        caption: None,
      }
    }
  };

  // CATEGORIES
  let section_category = document
    .select_unique_attr(&SECTION_SELECTOR, "content")
    .ok()
    .filter(|x| !x.is_empty());
  let subsection_categories = document
    .select_unique_attr(&SUBSECTION_SELECTOR, "content")
    .ok()
    .filter(|x| !x.is_empty());
  let categories = match (section_category, subsection_categories) {
    (Some(category), Some(sub_categories)) => format!("{}, {}", category, sub_categories)
      .split(", ")
      .map(|s| s.to_owned())
      .collect(),
    (Some(category), None) => vec![category],
    (None, Some(sub_categories)) => sub_categories.split(", ").map(|s| s.to_owned()).collect(),
    (None, None) => Vec::new(),
  };

  // IMAGES
  let images = story_body
    .select(&STORY_BODY_IMAGE_SELECTOR)
    // here are some things
    .filter_map(|figure| {
      figure
        .select_unique(&AMP_IMG_SELECTOR)
        .map(|img| {
          let url = img.value().attr("src").map(|s| s.to_owned());
          let caption = figure
            .select(&FIGCAPTION_SELECTOR)
            .next()
            .map(|e| e.full_fixed_html_text());
          Image {
            alt: None,
            url,
            caption,
          }
        })
        .ok()
    })
    .collect::<Vec<_>>();

  // VIDEOS
  let videos = story_body
    .select(&FIGURE_VIDEO_SELECTOR)
    .map(|el| {
      let caption = Some(
        el.select_unique(&FIGCAPTION_CAPTION_SELECTOR)?
          .full_fixed_html_text(),
      );
      Ok(Video {
        alt: None,
        url: None,
        caption,
      })
    })
    .collect::<Result<Vec<_>, InternalParseError>>()?;

  // BODY
  let body = story_body
    .select(&P_SELECTOR)
    .map(|p| p.full_fixed_html_text())
    .join_with_newline();

  // DATE UPDATED
  let date_updated = document.select_unique_attr(&DATE_UPDATED, "content")?;
  let date_updated = DateTime::parse_from_str(&date_updated, FORMAT)?.into();

  //DATE PUBLISHED
  let date_published = document.select_unique_attr(&DATE_PUBLISHED, "content")?;
  let date_published = DateTime::parse_from_str(&date_published, FORMAT)?.into();

  // Parsed::Err("I didn't bother to do this.".to_owned())
  Ok(ParsedPageContent::NewsArticle(NewsArticleContent {
    headline,
    twitter_headline,
    description,
    thumbnail,
    categories,
    images,
    videos,
    body,
    date_updated,
    date_published,
  }))
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::test_helpers::helpers::read_html_doc;
  use insta::assert_debug_snapshot;

  #[test]
  fn test_file_01() {
    let document = read_html_doc("test-01.html", std::file!());
    let article = parse(document);
    assert_debug_snapshot!(article);
  }

  #[test]
  fn test_file_02() {
    let document = read_html_doc("test-02.html", std::file!());
    let article = parse(document);
    assert_debug_snapshot!(article);
  }

  #[test]
  fn test_file_03() {
    let document = read_html_doc("test-03.html", std::file!());
    let article = parse(document);
    assert_debug_snapshot!(article);
  }

  #[test]
  fn test_file_04() {
    let document = read_html_doc("test-04.html", std::file!());
    let article = parse(document);
    assert_debug_snapshot!(article);
  }

  #[test]
  fn test_file_05() {
    let document = read_html_doc("test-05.html", std::file!());
    let article = parse(document);
    assert_debug_snapshot!(article);
  }
}
