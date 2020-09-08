//! # Regular Expression Matching
//!
//! [![src]][srcurl] [![Tests]][testsurl] [![Doc]][docurl] [![Problem]][problemurl]
//!
//! [src]: https://img.shields.io/badge/source-swfsql/leetcode-8DA0CB?style=for-the-badge&labelColor=555555&logo=github
//! [srcurl]: https://github.com/swfsql/leetcode/blob/master/src/_0010_regular_expression_matching.rs
//! [Tests]: https://img.shields.io/badge/tests-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [testsurl]: https://github.com/swfsql/leetcode/blob/master/tests/_0010_regular_expression_matching.rs
//! [doc]: https://img.shields.io/badge/docs-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [docurl]: https://swfsql.github.io/leetcode/doc/leetcode/_0010_regular_expression_matching/index.html
//! [Problem]: https://img.shields.io/badge/problem-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [problemurl]: https://leetcode.com/problems/regular-expression-matching/
//!
//! Based on [44-wildcard-matching](https://leetcode.com/problems/wildcard-matching/discuss/827028/Rust-tentative-solution-(not-original)-16-ms-2.1-MB).

use std::convert::{TryFrom, TryInto};

pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        Self::_is_match(&s, &p)
    }
    pub fn _is_match(s: &str, p: &str) -> bool {
        _is_match(s, p)
    }
}

/// Character which may actually be "any" character.
#[derive(Debug, Clone)]
enum Char {
    Some(char),
    Any,
}

/// Allows comparing a `Char` with a `char`.  
/// ie. `Char::Any` always matches.
impl PartialEq<char> for Char {
    fn eq(&self, other: &char) -> bool {
        match self {
            Self::Any => true,
            Self::Some(cc) => cc == other,
        }
    }
}


/// Represent all patterns.
///
/// Names based on [Substructural Type System](https://en.wikipedia.org/wiki/Substructural_type_system).
#[derive(Debug, Clone)]
enum Pat {
    /// `[a-z]` or `.` not followed by `*`/`+`/`?`.
    /// Matches a *single* `Char`.
    Linear(Char),
    /// `?`.
    /// Matches *none* or a *single* `Char`.
    Affine(Char),
    /// `+`.
    /// Matches a *single* or *many* times a `Char`.
    Relevant(Char),
    /// `*`.
    /// Matches *none* or a *single* or *many* times a `Char`.
    Normal(Char),
}

/// Treat `?`, `+`, etc.. operators as a conversion failure.
impl TryFrom<char> for Char {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Char::Any),
            '?' | '+' | '*'  => Err(()),
            c => Ok(Char::Some(c)),
        }
    }
}

/// Converts a `Char` (from a `char`) into a `Pat`, while peeking
/// into the next `char`.  
/// This is necessary because `Linear` has no following 
/// `char` operator, while the others have.
///
/// Note that after the last `Char`, there is no "next `char`".
impl TryFrom<(char, Option<char>)> for Pat {
    type Error = ();
    fn try_from((current, next): (char, Option<char>)) -> Result<Self, Self::Error> {
        Ok(match (TryInto::<Char>::try_into(current)?, next) {
            (c, Some('?')) => Self::Affine(c),
            (c, Some('+')) => Self::Relevant(c),
            (c, Some('*')) => Self::Normal(c),
            (c, _) => Self::Linear(c),
        })
    }
}

impl Pat {
    /// Given an string, produces an iterator of pattern.
    pub fn from_str(p: &str) -> impl Iterator<Item=Self> + '_ {
        p
            .chars()
            // we need to "preview" the next char,
            // so zip with itself, but skipping one,
            // but with this we lose the last char
            .zip(p.chars().skip(1).map(Some))
            // so we retrieve that last char
            .chain(p.chars().map(|c| (c, None)).last())
            // map each (curr, next) tuple into a proper Pat
            .map(Pat::try_from)
            // and filter-out errored Pats
            // (since that was part of the conversion logic)
            .filter_map(Result::ok)
    }
}

/// Walker that intends to walk over `chars` and `pats`.  
/// See it's `Iterator` impl for more info.
#[derive(Debug)]
pub struct Walker<'s> {
    last_good: Vec<bool>,
    chars: &'s [char],
    pats: &'s [Pat],
}

