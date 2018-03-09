pub fn compare(s1: &str, s2: &str) -> bool {
  // debug assert because this check is expensive on long strings.
  debug_assert!(s1.is_ascii(), "Input s1 must only contain ascii characters.");
  debug_assert!(s2.is_ascii(), "Input s2 must only contain ascii characters.");

  let mut iter1 = s1.bytes();
  let mut iter2 = s2.bytes();

  loop {
    match (iter1.next(), iter2.next()) {
      (None, None) => break true,
      (None, _) => break false,
      (_, None) => break false,
      (Some(c1), Some(c2)) => if c1 != c2 {break false}
    }
  }
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
