use binary::{cancel, Session};
use functionmpst::recv::recv_mpst_a_to_b;
use functionmpst::recv::recv_mpst_a_to_c;
use functionmpst::recv::recv_mpst_b_to_a;
use functionmpst::recv::recv_mpst_b_to_c;
use functionmpst::recv::recv_mpst_c_to_a;
use functionmpst::recv::recv_mpst_c_to_b;
use functionmpst::OfferMpst;
use role::a_to_b::RoleAtoB;
use role::a_to_c::RoleAtoC;
use role::b_to_a::RoleBtoA;
use role::b_to_c::RoleBtoC;
use role::c_to_a::RoleCtoA;
use role::c_to_b::RoleCtoB;
use role::Role;
use sessionmpst::SessionMpst;
use std::error::Error;

/// Offer a choice to B from A (on its session field related to A)
/// between two `SessionMpst`, `SessionMpst<S1, S2, R1>` and `SessionMpst<S3, S4, R2>`.
pub fn offer_mpst_session_b_to_a<'a, S1, S2, S3, S4, S5, F, G, R1, R2, R3, U>(
    s: SessionMpst<OfferMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, S5, RoleBtoA<R3>>,
    f: F,
    g: G,
) -> Result<U, Box<dyn Error + 'a>>
where
    S1: Session,
    S2: Session,
    S3: Session,
    S4: Session,
    S5: Session,
    R1: Role,
    R2: Role,
    R3: Role,
    F: FnOnce(SessionMpst<S1, S2, R1>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(SessionMpst<S3, S4, R2>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, s) = recv_mpst_b_to_a(s)?;
    cancel(s);
    e.either(f, g)
}

/// Offer a choice to A from B (on its session field related to B)
/// between two `SessionMpst`, `SessionMpst<S1, S2, R1>` and `SessionMpst<S3, S4, R2>`.
pub fn offer_mpst_session_a_to_b<'a, S1, S2, S3, S4, S5, F, G, R1, R2, R3, U>(
    s: SessionMpst<OfferMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, S5, RoleAtoB<R3>>,
    f: F,
    g: G,
) -> Result<U, Box<dyn Error + 'a>>
where
    S1: Session,
    S2: Session,
    S3: Session,
    S4: Session,
    S5: Session,
    R1: Role,
    R2: Role,
    R3: Role,
    F: FnOnce(SessionMpst<S1, S2, R1>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(SessionMpst<S3, S4, R2>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, s) = recv_mpst_a_to_b(s)?;
    cancel(s);
    e.either(f, g)
}

/// Offer a choice to A from C (on its session field related to C)
/// between two `SessionMpst`, `SessionMpst<S1, S2, R1>` and `SessionMpst<S3, S4, R2>`.
pub fn offer_mpst_session_a_to_c<'a, S1, S2, S3, S4, S5, F, G, R1, R2, R3, U>(
    s: SessionMpst<S5, OfferMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, RoleAtoC<R3>>,
    f: F,
    g: G,
) -> Result<U, Box<dyn Error + 'a>>
where
    S1: Session,
    S2: Session,
    S3: Session,
    S4: Session,
    S5: Session,
    R1: Role,
    R2: Role,
    R3: Role,
    F: FnOnce(SessionMpst<S1, S2, R1>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(SessionMpst<S3, S4, R2>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, s) = recv_mpst_a_to_c(s)?;
    cancel(s);
    e.either(f, g)
}

/// Offer a choice to C from A (on its session field related to A)
/// between two `SessionMpst`, `SessionMpst<S1, S2, R1>` and `SessionMpst<S3, S4, R2>`.
pub fn offer_mpst_session_c_to_a<'a, S1, S2, S3, S4, S5, F, G, R1, R2, R3, U>(
    s: SessionMpst<OfferMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, S5, RoleCtoA<R3>>,
    f: F,
    g: G,
) -> Result<U, Box<dyn Error + 'a>>
where
    S1: Session,
    S2: Session,
    S3: Session,
    S4: Session,
    S5: Session,
    R1: Role,
    R2: Role,
    R3: Role,
    F: FnOnce(SessionMpst<S1, S2, R1>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(SessionMpst<S3, S4, R2>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, s) = recv_mpst_c_to_a(s)?;
    cancel(s);
    e.either(f, g)
}

