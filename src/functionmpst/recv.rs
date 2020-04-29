use binary::{recv, Recv, Session};
use role::a_to_b::{next_a_to_b, RoleAtoB};
use role::a_to_c::{next_a_to_c, RoleAtoC};
use role::b_to_a::{next_b_to_a, RoleBtoA};
use role::b_to_c::{next_b_to_c, RoleBtoC};
use role::c_to_a::{next_c_to_a, RoleCtoA};
use role::c_to_b::{next_c_to_b, RoleCtoB};
use role::Role;
use sessionmpst::SessionMpst;
use std::error::Error;
use std::marker;

type ResultBoxError<T, S1, S2, R> = Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>;

/// Receive a value of type `T` on A from B. Can fail. Returns either a pair of the received
/// value and the continuation of the `SessionMpst<S1, S2, R>` or an error.
pub fn recv_mpst_a_to_b<T, S1, S2, R>(
    s: SessionMpst<Recv<T, S1>, S2, RoleAtoB<R>>,
) -> ResultBoxError<T, S1, S2, R>
where
    T: marker::Send + 'static,
    S1: Session + 'static,
    S2: Session + 'static,
    R: Role + 'static,
{
    let (v, new_session) = recv(s.session1)?;
    let new_queue = next_a_to_b(s.stack);
    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        stack: new_queue,
    };

    Ok((v, result))
}

/// Receive a value of type `T` on B from A. Can fail. Returns either a pair of the received
/// value and the continuation of the `SessionMpst<S1, S2, R>` or an error.
pub fn recv_mpst_b_to_a<T, S1, S2, R>(
    s: SessionMpst<Recv<T, S1>, S2, RoleBtoA<R>>,
) -> ResultBoxError<T, S1, S2, R>
where
    T: marker::Send + 'static,
    S1: Session + 'static,
    S2: Session + 'static,
    R: Role + 'static,
{
    let (v, new_session) = recv(s.session1)?;
    let new_queue = next_b_to_a(s.stack);
    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        stack: new_queue,
    };

    Ok((v, result))
}

/// Receive a value of type `T` on C from A. Can fail. Returns either a pair of the received
/// value and the continuation of the `SessionMpst<S1, S2, R>` or an error.
pub fn recv_mpst_c_to_a<T, S1, S2, R>(
    s: SessionMpst<Recv<T, S1>, S2, RoleCtoA<R>>,
) -> ResultBoxError<T, S1, S2, R>
where
    T: marker::Send + 'static,
    S1: Session + 'static,
    S2: Session + 'static,
    R: Role + 'static,
{
    let (v, new_session) = recv(s.session1)?;
    let new_queue = next_c_to_a(s.stack);
    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        stack: new_queue,
    };

    Ok((v, result))
}

/// Receive a value of type `T` on A from C. Can fail. Returns either a pair of the received
/// value and the continuation of the `SessionMpst<S1, S2, R>` or an error.
pub fn recv_mpst_a_to_c<T, S1, S2, R>(
    s: SessionMpst<S1, Recv<T, S2>, RoleAtoC<R>>,
) -> ResultBoxError<T, S1, S2, R>
where
    T: marker::Send + 'static,
    S1: Session + 'static,
    S2: Session + 'static,
    R: Role + 'static,
{
    let (v, new_session) = recv(s.session2)?;
    let new_queue = next_a_to_c(s.stack);
    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        stack: new_queue,
    };

    Ok((v, result))
}

/// Receive a value of type `T` on B from C. Can fail. Returns either a pair of the received
/// value and the continuation of the `SessionMpst<S1, S2, R>` or an error.
pub fn recv_mpst_b_to_c<T, S1, S2, R>(
    s: SessionMpst<S1, Recv<T, S2>, RoleBtoC<R>>,
) -> ResultBoxError<T, S1, S2, R>
where
    T: marker::Send + 'static,
    S1: Session + 'static,
    S2: Session + 'static,
    R: Role + 'static,
{
    let (v, new_session) = recv(s.session2)?;
    let new_queue = next_b_to_c(s.stack);
    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        stack: new_queue,
    };

    Ok((v, result))
}

/// Receive a value of type `T` on C from B. Can fail. Returns either a pair of the received
/// value and the continuation of the `SessionMpst<S1, S2, R>` or an error.
pub fn recv_mpst_c_to_b<T, S1, S2, R>(
    s: SessionMpst<S1, Recv<T, S2>, RoleCtoB<R>>,
) -> ResultBoxError<T, S1, S2, R>
where
    T: marker::Send + 'static,
    S1: Session + 'static,
    S2: Session + 'static,
    R: Role + 'static,
{
    let (v, new_session) = recv(s.session2)?;
    let new_queue = next_c_to_b(s.stack);
    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        stack: new_queue,
    };

    Ok((v, result))
}
