//! # Wildcard Matching
//!
//! [![src]][srcurl] [![Tests]][testsurl] [![Doc]][docurl] [![Problem]][problemurl]
//!
//! [src]: https://img.shields.io/badge/source-swfsql/leetcode-8DA0CB?style=for-the-badge&labelColor=555555&logo=github
//! [srcurl]: https://github.com/swfsql/leetcode/blob/master/src/_0044_wildcard_matching.rs
//! [Tests]: https://img.shields.io/badge/tests-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [testsurl]: https://github.com/swfsql/leetcode/blob/master/tests/_0044_wildcard_matching.rs
//! [doc]: https://img.shields.io/badge/docs-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [docurl]: https://swfsql.github.io/leetcode/doc/leetcode/_0044_wildcard_matching/index.html
//! [Problem]: https://img.shields.io/badge/problem-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [problemurl]: https://leetcode.com/problems/wildcard-matching/

pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        Self::_is_match(&s, &p)
    }
    pub fn _is_match(s: &str, p: &str) -> bool {
        _is_match(s, p)
    }
}

/// Represent all patterns.
///
/// Names based on [Substructural Type System](https://en.wikipedia.org/wiki/Substructural_type_system).
#[derive(Debug, Clone)]
enum Pat {
    /// `[a-z]`.
    /// Consumes a *single specified* char.
    LinearChar(char),
    /// `?`.
    /// Skips *any single* char.
    LinearSkip,
    /// `!`.
    /// Skips *none* or *any single* char.
    AffineSkip,
    /// `+`.
    /// Skips *any single* or *many arbitrary* chars.
    RelevantSkip,
    /// `*`.
    /// Skips *none* or *any single* or *many arbitrary* chars.
    NormalSkip,
}

impl From<char> for Pat {
    fn from(c: char) -> Self {
        match c {
            '?' => Self::LinearSkip,
            '!' => Self::AffineSkip,
            '+' => Self::RelevantSkip,
            '*' => Self::NormalSkip,
            c => Self::LinearChar(c),
        }
    }
}

#[derive(Debug)]
pub struct Walker<'s> {
    last_good: Vec<bool>,
    chars: &'s [char],
    pats: &'s [Pat],
}

impl<'s> Iterator for Walker<'s> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        match (self.chars.is_empty(), self.pats.is_empty()) {
            (false, false) => {
                // strings and pats not empty yet,
                // (continue)
            }
            (false, true) => {
                // strings not empty but patterns empty
                // (no more patterns available)
                return None;
            }
            (true, false) => {
                // strings empty, patterns still not empty
                return match *self.last_good.iter().last().unwrap() {
                    // patterns already matched to the string
                    true => Some(true),
                    // pattern didn't match, and never will
                    false => None,
                };
            }
            (true, true) => {
                // strings and patterns empty
                // they always match
                return Some(true);
            }
        };
        let mut next_good = vec![false; self.pats.len() + 1];

        let c = self.chars[0];

        for (j, p) in self.pats.iter().enumerate() {
            match p {
                // `[a-z]`.
                // Consumes a *single specified* char.
                Pat::LinearChar(cc) => {
                    // must advance the string slice (i-wise),
                    // and also itself - the pattern slice (j-wise),
                    // and also match the character.
                    //
                    // obs. advancing i-wise means
                    // using self.last_good[_];
                    // obs. advancing j-wise means using
                    // _[j].
                    next_good[j + 1] = self.last_good[j] && c == *cc;
                }
                // `?`.
                // Skips *any single* char.
                Pat::LinearSkip => {
                    // must advance the string slice (i-wise),
                    // and also itself - the pattern slice
                    // (j-wise).
                    //
                    // obs. advancing i-wise means using
                    // self.last_good[_];
                    // obs. advancing j-wise means using
                    // _[j].
                    next_good[j + 1] = self.last_good[j]
                }
                Pat::AffineSkip => {
                    // must advance the string slice (i-wise)
                    // or advance itself - the pattern slice
                    // (j-wise).
                    // but it always must at least advance
                    // itself.
                    //
                    // obs. advancing i-wise means using self.
                    // last_good[_];
                    // obs. advancing j-wise means using _[j].
                    next_good[j + 1] = self.last_good[j] || next_good[j];
                }
                Pat::RelevantSkip => {
                    // must advance the string slice (i-wise)
                    // or ignore itself - the pattern slice
                    // (j-wise).
                    // but it always must at least advance
                    // i-wise.
                    //
                    // obs. advancing i-wise menas using self.
                    // last_good[_];
                    // obs. advancing j-wise means using _[j].
                    next_good[j + 1] = self.last_good[j] || self.last_good[j + 1];
                }
                Pat::NormalSkip => {
                    // can ignore the strings by advancing the
                    // string slice (i-wise),
                    // or can be ignored by advancing itself, the
                    // pattern slice (j-wise).
                    //
                    // obs. advancing i-wise menas using self.
                    // last_good[_];
                    // obs. advancing j-wise means using _[j].
                    next_good[j + 1] = self.last_good[j + 1] || next_good[j];
                }
            }
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
            // otherwise, there is some pattern remaining,
            // so a true cannot be guaranteed yet
            Some(false)
        }
    }
}

/// Idiomatic entry point.
pub fn _is_match(s: &str, p: &str) -> bool {
    let mut last_good: Vec<bool> = 
        // starts with a single true value
        vec![true]
        .into_iter()
        // chain with another vector
        .chain(
            // based on the pattern
            p.chars()
                .map(Pat::from)
                // where some form of skips are also true
                .map(|p| match p {
                    Pat::NormalSkip => true,
                    Pat::AffineSkip => true,
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
    let chars: Vec<char> = s.chars().collect();
    let pats: Vec<Pat> = p.chars().map(Into::into).collect();
    let mut walker = Walker {
        last_good,
        chars: chars.as_ref(),
        pats: pats.as_ref(),
    };

    walker.any(|m| m == true)
}
