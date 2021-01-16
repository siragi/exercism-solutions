// uncomment version you want to use:

/* half struct/functional approach, that stores all rows in struct. First attempt.
 - uses .windows(2) to generate slices of pairs
 - sums each window with .map() and .sum()
 - initialize the vector with just the right capacity!
 - use append() to merge a Vec with another vector
*/
mod pascals_triangles_rows;
pub use pascals_triangles_rows::*;

/*  mostly functional approach, that stores only row_count in struct.
 - nice use of .once() .chain() range and scan() (analog fold())
 - next row uses current row and .zip().skip(1) to generates almost the same like .windows(2)
*/
// mod pascals_triangles_row_count;
// pub use pascals_triangles_row_count::*;
