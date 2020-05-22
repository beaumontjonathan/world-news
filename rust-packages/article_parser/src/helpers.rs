use crate::errors::{HtmlErrorCause, HtmlStructureError, InternalParseError};
// use chrono::{DateTime, Utc};
use crate::root_structs::ParsedPageContent;
use lazy_static::lazy_static;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};
// use std::fmt::{self, Debug, Display, Formatter};
pub trait ToOwnedOrNone {
  fn to_owned_or_none(self) -> Option<String>;
}

impl<'a> ToOwnedOrNone for Option<&'a str> {
  fn to_owned_or_none(self) -> Option<String> {
    self.and_then(|mut s| {
      s = s.trim();
      if s.is_empty() {
        None
      } else {
        Some(String::from(s))
      }
    })
  }
}
// pub trait NoneIfEmpty
// where
//   Self: Sized,
// {
//   fn none_if<F>(self, callback: F) -> Option<Self>
//   where
//     F: Fn(&Self) -> bool,
//   {
//     if callback(&self) {
//       None
//     } else {
//       Some(self)
//     }
//   }
// }
// impl<T> NoneIfEmpty for T {}

pub type ParseResult = Result<ParsedPageContent, InternalParseError>;

pub trait FixHtmlWhitespace {
  fn fix_html_whitespace(self) -> Self;
}

fn fix_whitespace(text: String) -> String {
  lazy_static! {
    static ref RE: Regex = Regex::new(r#"\s+"#).unwrap();
  }
  RE.replace_all(&text, " ").into_owned()
}

impl FixHtmlWhitespace for String {
  fn fix_html_whitespace(self) -> String {
    fix_whitespace(self)
  }
}

pub trait AttrValueOrNone {
  fn attr_value_or_none(&self, attr: &str) -> Option<String>;
}

impl<'a> AttrValueOrNone for ElementRef<'a> {
  fn attr_value_or_none(&self, attr: &str) -> Option<String> {
    self.value().attr(attr).to_owned_or_none()
  }
}

pub trait FullFixedHtmlText {
  fn full_fixed_html_text(self) -> String;
}

impl<'a> FullFixedHtmlText for ElementRef<'a> {
  fn full_fixed_html_text(self) -> String {
    self
      .text()
      .map(|text| text.trim().to_string())
      .collect::<Vec<String>>()
      .join(" ")
      .fix_html_whitespace()
      .trim()
      .to_owned()
  }
}

pub trait JoinWithNewline {
  fn join_with_newline(self) -> String;
}

impl<T> JoinWithNewline for T
where
  T: Iterator<Item = String>,
{
  fn join_with_newline(self) -> String {
    self.collect::<Vec<String>>().join("\n").trim().to_owned()
  }
}

pub trait UniqueElementRef<'a> {
  fn unique(self) -> std::result::Result<ElementRef<'a>, HtmlErrorCause>;
}

impl<'a, T> UniqueElementRef<'a> for T
where
  T: Iterator<Item = ElementRef<'a>>,
{
  fn unique(mut self) -> std::result::Result<ElementRef<'a>, HtmlErrorCause> {
    match (self.next(), self.next()) {
      (None, _) => Err(HtmlErrorCause::MissingElement),
      (_, Some(_)) => Err(HtmlErrorCause::NonUniqueElement),
      (Some(el_ref), None) => Ok(el_ref),
    }
  }
}

pub trait SelectHelpers {
  fn select_unique(&self, selector: &'static Selector) -> Result<ElementRef, InternalParseError>;

  fn select_first(&self, selector: &'static Selector) -> Result<ElementRef, InternalParseError>;

  fn select_unique_attr(
    &self,
    selector: &'static Selector,
    attr: &str,
  ) -> Result<String, InternalParseError>;
}

impl SelectHelpers for Html {
  fn select_unique(&self, selector: &'static Selector) -> Result<ElementRef, InternalParseError> {
    self
      .select(selector)
      .unique()
      .map_err(|cause| InternalParseError::html(HtmlStructureError { cause, selector }))
  }

  fn select_first(&self, selector: &'static Selector) -> Result<ElementRef, InternalParseError> {
    self.select(selector).next().ok_or_else(|| {
      InternalParseError::html(HtmlStructureError {
        cause: HtmlErrorCause::MissingElement,
        selector,
      })
    })
  }

  fn select_unique_attr(
    &self,
    selector: &'static Selector,
    attr: &str,
  ) -> Result<String, InternalParseError> {
    self
      .select_unique(selector)?
      .value()
      .attr(attr)
      .map(|s| s.trim().to_owned())
      .ok_or_else(|| {
        InternalParseError::html(HtmlStructureError {
          cause: HtmlErrorCause::MissingAttribute(attr.to_owned()),
          selector,
        })
      })
  }
}

impl SelectHelpers for scraper::element_ref::ElementRef<'_> {
  fn select_unique(&self, selector: &'static Selector) -> Result<ElementRef, InternalParseError> {
    self
      .select(selector)
      .unique()
      .map_err(|cause| InternalParseError::html(HtmlStructureError { cause, selector }))
  }

  fn select_first(&self, selector: &'static Selector) -> Result<ElementRef, InternalParseError> {
    self.select(selector).next().ok_or_else(|| {
      InternalParseError::html(HtmlStructureError {
        cause: HtmlErrorCause::MissingElement,
        selector,
      })
    })
  }

  fn select_unique_attr(
    &self,
    selector: &'static Selector,
    attr: &str,
  ) -> Result<String, InternalParseError> {
    self
      .select_unique(selector)?
      .value()
      .attr(attr)
      .map(|s| s.trim().to_owned())
      .ok_or_else(|| {
        InternalParseError::html(HtmlStructureError {
          cause: HtmlErrorCause::MissingAttribute(attr.to_owned()),
          selector,
        })
      })
  }
}

pub trait FormattedText {
  fn formatted_text(self) -> String;
}

impl<'a, T> FormattedText for T
where
  T: Iterator<Item = ElementRef<'a>>,
{
  fn formatted_text(self) -> String {
    self.map(|p| p.full_fixed_html_text()).join_with_newline()
  }
}

pub trait ChildrenHelpers {
  fn children_first(&self) -> Option<ElementRef>;
  fn children_all(&self) -> Vec<ElementRef>;
}

impl ChildrenHelpers for scraper::element_ref::ElementRef<'_> {
  fn children_first(&self) -> Option<ElementRef> {
    self.children_all().into_iter().next()
  }

  fn children_all(&self) -> Vec<ElementRef> {
    self.children().filter_map(ElementRef::wrap).collect()
  }
}
