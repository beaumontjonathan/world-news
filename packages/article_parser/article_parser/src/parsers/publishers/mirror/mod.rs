use crate::errors::InternalParseError;
use crate::helpers::{
  AttrValueOrNone, FullFixedHtmlText, JoinWithNewline, ParseResult, SelectHelpers,
};
use crate::my_selectors;
use crate::root_structs::{Image, NewsArticleContent, ParsedPageContent, Video};
use chrono::DateTime;
use scraper::{Html, Selector};

my_selectors! {
  ARTICLE_ROOT_SELECTOR => "body > main > article.article-main",
  ARTICLE_CONTENT_SELECTOR => ":scope > div.article-wrapper > div.content-column",
  ARTICLE_BODY_SELECTOR => ":scope > div.article-body",
  ARTICLE_CONTENT_CHILDREN_SELECTOR => ":scope > *",
  H1_SELECTOR => "h1",
  SUB_TITLE_SELECTOR => "p.sub-title",
  THUMBNAIL_FIGURE_SELECTOR => "figure.lead-article-image",
  IMG_SELECTOR => "img",
  CAPTION_SELECTOR => "figcaption > .caption",
  URL_SELECTOR => "meta[itemprop=\"url\"]",
  ARTICLE_BODY_FIGURE_IMAGE_SELECTOR => ".article-body > figure.in-article-image",
  ARTICLE_BODY_P_AND_URL_SELECTOR => ":scope > p, :scope > ul",
  DATE_UPDATED_SELECTOR => "head > meta[property=\"article:modified_time\"]",
  DATE_PUBLISHED_SELECTOR => "head > meta[property=\"article:published_time\"]",
  DIV_MOD_VIDEO => "div.mod-video",
  FIGURE_LEAD_ARTICLE_IMAGE => "figure.lead-article-image",
  FIGURE_IN_ARTICLE_IMAGE => ":scope > figure.in-article-image",
}

pub fn parse(document: Html) -> ParseResult {
  // DOCUMENT SELECTED
  let article_root = document.select_unique(&ARTICLE_ROOT_SELECTOR)?;
  let article_content = article_root.select_unique(&ARTICLE_CONTENT_SELECTOR)?;
  let article_body = article_content.select_unique(&ARTICLE_BODY_SELECTOR)?;

  // HEADLINE
  let headline = article_root
    .select_unique(&H1_SELECTOR)?
    .full_fixed_html_text();

  // TWITTER HEADLINE
  let twitter_headline = None;

  // DESCRIPTION
  let description: String = article_root
    .select_unique(&SUB_TITLE_SELECTOR)
    .map(|sub_title| sub_title.full_fixed_html_text())
    .unwrap_or_else(|_| String::new());

  // THUMBNAIL
  let thumbnail: Image = {
    let first_child = article_content.select_first(&ARTICLE_CONTENT_CHILDREN_SELECTOR)?;
    if DIV_MOD_VIDEO.matches(&first_child) {
      // todo - could get data from the "div.json-placeholder" element, but
      // html escaping breaks the json - e.g., double quotes in headline text.
      Ok(Image {
        alt: None,
        url: None,
        caption: None,
      })
    } else if FIGURE_LEAD_ARTICLE_IMAGE.matches(&first_child) {
      let caption = Some(
        first_child
          .select_unique(&CAPTION_SELECTOR)?
          .full_fixed_html_text(),
      );
      let img = first_child.select_unique(&IMG_SELECTOR)?;
      let url = img.attr_value_or_none("src");
      let alt = img.attr_value_or_none("alt");
      Ok(Image { alt, url, caption })
    } else {
      Err(InternalParseError::unknown("thumbnail type unknown"))
    }
  }?;

  // CATEGORIES
  // TODO - implement
  let categories: Vec<String> = vec![];

  // IMAGES
  let images = article_body
    .select(&FIGURE_IN_ARTICLE_IMAGE)
    .map(|figure| {
      let img = figure.select_first(&IMG_SELECTOR)?;
      let alt = img.value().attr("alt").and_then(|s| match s.len() {
        0 => None,
        _ => Some(s.to_owned()),
      });
      let url = figure
        .select_unique(&URL_SELECTOR)?
        .attr_value_or_none("content");
      let caption = figure
        .select(&CAPTION_SELECTOR)
        .next()
        .map(|el| el.full_fixed_html_text());
      Ok(Image { url, alt, caption })
    })
    .collect::<Result<_, InternalParseError>>()?;

  // VIDEOS
  // todo - implement
  let videos: Vec<Video> = vec![];

  // BODY
  let body = article_body
    .select(&ARTICLE_BODY_P_AND_URL_SELECTOR)
    .map(|el| el.full_fixed_html_text())
    .join_with_newline();

  // DATE UPDATED
  let date_updated = document.select_unique_attr(&DATE_UPDATED_SELECTOR, "content")?;
  let date_updated = DateTime::parse_from_rfc3339(&date_updated)?.into();

  //DATE PUBLISHED
  let date_published = document.select_unique_attr(&DATE_PUBLISHED_SELECTOR, "content")?;
  let date_published = DateTime::parse_from_rfc3339(&date_published)?.into();

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
  fn test_file_06() {
    let document = read_html_doc("test-06.html", std::file!());
    let article = parse(document);
    assert_debug_snapshot!(article);
  }

  #[test]
  fn test_file_07() {
    let document = read_html_doc("test-07.html", std::file!());
    let article = parse(document);
    assert_debug_snapshot!(article);
  }
}
