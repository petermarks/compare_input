extern crate zip_longest;

use zip_longest::ZipLongestIteratorExt;
use zip_longest::EitherOrBoth::Both;
use std::iter::Rev;
use std::str::Bytes;
use std::mem::swap;

// Compare two input strings for equality. Input strings consist of ascii
// characters, backspaces and caps lock toggles. Backspaces are represented as
// \0 and caps lock toggles as \t as these are easy to embed in string literals.
// The challenge is to do this in constant space without modifying the input
// buffers.
//
// Our approach is to process the strings in reverse so that backspaces just
// become skips of subsequent characters. We track caps lock toggles as we go
// and keep state of whether the strings match and/or match with case inverted.
// As we are working backwards, each time we encounter a caps lock toggle, we
// swap our two states over. We must do this once we have finished going through
// the strings too.
//
// If we ever find that both of our states are false, the strings can never
// match and we fail early.
pub fn compare(s1: &str, s2: &str) -> bool {
  // debug_assert! because this check is expensive on long strings.
  debug_assert!(s1.is_ascii(), "Input s1 must only contain ascii characters.");
  debug_assert!(s2.is_ascii(), "Input s2 must only contain ascii characters.");

  // Initialise match state to true. This handles the empty case correctly.
  let mut match_case = true;
  let mut match_inverted = true;

  // We need to hold on to the iterators as we have to check the caps lock
  // toggle state at the end.
  let mut it1 = mk_iter(s1);
  let mut it2 = mk_iter(s2);
  
  for i in (&mut it1).zip_longest(&mut it2) {
    if let Both((c1, t1), (c2, t2)) = i {
      // If only one of the input strings has toggled caps lock, swap the match
      // state.
      if t1 ^ t2 {
        swap(&mut match_case, &mut match_inverted)
      }

      let c2_inverted = if c2.is_ascii_lowercase() {c2.to_ascii_uppercase()} else {c2.to_ascii_lowercase()};
      match_case &= c1 == c2;
      match_inverted &= c1 == c2_inverted;

      if !(match_case || match_inverted) {return false}
    } else {
      // One of the strings contains additional characters on the front.
      return false
    }
  }

  // If one of the input strings has toggled caps lock at the beginning, return
  // the inverted match state else return the match state.
  return if it1.is_caps_toggled() ^ it2.is_caps_toggled() {match_inverted} else {match_case}
}

// Iter iterates over the input string in reverse skipping characters which have
// been backspaced. It also keeps a caps lock toggled state which indicates an
// odd number of caps locks since the *previous* character emitted. It yields
// the caps lock toggled state with each byte and the state can also be queried
// directly, even after the iterator has finished. This is needed in order to
// handle caps locks at the beginning of the input string.
struct Iter<'a> {
  bytes: Rev<Bytes<'a>>,
  caps_toggled: bool,
} 

#[inline]
fn mk_iter<'a>(s: &'a str) -> Iter<'a> {
  Iter{
    bytes: s.bytes().rev(),
    caps_toggled: false,
  }
}

impl<'a> Iter<'a> {
  #[inline]
  fn is_caps_toggled(&self) -> bool {
    self.caps_toggled
  }
}

impl<'a> Iterator for Iter<'a> {
  type Item = (u8, bool);

  #[inline]
  fn next(&mut self) -> Option<(u8, bool)> {
    self.caps_toggled = false; // We only want to track toggles since the previous character.
    let mut skip = 0;

    for c in &mut self.bytes {
      match c {
        b'\0' => skip += 1,
        b'\t' => self.caps_toggled ^= true, // Toggle the caps_toggled field.
        c => if skip == 0 {return Some((c, self.caps_toggled))} else {skip -= 1}
      }
    }
    None
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    let (_, upper) = self.bytes.size_hint();
    (0, upper) // Can't know a lower bound, as all remaining bytes could be deleted.
  }
}

#[cfg(test)]
mod tests {
  use super::compare;

  #[test]
  fn empty_match() {
    assert_eq!(compare("", ""), true);
  }

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
  fn prefix_s1() {
    assert_eq!(compare("xabc", "abc"), false);
  }

  #[test]
  fn prefix_s2() {
    assert_eq!(compare("abc", "xabc"), false);
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

  #[test]
  fn caps_match() {
    assert_eq!(compare("ABC", "\tabc"), true);
  }

  #[test]
  fn caps_lower_match() {
    assert_eq!(compare("\tABC", "abc"), true);
  }

  #[test]
  fn caps_no_match() {
    assert_eq!(compare("abc", "\tabc"), false);
  }

  #[test]
  fn caps_on_off() {
    assert_eq!(compare("a\tb\tc", "aBc"), true);
  }

  #[test]
  fn caps_end() {
    assert_eq!(compare("abc\t", "abc"), true);
  }

  #[test]
  fn caps_with_bs() {
    assert_eq!(compare("abc\t\0C", "abc"), true);
  }
}
