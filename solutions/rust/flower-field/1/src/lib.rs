// The object of the game is to 
//    find all the flowers in the garden using numeric hints.
//    The hints indicate how many flowers are directly adjacent (horizontally, vertically, diagonally) to a square. 
//  The garden itself is a rectangle board composed of squares that are either empty (' ') or a flower ('*').

// TODO: Add flower counts to empty squares in a completed Flower Field garden
// For each empty square:
//    count the number of flowers adjacent to it (horizontally, vertically, diagonally). 
//     If the empty square has no adjacent flowers, leave it empty. 
//     Otherwise replace it with the count of adjacent flowers.

// Example:
// You may receive a 5 x 4 board like this (empty spaces are represented here with the '·' character for display on screen):
    // ·*·*·
    // ··*··
    // ··*··
    // ·····
// Which your code should transform into this:
    // 1*3*1
    // 13*31
    // ·2*2·
    // ·111·

// Taking a smaller subset of the problem:
    // *·*
    // ·*·
    // ·*·

// How do I represent this data in Rust?
// Hint from Claude
// Imagine you take all the rows of your garden and lay them end-to-end in a single Vec
// [' ',' ','*',' ',' ', ' ',' ','*',' ',' ', '*','*','*','*','*']
//  0   1   2   3   4    5   6   7   8   9   10  11  12  13  14

// Given that the board is 5 columns wide, how would you express the index of the element at row 1, column 2 (which is the '*' in the middle of the second row) as a formula involving the row, column, and width?

// Assuming rows are 0-indiex, row 1, starts at index 5 than +2 for the colum, so (5,2) -> index 7
// index = row * width + col
// right neighbour = index + 1
// left neighbour = index -1
// up = index - width
// down = index + width
// diagnols = up +1, up -1, down+1, down -1
// For each neighbour
//    Does the index underflow/overflow the entire array?
//    if neighbor_index >= array length
//        skip neighbour
//    
//    Does the neighbor wrap around to a different row?
//        if the neighbour exceeded row width
//        if index of neihbour > the row indices for that row
//        row indices for a row, row*width .. 2*row*width - 1
//           skip neighbour


// How do I build my final Vec<String> from this
    // *·*
    // ·*·
    // ·*·
// we don't seem to need no. of rows
// width = 3
// ['*', ' ', '*', ' ','*', ' ',' ', '*', ' ']
// ['*', '3', '*', '3', '*', '3', '2', '*', '2']
// ["*3*", "3*3", "2*2"]

// Let's write some pseudocode for this
// T1
// for ech of the chars
//    if it's a star, continue
//    else:
//        for neighbours: up, down, left, right and diagnols
//            if neighbor is space 
//                || neighbor_index >= array length 
//                || neighbor_index < row*width
//                || neighbor_index > 2*row*width:
//                skip neighbour
//            else:
//                add count to a new vector
//     
// T2
// for i=0, i<len, i+=width {
//     Read #width chars
//     join chars in a single string, no seperator 
//     push the string to a new vector

use std::slice::Iter;
use std::str::Chars;

pub fn neighbor_indices(index: usize, width: usize, total_indices: usize) -> Vec<usize> {
    let index: i32 = index.try_into().unwrap();
    let width: i32 = width.try_into().unwrap();
    let total_indices: i32 = total_indices.try_into().unwrap();
    let mut neighbor_indices: Vec<i32> = vec![];

    // calculate the row index range given, pivot index, width, total_indices
    // no. of rows = width/total_indices = 3/9 = 3
    // range for row 0, 0..2
    // current row = pivot/width
    // row_range = row*width..row*width + width
    //           = row*width..row*width + width
    // row 0     = 0*3..0*3 + 3 = 0..3
    // row 1     = 1*3..1*3 + 3 = 3..6
    // row 2     = 2*3..2*3 + 3 = 6..9
    // calculate neighbor indices
    // index = 2, width = 3, total_indices = 9
    let curr_row = index/width; // 2/3 = 0
    let curr_row_range = curr_row*width..curr_row*width + width; // 0..3
    // right
    if curr_row_range.contains(&(index + 1)) { // 3 not in 0..3
        neighbor_indices.push(index + 1);
    }
    // left
    if curr_row_range.contains(&(index - 1)) { // 1 in 0..3
       neighbor_indices.push(index - 1);
    }
    // up
    let up = index - width; // 2-3 = -1
    // if up itself isn't a valid index, we needn't bother with its neighbors
    if (0..total_indices).contains(&up) {
        neighbor_indices.push(up); // -1

        let up_row = up/width; // -1/3 = 0
        let up_row_range = up_row*width..up_row*width + width; // 0..3
        if up_row_range.contains(&(up+1)) { // -1 + 1 = 0
            neighbor_indices.push(up+1);
        }
        if up_row_range.contains(&(up-1)) { // -1 -1 = -2
            neighbor_indices.push(up-1);
        }
    }

    // if down itself isn't a valid index, we needn't bother with its neighbors
    let down = index + width;
    if (0..total_indices).contains(&down) {
        neighbor_indices.push(down);

        let down_row = down/width;
        let down_row_range = down_row*width..down_row*width + width;
        if down_row_range.contains(&(down+1)) {
            neighbor_indices.push(down+1)
        }
        if down_row_range.contains(&(down-1)) {
            neighbor_indices.push(down-1)
        }
    }


    let mut real_neighbor_indices = vec![];
    for n in neighbor_indices {
        if n < 0 ||        
           n >= total_indices {
            continue;
        } else {
            real_neighbor_indices.push(n as usize);
        }
    }
    real_neighbor_indices
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return vec![];
    } else if garden.len() == 1 && garden[0].is_empty() {
        return vec!["".into()];
    }


    // assuming a uniform width across rows
    let width = garden[0].chars().count();
    let chars: Vec<char> = garden.iter().flat_map(|row| row.chars()).collect();

    chars
        .chunks(width)
        .map(|chunk| chunk.iter().collect())
        .collect()

}

