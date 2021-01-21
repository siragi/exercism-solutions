/* adapted from glennpratt's solution: https://exercism.io/tracks/rust/exercises/sieve/solutions/a5112b3d37a04dd49bbba597fd03f050
 - half functional, half structural
 - very readable (understandable)
 - clever use of Vec-index as end result
*/
// mod lib_filter_map;
// pub use lib_filter_map::primes_up_to;

/* Commented (I could not see at first how it works), but unchanged excellent zrwi's solution: https://exercism.io/tracks/rust/exercises/sieve/solutions/5f64e30e37884b9aa001f7f45ca40579
 - functional approach
 - To each item of a range it maps a constructor as closure -> map(Option::from) to collect a vec of Options (cool!)
 - It uses Some() vs. None as if it means true, false
 - clever use of take() to change unmarked items 'some(prime)' to None (marked)
 - in combination with '?' (works on options too!) it seems to have the effect of giving back None directly to the holding filter_map() function (like 'return None')
*/
// mod lib_filter_map_take;
// pub use lib_filter_map_take::primes_up_to;

/* Using a hashset to insert multiples of prime: // KoStard's solution: https://exercism.io/tracks/rust/exercises/sieve/solutions/e039e95f34a14201b57d4890541cec00
 - readable, maybe costly because of the inserts
*/
// mod lib_hashset;
// pub use lib_hashset::primes_up_to;

/* My solution is way clumsier than all others, but readable
  - creates a Vec over tuples(u64, bool) via zip of a Range and a Vec<bool>
  - helper fn for getting next unmarked prime and for marking multiples

*/
mod lib_my;
pub use lib_my::primes_up_to;
