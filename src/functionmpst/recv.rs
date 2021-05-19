//! This module contains all the *receive* functions

use crate::binary::struct_trait::{Recv, Session};
use crate::role::a::RoleA;
use crate::role::all_to_a::RoleAlltoA;
use crate::role::all_to_b::RoleAlltoB;
use crate::role::all_to_c::RoleAlltoC;
use crate::role::b::RoleB;
use crate::role::c::RoleC;
use crate::role::end::RoleEnd;
use crate::role::Role;
use crate::sessionmpst::SessionMpst;
use std::error::Error;
use std::marker;

type ResultBoxError<T, S1, S2, R, N> = Result<(T, SessionMpst<S1, S2, R, N>), Box<dyn Error>>;

// #[doc(hidden)]
// #[macro_export]
// macro_rules! recv_aux_simple {
//     ($session:expr, $role:ident, $exclusion:literal) => {
//         mpst_seq::seq!(N in 1..3 ! $exclusion {
//             || -> Result<_, Box<dyn std::error::Error>> { // exclusion: index of binary channel among the 2 others

//                 %(
//                 )(
//                     let (v, new_session) = crate::binary::recv::recv($session.session#N:0)?;
//                 )0*

//                 let new_stack = {
//                     fn temp<R>(r: $role<R>) -> R
//                     where
//                         R: crate::role::Role,
//                     {
//                         let (here, there) = <R as crate::role::Role>::new();
//                         r.sender.send(there).unwrap_or(());
//                         here
//                     }
//                     temp($session.stack)
//                 };

//                 Ok((
//                     v,
//                     crate::sessionmpst::SessionMpst {
//                         %(
//                             session#N:0: $session.session#N:0,
//                         )(
//                             session#N:0: new_session,
//                         )0*
//                         stack: new_stack,
//                         name: $session.name,
//                     }
//                 ))
//             }
//         });
//     }
// }
#[doc(hidden)]
#[macro_export]
macro_rules! recv_aux_simple {
    ($session:expr, $role:ident, $exclusion:literal) => {
        mpst_seq::recv_aux_simple!($role, $exclusion, ($session));
    };
}

// #[doc(hidden)]
// #[macro_export]
// macro_rules! recv_all_aux_simple {
//     ($session:expr, $role:ident, $exclusion:literal) => {
//         mpst_seq::seq!(N in 1..3 ! $exclusion {
//             || -> Result<_, Box<dyn std::error::Error>> { // exclusion: index of binary channel among the 2 others

//                 %(
//                 )(
//                     let (v, new_session) = crate::binary::recv::recv($session.session#N:0)?;
//                 )0*

//                 let (new_stack_left, _new_stack_right) = { // new_stack_right = new_stack_left
//                     fn temp(r: $role<crate::role::end::RoleEnd, crate::role::end::RoleEnd>) -> (crate::role::end::RoleEnd, crate::role::end::RoleEnd)
//                     {
//                         let (here1, there1) = <crate::role::end::RoleEnd as crate::role::Role>::new();
//                         let (here2, there2) = <crate::role::end::RoleEnd as crate::role::Role>::new();
//                         r.sender1.send(there1).unwrap_or(());
//                         r.sender2.send(there2).unwrap_or(());
//                         (here1, here2)
//                     }
//                     temp($session.stack)
//                 };

//                 Ok((
//                     v,
//                     crate::sessionmpst::SessionMpst {
//                         %(
//                             session#N:0: $session.session#N:0,
//                         )(
//                             session#N:0: new_session,
//                         )0*
//                         stack: new_stack_left,
//                         name: $session.name,
//                     }
//                 ))
//             }
//         });
//     }
// }
#[doc(hidden)]
#[macro_export]
macro_rules! recv_all_aux_simple {
    ($session:expr, $role:ident, $exclusion:literal) => {
        mpst_seq::recv_all_aux_simple!($role, $exclusion, ($session));
    };
}

