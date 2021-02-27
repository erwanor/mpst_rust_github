pub mod cancel;
pub mod choose;
pub mod close;
pub mod fork;
pub mod offer;
pub mod recv;
pub mod send;
pub mod sessionmpst;

///  Creates both the
/// [`mpstthree::close_mpst`](../macro. close_mpst.html)
/// and [`mpstthree::fork_mpst_multi`](../macro. fork_mpst_multi.html)
/// functions to be used with more
/// than 3 participants.  
///
///  # Arguments
///  
///  * The name of the new *close* function
///  * The name of the new *fork* function
///  * The name of the *SessionMpst* type that will be used
///  * The number of participants (all together)
///  
///  # Example
///  
///  ```
///  use mpstthree::{bundle_fork_close_multi, create_sessionmpst};
///
///  create_sessionmpst!(SessionMpst, 3);
///
///  bundle_fork_close_multi!(close_mpst, fork_mpst, SessionMpst, 3);
/// ```
#[macro_export]
macro_rules! bundle_fork_close_multi {
    ($func_name_close:ident, $func_name_fork:ident, $struct_name:ident, $nsessions:literal) => {
        mpstthree::close_mpst!($func_name_close, $struct_name, $nsessions);
        mpstthree::fork_mpst_multi!($func_name_fork, $struct_name, $nsessions);
    };
}
