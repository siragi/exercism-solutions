/* # My original solution:
  - functional approach, use of chain(once) and find() for plausability check.
  - find gives an option to match over and extract Err or usize to make a Result.
Result
  - implements FromIterator, so you can move the Result outside (here it is the function count) and iterators will take care of the rest (including stopping iteration if an error is found, here the map() that usees count). Question mark throws Error.
  from: https://stackoverflow.com/questions/26368288/how-do-i-stop-iteration-and-return-an-error-when-iteratormap-returns-a-result
*/
mod lib_orig;
pub use lib_orig::*;

/* # Analog orignal solution, but here i did not know, that Result implements FromIterator, so i made use of a
  - helper function named until_err(), that translates a Result to an Option, which is then understood by the scan() function as a closure
*/
// mod lib_until_err;
// pub use lib_until_err::*;

/* bucatini-coder's solution:
  - perfect use of match statement, very readable
  - makes use of the fact, that sum() evaluates as Result and throws an Err() if one of the elements cannot be added, which is perfect!
  - the nucleotide_counts constructs via map() a Resuit<tuple> which can be easily turned into a HashMap.
*/
// mod lib_match_sum;
// pub use lib_match_sum::*;

/* someone else's solution, unfortunately i don't find the creator again, but i like the straight forward structural approach,
  - make a var and throw Errors in good old if-clauses
  - for loop to go through an iterator and simply insert what is needed.
*/
// mod lib_struct;
// pub use lib_struct::*;