/// Receive a value of type `T` on A from B. Can fail.
/// Returns either a pair of the received value and the
/// continuation of the `SessionMpst<S1, S2, R, N>`
/// or an error.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{End, Recv, Session};
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::functionmpst::recv::recv_mpst_a_from_b;
/// use mpstthree::functionmpst::send::send_mpst_b_to_a;
///
/// // Creating the binary sessions
/// type AtoB = Recv<(), End>;
/// type BtoA = <AtoB as Session>::Dual;
///
/// // Stack
/// type StackA = RoleB<RoleEnd>;
/// type StackB = RoleA<RoleEnd>;
///
/// // Name
/// type NameA = RoleA<RoleEnd>;
/// type NameB = RoleB<RoleEnd>;
///
/// // Creating the MP sessions
/// type EndpointA = SessionMpst<AtoB, End, StackA, NameA>;
/// type EndpointB = SessionMpst<BtoA, End, StackB, NameB>;
///
/// // From this point...
///
/// let (channel_ab, channel_ba) = AtoB::new();
/// let (channel_ac, _) = End::new();
/// let (channel_bc, _) = End::new();
///
/// let (role_a, _) = StackA::new();
/// let (role_b, _) = StackB::new();
///
/// let (name_a, _) = NameA::new();
/// let (name_b, _) = NameB::new();
///
/// let sess_a = SessionMpst {
///   session1: channel_ab,
///   session2: channel_ac,
///   stack: role_a,
///   name: name_a,
/// };
///
/// let sess_b = SessionMpst {
///   session1: channel_ba,
///   session2: channel_bc,
///   stack: role_b,
///   name: name_b,
/// };
///
/// // ...to this point, should not be written in general. Please look at [`mpstthree::fork`](../fork/index.html).
///
/// send_mpst_b_to_a((), sess_b);
/// recv_mpst_a_from_b(sess_a);
/// ```
pub fn recv_mpst_a_from_b<T, S1, S2, R>(
    s: SessionMpst<Recv<T, S1>, S2, RoleB<R>, RoleA<RoleEnd>>,
) -> ResultBoxError<T, S1, S2, R, RoleA<RoleEnd>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    recv_aux_simple!(s, RoleB, 1)()
}

/// Receive a value of type `T` on B from A. Can fail.
/// Returns either a pair of the received value and the
/// continuation of the `SessionMpst<S1, S2, R, N>`
/// or an error.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{End, Recv, Session};
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::functionmpst::recv::recv_mpst_b_from_a;
/// use mpstthree::functionmpst::send::send_mpst_a_to_b;
///
/// // Creating the binary sessions
/// type BtoA = Recv<(), End>;
/// type AtoB = <BtoA as Session>::Dual;
///
/// // Stack
/// type StackB = RoleA<RoleEnd>;
/// type StackA = RoleB<RoleEnd>;
///
/// // Name
/// type NameB = RoleB<RoleEnd>;
/// type NameA = RoleA<RoleEnd>;
///
/// // Creating the MP sessions
/// type EndpointB = SessionMpst<BtoA, End, StackB, NameB>;
/// type EndpointA = SessionMpst<AtoB, End, StackA, NameA>;
///
/// // From this point...
///
/// let (channel_ba, channel_ab) = BtoA::new();
/// let (channel_ac, _) = BtoA::new();
/// let (channel_bc, _) = End::new();
///
/// let (role_b, _) = StackB::new();
/// let (role_a, _) = StackA::new();
///
/// let (name_b, _) = NameB::new();
/// let (name_a, _) = NameA::new();
///
/// let sess_b = SessionMpst {
///   session1: channel_ba,
///   session2: channel_bc,
///   stack: role_b,
///   name: name_b,
/// };
///
/// let sess_a = SessionMpst {
///   session1: channel_ab,
///   session2: channel_ac,
///   stack: role_a,
///   name: name_a,
/// };
///
/// // ...to this point, should not be written in general. Please look at [`mpstthree::fork`](../fork/index.html).
///
/// send_mpst_a_to_b((), sess_a);
/// recv_mpst_b_from_a(sess_b);
/// ```
pub fn recv_mpst_b_from_a<T, S1, S2, R>(
    s: SessionMpst<Recv<T, S1>, S2, RoleA<R>, RoleB<RoleEnd>>,
) -> ResultBoxError<T, S1, S2, R, RoleB<RoleEnd>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    recv_aux_simple!(s, RoleA, 1)()
}

