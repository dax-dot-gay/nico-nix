use crate::{cli::StatusArgs, dispatch::Dispatcher};

pub struct StatusDispatcher;
impl Dispatcher for StatusDispatcher {
    type Args = StatusArgs;
    fn dispatch(context: crate::context::Context, args: Self::Args) -> crate::Result<()> {
        println!("{context:?}");
        Ok(())
    }
}
