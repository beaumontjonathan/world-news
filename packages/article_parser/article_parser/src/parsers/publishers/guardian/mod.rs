use crate::errors::{HtmlErrorCause, HtmlStructureError, InternalParseError};
use crate::helpers::{
  AttrValueOrNone, ChildrenHelpers, FullFixedHtmlText, JoinWithNewline, ParseResult, SelectHelpers,
};
use crate::root_structs::{Image, NewsArticleContent, ParsedPageContent, Video};

use crate::my_selectors;
use chrono::{DateTime, Utc};
use scraper::{Html, Selector};

my_selectors! {
  ARTICLE_VIDEO_PRECHECK => "body > div > article[id=\"article\"][itemtype=\"http://schema.org/VideoObject\"].content--media--video",
  TWITTER_TITLE_SELECTOR => "head > meta[name=\"twitter:text:title\"]",
  DATE_UPDATED_SELECTOR => "meta[property=\"article:modified_time\"]",
  DATE_PUBLISHED_SELECTOR => "meta[property=\"article:published_time\"]",
  HEADER => "div.content__main-column > header",
  HEADLINE => "h1.content__headline",
  DESCRIPTION => "div.content__standfirst",
  FIGURE_HEADER => "figure.media-primary",
  IMG => "img",
  FIGCAPTION => "figcaption.caption",
  ARTICLE_CONTENT => "div.content__article-body",
  ELEMENT_IMAGE => "figure.element-image",
  ELEMENT_INTERACTIVE => "figure.element-interactive",
  VIDEO_FIGURE => "figure.element[data-atom-type=\"media\"]",
  YOUTUBE_IFRAME => "div.youtube-media-atom__iframe",
}

type Result<T> = std::result::Result<T, InternalParseError>;

pub fn parse(document: Html) -> ParseResult {
  // PRECHECKS
  match document.select(&ARTICLE_VIDEO_PRECHECK).next() {
    None => Ok(()),
    Some(_) => Err(InternalParseError::unknown(
      "Article type: video article, not yet supported",
    )),
    // Some(_) => Err(InternalParseError::precondition(
    //   "Article type: video article, not yet supported",
    // )),
  }?;

  let header = document.select_unique(&HEADER)?;
  let article_content = document.select_unique(&ARTICLE_CONTENT)?;

  // HEADLINE
  let headline = {
    let h1s = header
      .select(&HEADLINE)
      .map(|h1| h1.full_fixed_html_text())
      .collect::<Vec<_>>();
    match h1s.len() {
      0 => Err(InternalParseError::html(HtmlStructureError {
        cause: HtmlErrorCause::MissingElement,
        selector: &HEADLINE,
      })),
      1 => Ok(h1s.into_iter().next().unwrap()),
      _ => {
        if h1s.windows(2).all(|w| w[0] == w[1]) {
          Ok(h1s.into_iter().next().unwrap())
        } else {
          Err(InternalParseError::unknown(
            "Unknown headline type - too many <h1>s with differing text",
          ))
        }
      }
    }
  }?;

  // TWITTER HEADLINE
  let twitter_headline = document
    .select_unique_attr(&TWITTER_TITLE_SELECTOR, "content")
    .ok();

  // DESCRIPTION
  let description = header.select_unique(&DESCRIPTION)?.full_fixed_html_text();

  // THUMBNAIL
  // todo - implement
  let thumbnail = {
    let figure = header.select_unique(&FIGURE_HEADER)?;
    let img = figure.select_unique(&IMG)?;
    let alt = img.attr_value_or_none("alt");
    let url = img.attr_value_or_none("src");
    let caption = Some(figure.select_unique(&FIGCAPTION)?.full_fixed_html_text());
    Image { alt, url, caption }
  };

  // CATEGORIES
  // TODO - implement
  let categories: Vec<String> = vec![];

  // IMAGES
  let images = article_content
    .children_all()
    .into_iter()
    .filter(|el| ELEMENT_IMAGE.matches(&el) || ELEMENT_INTERACTIVE.matches(&el))
    .map(|figure| {
      if ELEMENT_IMAGE.matches(&figure) {
        let img = figure.select_unique(&IMG)?;
        let alt = img.attr_value_or_none("alt");
        let url = img.attr_value_or_none("src");
        let caption = Some(figure.select_unique(&FIGCAPTION)?.full_fixed_html_text());
        Ok(Image { alt, url, caption })
      } else if ELEMENT_INTERACTIVE.matches(&figure) {
        // unimplemented!();
        Ok(Image {
          alt: None,
          url: None,
          caption: None,
        })
      } else {
        panic!("how are you here?");
      }
    })
    .collect::<Result<Vec<_>>>()?;

  // VIDEOS
  let videos = article_content
    .children_all()
    .into_iter()
    .filter(|figure| VIDEO_FIGURE.matches(&figure))
    .map(|figure| {
      let caption = Some(figure.select_unique(&FIGCAPTION)?.full_fixed_html_text());
      let youtube_id = figure.select_unique_attr(&YOUTUBE_IFRAME, "data-asset-id")?;
      let url = Some(format!("https://www.youtube.com/watch?v={}", youtube_id));

      Ok(Video {
        alt: None,
        url,
        caption,
      })
    })
    .collect::<Result<Vec<_>>>()?;

  // BODY
  let body = article_content
    .children_all()
    .into_iter()
    .filter_map(|el| {
      if el.value().name() == "p" || el.value().name() == "ul" {
        Some(el.full_fixed_html_text())
      } else {
        None
      }
    })
    .collect::<Vec<String>>()
    .into_iter()
    .join_with_newline();

  // DATE_UPDATED
  let date_updated = document.select_unique_attr(&DATE_UPDATED_SELECTOR, "content")?;
  let date_updated = DateTime::parse_from_rfc3339(&date_updated)?.with_timezone(&Utc);

  //DATE_PUBLISHED
  let date_published = document.select_unique_attr(&DATE_PUBLISHED_SELECTOR, "content")?;
  let date_published = DateTime::parse_from_rfc3339(&date_published)?.with_timezone(&Utc);

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

  // todo - error nicely?
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

  #[test]
  fn test_file_08() {
    let document = read_html_doc("test-08.html", std::file!());
    let article = parse(document);
    assert_debug_snapshot!(article);
  }
}