/// Iterates by fully walking `chars`.  
/// [And every single step of the way ~(of pain)~](https://youtu.be/VBBFDb0hC4Y?t=42), it fully walks `pats` and updates
/// `last_good`.
impl<'s> Iterator for Walker<'s> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        // the current input/i-wise character is..
        let c = match (self.chars.iter().next(), self.pats.is_empty()) {
            (Some(c), false) => {
                // strings and pats not empty yet,
                // (continue)
                c
            }
            (Some(_c), true) => {
                // strings not empty but patterns empty
                // (no more patterns available)
                return None;
            }
            (None, false) => {
                // strings empty, patterns still not empty
                return match *self.last_good.iter().last().unwrap() {
                    // patterns already matched to the string
                    true => Some(true),
                    // pattern didn't match, and never will
                    false => None,
                };
            }
            (None, true) => {
                // strings and patterns empty
                // they always match
                return Some(true);
            }
        };
        
        let mut next_good = vec![false; self.pats.len() + 1];

        for (j, p) in self.pats.iter().enumerate() {
            next_good[j + 1] = match p {
                // `[a-z]` or `.` not followed by `*`/`+`/`?`.
                // Matches a *single* `Char`.
                Pat::Linear(cc) => {
                    // must advance the string slice (i-wise 
                    // - last_good[j] -> next_good[_]),
                    //
                    // and also itself - the pattern slice
                    // (j-wise - _[j] -> next_good[j+1]),
                    //
                    // and also match the character.
                    //
                    self.last_good[j] && cc == c
                }

                // `?`.
                // Matches *none* or a *single* `Char`.
                Pat::Affine(cc) => {
                    // must advance the string slice (i-wise 
                    // - last_good[j] -> next_good[_])
                    // and also match the character.
                    //
                    // or advance itself - the pattern slice
                    // (j-wise - next_good[j] -> next_good[j+1]),
                    // while ignoring the character.
                    //
                    self.last_good[j] && cc == c || next_good[j] 
                }

                // `+`.
                // Matches a *single* or *many* times a `Char`.
                Pat::Relevant(cc) => {
                    // must advance the string slice (i-wise 
                    // - last_good[j] -> next_good[_],
                    // or last_good[j+1] -> next_good[_]),
                    // and also match the character.
                    //
                    (self.last_good[j] || self.last_good[j + 1]) && cc == c
                }
                
                // `*`.
                // Matches *none* or a *single* or *many* times a `Char`.
                Pat::Normal(cc) => {
                    // must advance the string slice (i-wise 
                    // - last_good[j+1] -> next_good[_])
                    // and also match the character.
                    //
                    // or advance itself - the pattern slice
                    // (j-wise - next_good[j] -> next_good[j+1]),
                    // while ignoring the character.
                    //
                    self.last_good[j + 1] && cc == c || next_good[j] 
                }
            };
        }

        // moves the string (i-wise) forward
        self.chars = &self.chars[1..];
        // forgets the last_good,
        // and only remembers next_good
        self.last_good = next_good;

        // if string has been consumed
        if self.chars.len() == 0 {
            // may return true,
            // if the patterns had matched the string
            Some(*self.last_good.iter().last().unwrap())
        } else {
            // otherwise, there is some string remaining,
            // so a true cannot be guaranteed yet
            Some(false)
        }
    }
}

/// Idiomatic entry point.
pub fn _is_match(s: &str, p: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let pats: Vec<Pat> = Pat::from_str(p).collect();
    let last_good: Vec<bool> = 
        // starts with a single true value
        vec![true]
        .into_iter()
        // chain with another vector
        .chain(
            // based on the pattern
            pats.iter()
                // where some form of skips are also true
                .map(|p| match p {
                    Pat::Normal(_) => true,
                    Pat::Affine(_) => true,
                    _ => false,
                })
                // and those trues must be consecutive from the 
                // beggining, otherwise they are falsified
                .scan(true, |acc, b| {
                    *acc = *acc && b;
                    Some(*acc)
                }),
        )
        .collect();
    let mut walker = Walker {
        last_good,
        chars: chars.as_ref(),
        pats: pats.as_ref(),
    };

    walker.any(|m| m == true)
}
