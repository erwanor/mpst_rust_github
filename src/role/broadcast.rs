use crossbeam_channel::{bounded, Receiver, Sender};

/// This structure is used to close an ordering or a name.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::End;
///
/// // Creating the binary sessions
/// type Close = End;
/// ```
#[derive(Debug)]
pub struct RoleBroadcast {
    pub sender: Sender<()>,
    pub receiver: Receiver<()>,
}

impl crate::role::Role for RoleBroadcast {
    type Dual = RoleBroadcast;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        (
            RoleBroadcast {
                sender: sender1,
                receiver: receiver2,
            },
            RoleBroadcast {
                sender: sender2,
                receiver: receiver1,
            },
        )
    }

    #[doc(hidden)]
    fn head_str() -> String {
        String::from("RoleBroadcast")
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        String::from("")
    }
}