/// Receive a value of type `T` on C from A. Can fail.
/// Returns either a pair of the received value and the
/// continuation of the `SessionMpst<S1, S2, R, N>`
/// or an error.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{End, Recv, Session};
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::functionmpst::recv::recv_mpst_c_from_a;
/// use mpstthree::functionmpst::send::send_mpst_a_to_c;
///
/// // Creating the binary sessions
/// type CtoA = Recv<(), End>;
/// type AtoC = <CtoA as Session>::Dual;
///
/// // Stack
/// type StackC = RoleA<RoleEnd>;
/// type StackA = RoleC<RoleEnd>;
///
/// // Name
/// type NameC = RoleC<RoleEnd>;
/// type NameA = RoleA<RoleEnd>;
///
/// // Creating the MP sessions
/// type EndpointC = SessionMpst<CtoA, End, StackC, NameC>;
/// type EndpointA = SessionMpst<End, AtoC, StackA, NameA>;
///
/// // From this point...
///
/// let (channel_ca, channel_ac) = CtoA::new();
/// let (channel_cb, _) = End::new();
/// let (channel_ab, _) = End::new();
///
/// let (role_c, _) = StackC::new();
/// let (role_a, _) = StackA::new();
///
/// let (name_c, _) = NameC::new();
/// let (name_a, _) = NameA::new();
///
/// let sess_c = SessionMpst {
///   session1: channel_ca,
///   session2: channel_cb,
///   stack: role_c,
///   name: name_c,
/// };
///
/// let sess_a = SessionMpst {
///   session1: channel_ab,
///   session2: channel_ac,
///   stack: role_a,
///   name: name_a,
/// };
///
/// // ...to this point, should not be written in general. Please look at [`mpstthree::fork`](../fork/index.html).
///
/// send_mpst_a_to_c((), sess_a);
/// recv_mpst_c_from_a(sess_c);
/// ```
pub fn recv_mpst_c_from_a<T, S1, S2, R>(
    s: SessionMpst<Recv<T, S1>, S2, RoleA<R>, RoleC<RoleEnd>>,
) -> ResultBoxError<T, S1, S2, R, RoleC<RoleEnd>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    recv_aux_simple!(s, RoleA, 1)()
}

/// Receive a value of type `T` on A from C. Can fail.
/// Returns either a pair of the received value and the
/// continuation of the `SessionMpst<S1, S2, R, N>`
/// or an error.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{End, Recv, Session};
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::functionmpst::recv::recv_mpst_a_from_c;
/// use mpstthree::functionmpst::send::send_mpst_c_to_a;
///
/// // Creating the binary sessions
/// type AtoC = Recv<(), End>;
/// type CtoA = <AtoC as Session>::Dual;
///
/// // Stack
/// type StackA = RoleC<RoleEnd>;
/// type StackC = RoleA<RoleEnd>;
///
/// // Name
/// type NameA = RoleA<RoleEnd>;
/// type NameC = RoleC<RoleEnd>;
///
/// // Creating the MP sessions
/// type EndpointA = SessionMpst<End, AtoC, StackA, NameA>;
/// type EndpointC = SessionMpst<CtoA, End, StackC, NameC>;
///
/// // From this point...
///
/// let (channel_ab, _) = End::new();
/// let (channel_cb, _) = End::new();
/// let (channel_ac, channel_ca) = AtoC::new();
///
/// let (role_a, _) = StackA::new();
/// let (role_c, _) = StackC::new();
///
/// let (name_a, _) = NameA::new();
/// let (name_c, _) = NameC::new();
///
/// let sess_a = SessionMpst {
///   session1: channel_ab,
///   session2: channel_ac,
///   stack: role_a,
///   name: name_a,
/// };
///
/// let sess_c = SessionMpst {
///   session1: channel_ca,
///   session2: channel_cb,
///   stack: role_c,
///   name: name_c,
/// };
///
/// // ...to this point, should not be written in general. Please look at [`mpstthree::fork`](../fork/index.html).
///
/// send_mpst_c_to_a((), sess_c);
/// recv_mpst_a_from_c(sess_a);
/// ```
pub fn recv_mpst_a_from_c<T, S1, S2, R>(
    s: SessionMpst<S1, Recv<T, S2>, RoleC<R>, RoleA<RoleEnd>>,
) -> ResultBoxError<T, S1, S2, R, RoleA<RoleEnd>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    recv_aux_simple!(s, RoleC, 2)()
}

