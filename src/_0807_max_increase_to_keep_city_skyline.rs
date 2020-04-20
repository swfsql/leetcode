//! # Max Increase to Keep City Skyline
//! https://leetcode.com/problems/max-increase-to-keep-city-skyline/
//!
pub struct Solution;

impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        use std::convert::TryInto;
        _max_increase_keeping_skyline(
            grid.into_iter()
                .map(|v| v.into_iter().map(|vi| vi.try_into().unwrap()).collect())
                .collect(),
        )
        .try_into()
        .unwrap()
    }
}

/// Idiomatic entry point.
pub fn _max_increase_keeping_skyline(grid: Vec<Vec<u8>>) -> usize {
    // get horizontal and vertical projections

    // horizontal projection
    let hor: Vec<u8> = grid
        .iter()
        .map(|row| row.iter().max().unwrap())
        .cloned()
        .collect();

    // vertical projection
    let ver: Vec<u8> = grid.iter().fold(vec![0; grid.len()], |mut acc, row| {
        (&mut acc)
            .into_iter()
            .zip(row)
            .map(|(a, b)| *a = std::cmp::max(*a, *b))
            .for_each(drop); // drops ()
        acc
    });

    // for each cell[i][j],
    // takes the minimum between
    // hor[i] and ver[j]
    //
    // for each row
    hor.into_iter()
        // calculate the sum for such row
        .map(|row_max|
            // for each column
            ver.iter()
                .map(|col_max| std::cmp::min(row_max, *col_max))
                .map(usize::from)
                .sum::<usize>()
        )
        // then sums from such row
        .sum::<usize>()
    // then gets subtracted from the original grid total size 
    - grid.into_iter()
        .map(|row| row.into_iter()
            .map(usize::from)
            .sum::<usize>())
        .sum::<usize>()
}
