use chrono::ParseError as ChronoParseError;
use htmlescape::DecodeErr;
use scraper::Selector;
use serde_json::Error as SerdeJsonError;

#[derive(Debug)]
pub enum InternalParseError {
  UnknownError(&'static str),
  HtmlStructureError(HtmlStructureError),
  JsonParseError(JsonParseError),
  InvalidDateTime(ChronoParseError),
  HtmlDecodeErr(DecodeErr),
}

#[derive(Debug)]
pub enum JsonParseErrorCause {
  InvalidLdJson,
  InvalidDataAttrJson,
}

#[derive(Debug)]
pub struct JsonParseError {
  pub cause: JsonParseErrorCause,
  pub serde_error: SerdeJsonError,
}

#[derive(Debug)]
pub enum HtmlErrorCause {
  MissingElement,
  NonUniqueElement,
  MissingAttribute(String),
}

#[derive(Debug)]
pub struct HtmlStructureError {
  pub cause: HtmlErrorCause,
  pub selector: &'static Selector,
}

impl InternalParseError {
  pub fn html(error: HtmlStructureError) -> InternalParseError {
    InternalParseError::HtmlStructureError(error)
  }

  pub fn json(error: JsonParseError) -> InternalParseError {
    InternalParseError::JsonParseError(error)
  }

  pub fn ld_json(serde_error: SerdeJsonError) -> InternalParseError {
    InternalParseError::JsonParseError(JsonParseError {
      cause: JsonParseErrorCause::InvalidLdJson,
      serde_error,
    })
  }

  pub fn json_data_attr(serde_error: SerdeJsonError) -> InternalParseError {
    InternalParseError::JsonParseError(JsonParseError {
      cause: JsonParseErrorCause::InvalidDataAttrJson,
      serde_error,
    })
  }

  pub fn unknown(s: &'static str) -> InternalParseError {
    InternalParseError::UnknownError(s)
  }

  pub fn chrono(err: ChronoParseError) -> InternalParseError {
    InternalParseError::InvalidDateTime(err)
  }

  pub fn decode_html(err: DecodeErr) -> InternalParseError {
    InternalParseError::HtmlDecodeErr(err)
  }
}

impl From<ChronoParseError> for InternalParseError {
  fn from(err: ChronoParseError) -> Self {
    InternalParseError::chrono(err)
  }
}

impl From<DecodeErr> for InternalParseError {
  fn from(err: DecodeErr) -> Self {
    InternalParseError::decode_html(err)
  }
}
