extern crate zip_longest;

use zip_longest::ZipLongestIteratorExt;
use zip_longest::EitherOrBoth::Both;

// Compare two input strings for equality. Input strings consist of ascii
// characters, backspaces and caps lock toggles. Backspaces are represented as
// \0 and caps lock toggles as \t as these are easy to embed in string literals.
// The challenge is to do this in constant space without modifying the input
// buffers.
//
// Caps lock toggles are not yet supported. 
//
// Our approach is to process the strings in reverse so that backspaces just
// become skips of subsequent characters.
pub fn compare(s1: &str, s2: &str) -> bool {
  // debug_assert! because this check is expensive on long strings.
  debug_assert!(s1.is_ascii(), "Input s1 must only contain ascii characters.");
  debug_assert!(s2.is_ascii(), "Input s2 must only contain ascii characters.");

  mk_iter(s1).zip_longest(mk_iter(s2)).all(|it| if let Both(a, b) = it {a == b} else {false})
}

// We would like to return a Filter iterator here, but it contains a closure
// which has an anonymous type, so can't be written. Rust RFC 1522 provides a
// solution, but is not stable yet. For now we will use a box and a trait
// object, but this introduces indirection and dynamic dispatch which should not
// be necessary. It would be better to implement our own iterator rather than
// using the generalised filter. We'll try this next.
fn mk_iter<'a>(s: &'a str) -> Box<Iterator<Item=u8> + 'a> {
  let mut skip = 0;

  let iter = s.bytes().rev().filter(move |c| {
    if *c == b'\0' {
      skip += 1; false
    } else if skip == 0 {
      true
    } else {
      skip -= 1; false
    }
  });

  Box::new(iter)
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

  #[test]
  fn bs_beginning() {
    assert_eq!(compare("\0abc", "abc"), true);
  }

  #[test]
  fn bs_middle() {
    assert_eq!(compare("ax\0bc", "abc"), true);
  }

  #[test]
  fn bs_end() {
    assert_eq!(compare("abcx\0", "abc"), true);
  }

  #[test]
  fn multiple_bs() {
    assert_eq!(compare("abxxx\0\0\0c", "abc"), true);
  }

  #[test]
  fn complex_bs() {
    assert_eq!(compare("abc", "axx\0y\0\0bx\0c"), true);
  }
}
