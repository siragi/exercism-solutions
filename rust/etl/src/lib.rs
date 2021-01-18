/* Extract-Transform-Load (ETL) is a fancy way of saying, "We have some crufty, legacy data over in this system, and now we need it in this shiny new system over here, so we're going to migrate this." */

/* Structural Version
  - two nested for-loops only
  - written in one stroke (fast)
  - most readable
*/
mod lib_struct;
pub use lib_struct::transform;

/* First own functional approach
- uses Iterator functions map() and
- folds collected Iterators into a single Iterator
  (fold can be replaced by flatten() and collect(), which is slightly less verbose, but not so cool :-) ) */
// mod lib_map_fold;
// pub use lib_map_fold::transform;

/* Most elegant function version, adapted from: https://exercism.io/tracks/rust/exercises/etl/solutions/fd172a455487484da3c9786b70a6e0a6
 - uses the all in one Iter function flatmap(), that returns an Iter per input element
 - moves the ownership of every character into the final tuple (consumes chars)
*/
// mod lib_flatmap;
// pub use lib_flatmap::transform;
