# Compare Input

## The challenge

Compare two input strings for equality. Input strings consist of ASCII
characters, backspaces and caps lock toggles. In this implementation, backspaces
are represented as \0 and caps lock toggles as \t as these are easy to embed in
string literals in Rust. The challenge is to do this comparison in constant
space without modifying the input buffers.

## Background

This task was proposed by L.S. Knox at the Hoodlums Meetup
https://www.meetup.com/hoodlums/events/hrbdtnyxfblb/, but he got it from a
friend. We developed a partial solution in Haskell, but didn't complete it due
to too many differences of opinion on how we should handle caps lock toggling. I
reimplemented what we came up with in Rust and finished it off. I haven't tested
all that many cases, but have tried to cover all the combinations of input
features. This implementation is based on many ideas thrown around during the
meetup by all the attendees. 

I've only been learning Rust for a couple of weeks, so the code is probably not
all that idiomatic. That said, I have tried to do it in _the Rust way_ rather than
_Haskell in Rust_. I've written it with machine representation and efficiency in
mind. Comments and suggestions welcome.