/// Offer a choice to B from C (on its session field related to C)
/// between two `SessionMpst`, `SessionMpst<S1, S2, R1>` and `SessionMpst<S3, S4, R2>`.
pub fn offer_mpst_session_b_to_c<'a, S1, S2, S3, S4, S5, F, G, R1, R2, R3, U>(
    s: SessionMpst<S5, OfferMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, RoleBtoC<R3>>,
    f: F,
    g: G,
) -> Result<U, Box<dyn Error + 'a>>
where
    S1: Session,
    S2: Session,
    S3: Session,
    S4: Session,
    S5: Session,
    R1: Role,
    R2: Role,
    R3: Role,
    F: FnOnce(SessionMpst<S1, S2, R1>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(SessionMpst<S3, S4, R2>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, s) = recv_mpst_b_to_c(s)?;
    cancel(s);
    e.either(f, g)
}

/// Offer a choice to C from B (on its session field related to B)
/// between two `SessionMpst`, `SessionMpst<S1, S2, R1>` and `SessionMpst<S3, S4, R2>`.
pub fn offer_mpst_session_c_to_b<'a, S1, S2, S3, S4, S5, F, G, R1, R2, R3, U>(
    s: SessionMpst<S5, OfferMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, RoleCtoB<R3>>,
    f: F,
    g: G,
) -> Result<U, Box<dyn Error + 'a>>
where
    S1: Session,
    S2: Session,
    S3: Session,
    S4: Session,
    S5: Session,
    R1: Role,
    R2: Role,
    R3: Role,
    F: FnOnce(SessionMpst<S1, S2, R1>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(SessionMpst<S3, S4, R2>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, s) = recv_mpst_c_to_b(s)?;
    cancel(s);
    e.either(f, g)
}

/// Offer a choice at A from C between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! offer_mpst_a_to_c {
    ($session:expr, { $($pat:pat => $result:block,)* }) => {
        (move || -> Result<_, _> {
            let (l, s) = recv_mpst_a_to_c($session)?;
            cancel(s);
            match l {
                $(
                    $pat => { $result },
                )*
            }
        })()
    };
}

/// Offer a choice at B from C between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! offer_mpst_b_to_c {
    ($session:expr, { $($pat:pat => $result:block,)* }) => {
        (move || -> Result<_, _> {
            let (l, s) = recv_mpst_b_to_c($session)?;
            cancel(s);
            match l {
                $(
                    $pat => { $result },
                )*
            }
        })()
    };
}

/// Offer a choice at A from B between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! offer_mpst_a_to_b {
    ($session:expr, { $($pat:pat => $result:block,)* }) => {
        (move || -> Result<_, _> {
            let (l, s) = recv_mpst_a_to_b($session)?;
            cancel(s);
            match l {
                $(
                    $pat => { $result },
                )*
            }
        })()
    };
}

/// Offer a choice at B from A between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! offer_mpst_b_to_a {
    ($session:expr, { $($pat:pat => $result:block,)* }) => {
        (move || -> Result<_, _> {
            let (l, s) = recv_mpst_b_to_a($session)?;
            cancel(s);
            match l {
                $(
                    $pat => { $result },
                )*
            }
        })()
    };
}

/// Offer a choice at C from B between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! offer_mpst_c_to_b {
    ($session:expr, { $($pat:pat => $result:block,)* }) => {
        (move || -> Result<_, _> {
            let (l, s) = recv_mpst_c_to_b($session)?;
            cancel(s);
            match l {
                $(
                    $pat => { $result },
                )*
            }
        })()
    };
}

/// Offer a choice at C from A between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! offer_mpst_c_to_a {
    ($session:expr, { $($pat:pat => $result:block,)* }) => {
        (move || -> Result<_, _> {
            let (l, s) = recv_mpst_c_to_a($session)?;
            cancel(s);
            match l {
                $(
                    $pat => { $result },
                )*
            }
        })()
    };
}