/// Receive a value of type `T` on B from C. Can fail.
/// Returns either a pair of the received value and the
/// continuation of the `SessionMpst<S1, S2, R, N>`
/// or an error.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{End, Recv, Session};
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::functionmpst::recv::recv_mpst_b_from_c;
/// use mpstthree::functionmpst::send::send_mpst_c_to_b;
///
/// // Creating the binary sessions
/// type BtoC = Recv<(), End>;
/// type CtoB = <BtoC as Session>::Dual;
///
/// // Stack
/// type StackB = RoleC<RoleEnd>;
/// type StackC = RoleB<RoleEnd>;
///
/// // Name
/// type NameB = RoleB<RoleEnd>;
/// type NameC = RoleC<RoleEnd>;
///
/// // Creating the MP sessions
/// type EndpointB = SessionMpst<End, BtoC, StackB, NameB>;
/// type EndpointC = SessionMpst<End, CtoB, StackC, NameC>;
///
/// // From this point...
///
/// let (channel_ba, _) = End::new();
/// let (channel_ca, _) = End::new();
/// let (channel_bc, channel_cb) = BtoC::new();
///
/// let (role_b, _) = StackB::new();
/// let (role_c, _) = StackC::new();
///
/// let (name_b, _) = NameB::new();
/// let (name_c, _) = NameC::new();
///
/// let sess_b = SessionMpst {
///   session1: channel_ba,
///   session2: channel_bc,
///   stack: role_b,
///   name: name_b,
/// };
///
/// let sess_c = SessionMpst {
///   session1: channel_ca,
///   session2: channel_cb,
///   stack: role_c,
///   name: name_c,
/// };
///
/// // ...to this point, should not be written in general. Please look at [`mpstthree::fork`](../fork/index.html).
///
/// send_mpst_c_to_b((), sess_c);
/// recv_mpst_b_from_c(sess_b);
/// ```
pub fn recv_mpst_b_from_c<T, S1, S2, R>(
    s: SessionMpst<S1, Recv<T, S2>, RoleC<R>, RoleB<RoleEnd>>,
) -> ResultBoxError<T, S1, S2, R, RoleB<RoleEnd>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    recv_aux_simple!(s, RoleC, 2)()
}

/// Receive a value of type `T` on C from B. Can fail.
/// Returns either a pair of the received value and the
/// continuation of the `SessionMpst<S1, S2, R, N>`
/// or an error.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{End, Recv, Session};
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::functionmpst::recv::recv_mpst_c_from_b;
/// use mpstthree::functionmpst::send::send_mpst_b_to_c;
///
/// // Creating the binary sessions
/// type CtoB = Recv<(), End>;
/// type BtoC = <CtoB as Session>::Dual;
///
/// // Stack
/// type StackC = RoleB<RoleEnd>;
/// type StackB = RoleC<RoleEnd>;
///
/// // Name
/// type NameC = RoleC<RoleEnd>;
/// type NameB = RoleB<RoleEnd>;
///
/// // Creating the MP sessions
/// type EndpointC = SessionMpst<End, CtoB, StackC, NameC>;
/// type EndpointB = SessionMpst<End, BtoC, StackB, NameB>;
///
/// // From this point...
///
/// let (channel_ba, _) = End::new();
/// let (channel_ca, _) = End::new();
/// let (channel_cb, channel_bc) = CtoB::new();
///
/// let (role_c, _) = StackC::new();
/// let (role_b, _) = StackB::new();
///
/// let (name_c, _) = NameC::new();
/// let (name_b, _) = NameB::new();
///
/// let sess_c = SessionMpst {
///   session1: channel_ca,
///   session2: channel_cb,
///   stack: role_c,
///   name: name_c,
/// };
///
/// let sess_b = SessionMpst {
///   session1: channel_ba,
///   session2: channel_bc,
///   stack: role_b,
///   name: name_b,
/// };
///
/// // ...to this point, should not be written in general. Please look at [`mpstthree::fork`](../fork/index.html).
///
/// send_mpst_b_to_c((), sess_b);
/// recv_mpst_c_from_b(sess_c);
/// ```
pub fn recv_mpst_c_from_b<T, S1, S2, R>(
    s: SessionMpst<S1, Recv<T, S2>, RoleB<R>, RoleC<RoleEnd>>,
) -> ResultBoxError<T, S1, S2, R, RoleC<RoleEnd>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    recv_aux_simple!(s, RoleB, 2)()
}

