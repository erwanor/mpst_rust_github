use crossbeam_channel::{bounded, Sender};
use role::c_to_b::RoleCtoB;
use role::Role;

/// Gives the order to the `SessionMpst` related to B to execute its `session` field with C.
///
/// This `struct` should only be used in the `queue` field of the `SessionMpst` related to B.
pub struct RoleBtoC<R: Role> {
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleBtoC<R> {
    type Dual = RoleCtoB<R::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender, _) = bounded::<R>(1);
        let (sender_dual, _) = bounded::<R::Dual>(1);

        return (
            RoleBtoC {
                sender: sender_dual,
            },
            RoleCtoB { sender: sender },
        );
    }
}

/// Send a value of type `Role`. Always succeeds. Returns the continuation of the
/// queue `R`.
pub fn next_b_to_c<R>(r: RoleBtoC<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    here
}