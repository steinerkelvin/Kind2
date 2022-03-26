pub use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
  #[clap(subcommand)]
  pub command: CliCmd,
}

#[derive(Subcommand)]
pub enum CliCmd {
  /// Compile a Kind2 file
  Compile {
    /// Input file
    file: String,
  },
  /// Run and a Kind2 file (interpreted)
  Run { 
    /// Input file
    file: String,
    #[clap(short, long)]
    debug: bool,
  },
}
