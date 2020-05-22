#[macro_export]
macro_rules! my_selectors(
  { $($key:ident => $value:expr),+, } => {
      lazy_static::lazy_static! {
        $(
            static ref $key: Selector = Selector::parse($value).unwrap();
        )+
      }
   };
);
