/* Version that sticks to the given function and struct declaration
   - structural approach.
*/
mod lib_original;
pub use lib_original::*;

/* Version that combines the ReadStats and WriteStats via Type aliasing, referring to IoStats Struct
   - structural approach
*/
// mod lib_iostat;
// pub use lib_iostat::*;

/* Version that combines the ReadStats and WriteStats via Type aliasing, referring to IoStats Struct
   - functional approach
*/
// mod lib_iostat_funct;
// pub use lib_iostat_funct::*;
