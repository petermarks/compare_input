extern crate zip_longest;

use zip_longest::ZipLongestIteratorExt;
use zip_longest::EitherOrBoth::Both;
use std::iter::Rev;
use std::str::Bytes;

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

// Iter iterates over the input string in reverse skipping characters which have
// been backspaced.
struct Iter<'a> {
  bytes: Rev<Bytes<'a>>,
} 

#[inline]
fn mk_iter<'a>(s: &'a str) -> Iter<'a> {
  Iter{
    bytes: s.bytes().rev(),
  }
}

impl<'a> Iterator for Iter<'a> {
  type Item = u8;

  #[inline]
  fn next(&mut self) -> Option<u8> {
    let mut skip = 0;

    for c in &mut self.bytes {
      if c == b'\0' {
        skip += 1;
      } else if skip == 0 {
        return Some(c);
      } else {
        skip -= 1;
      }
    }
    None
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    let (_, upper) = self.bytes.size_hint();
    (0, upper) // can't know a lower bound, as all remaining bytes could be deleted
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
