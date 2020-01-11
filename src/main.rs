//! How to extract subcommands' args into external structs.
use crate::commands::Command;
use structopt::StructOpt;

mod commands;
mod git;
mod ser;
mod std;

#[derive(Debug, StructOpt)]
#[structopt(name = "wof", author, about)]
pub struct ApplicationArguments {
  #[structopt(subcommand)]
  pub command: Command,
}

fn main() {
  let opt = ApplicationArguments::from_args();

  opt.command.exec();
}
