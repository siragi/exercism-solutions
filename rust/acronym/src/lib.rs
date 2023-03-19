/* Adapted from godtopus's solution: https://exercism.io/tracks/rust/exercises/acronym/solutions/95aa8536d16746f09807e63929984b2a#solution-comment-171809
  - Regular expression (regex) Search, that is actually readable (thanks to ryanplusplus)
  - the pushing of captures works like this: First Capture Element holds the full match as str: CamelCase -> Camel, next loop generates -> Case, and str can be sliced, when given a range (so you take the first letter with capture[0][..1] ).
  => Behaviour: 'wOrDS' -> 'WOD' (i prefer that Result)
*/
mod lib_regex;
pub use lib_regex::abbreviate;

/* My functional approach,
   - uses 'match if' ... statements
   - not many lines but longer ones
   - understandable at first glance
   => Behaviour: 'wOrDS' -> 'ODS'
*/
// mod lib_my;
// pub use lib_my::abbreviate;

/*  Adaptation of menski's very functional solution
    - almost the same but instead of 'match' statement it uses
    - chars().take(1).chain()
    - skipwhile() and filter()
    => Behaviour: 'wOrDS' -> 'WODS'
*/
// mod lib_skipwhile;
// pub use lib_skipwhile::abbreviate;
