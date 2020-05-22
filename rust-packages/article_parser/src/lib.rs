#[macro_use]
mod macros;
mod errors;
mod helpers;
mod parsers;
mod root_structs;
#[cfg(test)]
mod test_helpers;

pub use parsers::parse_article_html;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
