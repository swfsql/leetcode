//! # Climbing Stairs
//!
//! [![src]][srcurl] [![Tests]][testsurl] [![Doc]][docurl] [![Problem]][problemurl]
//!
//! [src]: https://img.shields.io/badge/source-swfsql/leetcode-8DA0CB?style=for-the-badge&labelColor=555555&logo=github
//! [srcurl]: https://github.com/swfsql/leetcode/blob/master/src/_0070_climbing_stairs.rs
//! [Tests]: https://img.shields.io/badge/tests-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [testsurl]: https://github.com/swfsql/leetcode/blob/master/tests/_0070_climbing_stairs.rs
//! [doc]: https://img.shields.io/badge/docs-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [docurl]: https://swfsql.github.io/leetcode/doc/leetcode/_0070_climbing_stairs/index.html
//! [Problem]: https://img.shields.io/badge/problem-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [problemurl]: https://leetcode.com/problems/climbing-stairs/
//!
//! Uses the Fibonacci iterator from [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/trait/iter.html).

use std::convert::TryInto;

pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        Self::_climb_stairs(n.try_into().unwrap())
            .try_into()
            .unwrap()
    }

    /// Idiomatic entry point.
    ///
    /// We pretend that we start at the last stair step,
    /// and calculate the answer for that step.  
    /// By moving the starting point one step below that
    /// last starting point, we just keep re-utilizing
    /// the previous results (we just use the 2 last stair steps
    /// results for that).
    ///
    /// Ie. the result is the Fibonacci sequence.
    pub fn _climb_stairs(n: u8) -> u32 {
        Fib::default().nth(n as usize).unwrap()
    }
}

pub struct Fib {
    pub curr: u32,
    pub next: u32,
}

impl Default for Fib {
    fn default() -> Self {
        Self { curr: 0, next: 1 }
    }
}

impl Iterator for Fib {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.curr + self.next;
        self.curr = self.next;
        self.next = next;
        Some(self.curr)
    }
}
