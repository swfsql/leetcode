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
    chars: &'s [char],
    pats: &'s [Pat],
}

impl<'s> Walker<'s> {
    /// Used to skip items from the slices of chars
    /// and a single item from the patterns.
    pub fn skip(&mut self, n: usize) {
        self.chars = &self.chars[n..];
        self.pats = &self.pats[1..];
    }

    /// Creates a new Walker based on a previous one,
    /// while also applying some skipping.
    pub fn with_skip(&self, n: usize) -> Walker<'_> {
        let chars: &'_ _ = &self.chars[n..];
        let pats: &'_ _ = &self.pats[1..];
        Walker { chars, pats }
    }

    pub fn check_children(&mut self, range: impl Iterator<Item = usize>) -> Option<bool> {
        // we don't know how many chars we need to skip,
        // so we try all possibilities
        // (skipping from 0, 1, 2, .., range-2, range-1, range)
        for l in range {
            // for each new spawned walker,
            // each one skipping some amount of chars,
            let mut w = self.with_skip(l);
            // we see if this new walker is able to
            // satisfy the remaining of the string on
            // the remaining of the pattern
            if w.any(|m| m == true) {
                // if it is, this is sufficient for the
                // parent walker to also be able to
                return Some(true);
            }
        }
        // if none of the spawned walkers were able to
        // satisfy it's remaining string with it's
        // remaining pattern,
        // then the parent definitely also cannot
        return None;
    }

    pub fn is_empty(&self) -> bool {
        self.chars.is_empty() && self.pats.is_empty()
    }
}

impl<'s> Iterator for Walker<'s> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        match self.pats {
            // no more patterns to test.
            [] => match self.is_empty() {
                false => None,
                true => Some(true),
            },
            // `[a-z]`.
            // Consumes a *single specified* char.
            [Pat::LinearChar(c), ..] => match self.chars {
                [cc, ..] if cc == c => {
                    self.skip(1);
                    Some(self.is_empty())
                }

                // different char, or no more chars.
                [] | [..] => None,
            },

            // `?`.
            // Skips *any single* char.
            [Pat::LinearSkip, ..] => match self.chars.len() {
                0 => None,
                _n => {
                    self.skip(1);
                    Some(self.is_empty())
                }
            },

            // `!`.
            // Skips *none* or *any single* char.
            [Pat::AffineSkip, ..] => match self.chars.len() {
                0 => self.check_children(0..=0),
                _n => self.check_children(0..=1),
            },

            // `+`.
            // Skips *any single* or *many arbitrary* chars.
            [Pat::RelevantSkip, ..] => match self.chars.len() {
                0 => None,
                n => self.check_children(0..=n),
            },

            // `*`.
            // Skips *none* or *any single* or *many arbitrary* chars.
            [Pat::NormalSkip, ..] => self.check_children(0..=self.chars.len()),
        }
    }
}

/// Idiomatic entry point.
pub fn _is_match(s: &str, p: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let pats: Vec<Pat> = p.chars().map(Into::into).collect();
    let mut walker = Walker {
        chars: chars.as_ref(),
        pats: pats.as_ref(),
    };

    // let a = aa();
    // a.poll();

    walker.any(|m| m == true)
}

async fn aa() -> bool {
    true
}

use std::cell::RefCell;
use std::future::Future;

use std::pin::Pin;
use std::task::{Context, Poll};

async fn foo() -> u32 {
    3
}

async fn square() -> u32 {
    let a = foo().await;
    let b = foo().await;
    a * b
}

pub fn xx(cx: &mut Context<'_>) -> u32 {
    let mut f = square();
    let mut f = unsafe { Pin::new_unchecked(&mut f) };
    loop {
        if let Poll::Ready(x) = f.as_mut().poll(cx) {
            break x;
        }
    }
}