/// Receive a broadcasted value of type `T` on B from A. Can
/// fail. Returns either a pair of the received value and
/// the continuation of the `SessionMpst<S1, S2, R, N>` or
/// an error. Should not be used as a standalone, but rather
/// with [`mpstthree::offer::offer_mpst_session_to_a_from_b`].
#[doc(hidden)]
pub fn recv_mpst_a_all_to_b<T, S1, S2>(
    s: SessionMpst<Recv<T, S1>, S2, RoleAlltoB<RoleEnd, RoleEnd>, RoleA<RoleEnd>>,
) -> ResultBoxError<T, S1, S2, RoleEnd, RoleA<RoleEnd>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
{
    recv_all_aux_simple!(s, RoleAlltoB, 1)()
}

/// Receive a broadcasted value of type `T` on C from A. Can
/// fail. Returns either a pair of the received value and
/// the continuation of the `SessionMpst<S1, S2, R, N>` or
/// an error. Should not be used as a standalone, but rather
/// with [`mpstthree::offer::offer_mpst_session_to_a_from_c`].
#[doc(hidden)]
pub fn recv_mpst_a_all_to_c<T, S1, S2>(
    s: SessionMpst<S1, Recv<T, S2>, RoleAlltoC<RoleEnd, RoleEnd>, RoleA<RoleEnd>>,
) -> ResultBoxError<T, S1, S2, RoleEnd, RoleA<RoleEnd>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
{
    recv_all_aux_simple!(s, RoleAlltoC, 2)()
}

/// Receive a broadcasted value of type `T` on A from B. Can
/// fail. Returns either a pair of the received value and
/// the continuation of the `SessionMpst<S1, S2, R, N>` or
/// an error. Should not be used as a standalone, but rather
/// with [`mpstthree::offer::offer_mpst_session_to_b_from_a`].
#[doc(hidden)]
pub fn recv_mpst_b_all_to_a<T, S1, S2>(
    s: SessionMpst<Recv<T, S1>, S2, RoleAlltoA<RoleEnd, RoleEnd>, RoleB<RoleEnd>>,
) -> ResultBoxError<T, S1, S2, RoleEnd, RoleB<RoleEnd>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
{
    recv_all_aux_simple!(s, RoleAlltoA, 1)()
}

/// Receive a broadcasted value of type `T` on C from A. Can
/// fail. Returns either a pair of the received value and
/// the continuation of the `SessionMpst<S1, S2, R, N>` or
/// an error. Should not be used as a standalone, but rather
/// with [`mpstthree::offer::offer_mpst_session_to_b_from_c`].
#[doc(hidden)]
pub fn recv_mpst_b_all_to_c<T, S1, S2>(
    s: SessionMpst<S1, Recv<T, S2>, RoleAlltoC<RoleEnd, RoleEnd>, RoleB<RoleEnd>>,
) -> ResultBoxError<T, S1, S2, RoleEnd, RoleB<RoleEnd>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
{
    recv_all_aux_simple!(s, RoleAlltoC, 2)()
}

/// Receive a broadcasted value of type `T` on A from B. Can
/// fail. Returns either a pair of the received value and
/// the continuation of the `SessionMpst<S1, S2, R, N>` or
/// an error. Should not be used as a standalone, but rather
/// with [`mpstthree::offer::offer_mpst_session_to_c_from_a`].
#[doc(hidden)]
pub fn recv_mpst_c_all_to_a<T, S1, S2>(
    s: SessionMpst<Recv<T, S1>, S2, RoleAlltoA<RoleEnd, RoleEnd>, RoleC<RoleEnd>>,
) -> ResultBoxError<T, S1, S2, RoleEnd, RoleC<RoleEnd>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
{
    recv_all_aux_simple!(s, RoleAlltoA, 1)()
}

/// Receive a broadcasted value of type `T` on B from C. Can
/// fail. Returns either a pair of the received value and
/// the continuation of the `SessionMpst<S1, S2, R, N>` or
/// an error. Should not be used as a standalone, but rather
/// with [`mpstthree::offer::offer_mpst_session_to_c_from_b`].
#[doc(hidden)]
pub fn recv_mpst_c_all_to_b<T, S1, S2>(
    s: SessionMpst<S1, Recv<T, S2>, RoleAlltoB<RoleEnd, RoleEnd>, RoleC<RoleEnd>>,
) -> ResultBoxError<T, S1, S2, RoleEnd, RoleC<RoleEnd>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
{
    recv_all_aux_simple!(s, RoleAlltoB, 2)()
}
