/* My very cool functional Solution
  - gives the new struct in one stroke. not a single unnecessary variables, when the even the struct itself has no var.
  - refined in quite some time
  - use of question mark '?', after having collected a Result<String, usize> to get the error out, that was the key to success :-)
  - Instead of chars().enumerate() -> chars_indices()
*/
// mod lib_funct_novars;
// pub use lib_funct_novars::*;

/* or the same version, but with 10 lines more code, because struct has named fields (vars): */
// mod lib_funct_vars;
// pub use lib_funct_vars::*;

/* My first attempt
  - structural/procedural
  - made quite fast
*/
// mod lib_struct;
// pub use lib_struct::*;

mod lib_emilio;
pub use lib_emilio::*;
