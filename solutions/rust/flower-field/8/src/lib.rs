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

// 1. Why is neighbor_indices so complex?
//
// Look at how much code exists just to prevent neighbors from "wrapping" across rows. Ask yourself: what caused this complexity? The
// answer is in your data representation. You flattened the 2D board into 1D, so now row boundaries are invisible and you have to
// reconstruct them with arithmetic. What if you never flattened in the first place? How would neighbor boundary checking look if your
// "position" carried both a row and column?
//
// ---
// 2. How do you enumerate the 8 neighbors?
//
// Right now you have distinct code paths for up, down, left, right, up+1, up-1, down+1, down-1. Is there a more uniform way to express
// "all 8 neighbors" that avoids repeating the same pattern 8 times? Think about what the offsets have in common.
//
// ---
// 3. The conversion dance: usize → i32 → usize
//
// You convert to i32 to safely subtract (avoiding underflow), do all your work, then filter out negative values at the end. This is a
// reasonable instinct, but Rust has tools for expressing "this arithmetic might fail" without a type conversion. Look at what methods
// usize exposes for subtraction. Also consider: if you represented positions differently (hint: see point 1), would you even need this
// dance?

pub fn neighbor_count(row: usize, col: usize, garden: &[&str]) -> u8 {
    let width = garden[0].len();
    let rows = garden.len();
    let index_diffs = [
        (0, 1), (0, -1), // right, left
        (-1, 0), (1, 0), // up, down
        (-1, 1), (-1, -1), // up-right, up-left
        (1, 1), (1, -1) // down-right, down-left
    ];

    index_diffs.iter().filter_map(|&(rd, cd)|
        if let Some(r) = row.checked_add_signed(rd)
            && r < rows
            && let Some(c) = col.checked_add_signed(cd)
            && c < width
            && garden[r].as_bytes()[c] == b'*' {
                Some(1)
        } else {
            None
        }
    ).count() as u8
}

// How do I iterate over `garden` in 2 D?
// I could represent `garden` as Vec<Vec<u8>>
// How do I access an index in Vec<Vec<u8>>
// Indices go from (0,0) to (rows - 1, width - 1)
// neighbors can be represented as:
// (row, index + 1), (row, index - 1), right, left
// (row-1, index),(row+1, index), up, down
// (row-1, index +1), (row-1, index-1), up-right, up-left
// (row+1, index +1), (row+1, index -1), down-right, down-left
pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() || garden[0].is_empty() {
        return garden.iter().map(|_| String::new()).collect();
    }


    // assuming a uniform width across rows
    let width = garden[0].len(); // 1 ASCII char = 1 byte
    debug_assert!(garden.iter().all(|row| row.len() == width), "board does not have uniform width!");

    // what if I don't create any intermediate representations?
    // I could enumerate over garden
    // get index access to each row via row.as_bytes then enumerate over it
    // get neighbors and iterate over them too
    // I'm worried the complexity of the problem continues to grow
    // ideal complexity would be row*col,
    // I have row*col*6
    let mut stars: Vec<String> = vec![];
    for (row, cols) in garden.iter().enumerate() {
        let mut star_row = vec![];
        for (col, c) in cols.as_bytes().iter().enumerate() {
            // how do I collapse the following if expressions
            // if neighbor count == 0 || c[index] == b'*', push current_char
            // else push count
            let n_count: u8 = neighbor_count(row, col, garden);
            if n_count == 0 || *c == b'*' {
                star_row.push(*c);
                continue;
            } else {
                star_row.push(b'0' + n_count);
            }
        }
        stars.push(String::from_utf8(star_row).unwrap())
    }
    stars
}

