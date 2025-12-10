use clap::{Command, CommandFactory, ValueEnum};
use clap_complete::{Generator, Shell, generate};

use crate::{
    cli::{Cli, CompletionArgs},
    dispatch::Dispatcher,
};

fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    generate(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut std::io::stdout(),
    );
}

pub struct CompletionsDispatcher;
impl Dispatcher for CompletionsDispatcher {
    type Args = CompletionArgs;
    fn dispatch(_: crate::context::Context, args: Self::Args) -> crate::Result<()> {
        println!(
            "Generating completions for {}...\n========= \n",
            args.shell.to_possible_value().unwrap().get_name()
        );

        print_completions(Shell::from(args.shell), &mut Cli::command());
        Ok(())
    }
}
