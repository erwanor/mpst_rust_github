use crossbeam_channel::{bounded, Sender};
use role::all_to_c::RoleAlltoC;
use role::Role;

/// Gives the order to the `SessionMpst` related to C to execute its `session`
/// fields with every other processes.
///
/// This `struct` is used for branching without `enum`. See test `usecase`.
pub struct RoleCtoAll<R1: Role, R2: Role> {
    pub sender1: Sender<R1::Dual>,
    pub sender2: Sender<R2::Dual>,
}

impl<R1: Role, R2: Role> Role for RoleCtoAll<R1, R2> {
    type Dual = RoleAlltoC<R1::Dual, R2::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, _) = bounded::<R1>(1);
        let (sender2, _) = bounded::<R2>(1);
        let (sender_dual1, _) = bounded::<R1::Dual>(1);
        let (sender_dual2, _) = bounded::<R2::Dual>(1);

        return (
            RoleCtoAll {
                sender1: sender_dual1,
                sender2: sender_dual2,
            },
            RoleAlltoC {
                sender1: sender1,
                sender2: sender2,
            },
        );
    }
}

/// Send two values of type `Role` from C, which may be different. Always succeeds. Returns the continuation of the
/// queue `(R1, R2)`.
pub fn next_c_to_all<R1, R2>(r: RoleCtoAll<R1, R2>) -> (R1, R2)
where
    R1: Role,
    R2: Role,
{
    let (here1, there1) = R1::new();
    let (here2, there2) = R2::new();
    r.sender1.send(there1).unwrap_or(());
    r.sender2.send(there2).unwrap_or(());
    (here1, here2)
}