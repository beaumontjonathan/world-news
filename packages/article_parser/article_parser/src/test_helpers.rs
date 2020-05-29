#[cfg(test)]
pub mod helpers {
  use scraper::Html;
  use std::path::PathBuf;

  // TODO - make this a macro
  pub fn read_html_doc(file_name: &str, std_file: &str) -> Html {
    let this_file = std_file;
    let mut file = PathBuf::from(this_file);
    file.pop();
    file.push("test-articles");
    file.push(file_name);

    let contents = std::fs::read(file).expect("test file should exist");

    let html = std::str::from_utf8(&contents).expect("test file should be utf8");

    Html::parse_document(html)
  }
}
