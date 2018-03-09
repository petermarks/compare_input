extern crate zip_longest;

use zip_longest::ZipLongestIteratorExt;
use zip_longest::EitherOrBoth::Both;

pub fn compare(s1: &str, s2: &str) -> bool {
  // debug assert because this check is expensive on long strings.
  debug_assert!(s1.is_ascii(), "Input s1 must only contain ascii characters.");
  debug_assert!(s2.is_ascii(), "Input s2 must only contain ascii characters.");

  s1.bytes().zip_longest(s2.bytes()).all(|it| if let Both(a, b) = it {a == b} else {false})
}

#[cfg(test)]
mod tests {
  use super::compare;

  #[test]
  fn simple_match() {
    assert_eq!(compare("abc", "abc"), true);
  }

  #[test]
  fn simple_mismatch() {
    assert_eq!(compare("abc", "def"), false);
  }

  #[test]
  fn longer_s1() {
    assert_eq!(compare("abcd", "abc"), false);
  }

  #[test]
  fn longer_s2() {
    assert_eq!(compare("abc", "abcd"), false);
  }
}
