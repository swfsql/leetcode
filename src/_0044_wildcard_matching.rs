//! # Wildcard Matching
//! https://leetcode.com/problems/wildcard-matching/
//!
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
pub struct Walker<'s, 'p> {
    chars: &'s [char],
    pats: &'p mut [Pat],
}

impl<'s, 'p> Walker<'s, 'p> {
    /// Used to skip items from the slices of chars
    /// and a single item from the patterns.
    pub fn skip(&mut self, n: usize) {
        self.chars = &self.chars[n..];

        // this step is necessary for lifetime reasons
        // https://stackoverflow.com/questions/61223234
        let pats_ = std::mem::replace(&mut self.pats, &mut []);
        // split the head out (and throw it away)
        let (_head, tail) = pats_.split_at_mut(1);
        // the, from now on, we'll only deal with the tail
        self.pats = tail;
    }

    /// Same as `skip`, except the string len is checked.
    pub fn try_skip(&mut self, n: usize) -> Option<()> {
        if self.chars.len() < n {
            None
        } else {
            Some(self.skip(n))
        }
    }

    /// Creates a new Walker based on a previous one,
    /// while also applying some skipping.
    pub fn with_skip(&mut self, n: usize) -> Walker<'_, '_> {
        let chars: &'_ _ = &self.chars[n..];
        let pats: &'_ mut _ = &mut self.pats[1..];
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

impl<'s, 'p> Iterator for Walker<'s, 'p>
where
    Self: 'p,
{
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        match self.pats {
            // no more patterns to test.
            [] => {
                return match self.is_empty() {
                    false => None,
                    true => Some(true),
                };
            }
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
                    return Some(self.is_empty());
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
    let mut pats: Vec<Pat> = p.chars().map(Into::into).collect();
    let mut walker = Walker {
        chars: chars.as_ref(),
        pats: pats.as_mut(),
    };
    walker.any(|m| m == true)
}
