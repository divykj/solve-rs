#![macro_use]

use clap::{AppSettings, Clap};
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

const HELP_TEMPLATE: &str = "\
{bin} {version}\n\
{about}\n\
\n\
{usage-heading}\n    {usage}\n\
\n\
{all-args}\
";

#[derive(Clap)]
#[clap(version = VERSION, about = DESCRIPTION)]
#[clap(help_template = HELP_TEMPLATE)]
#[clap(setting = AppSettings::ColoredHelp)]
pub(crate) struct Cli {
    #[clap(
        short,
        long,
        parse(from_os_str),
        about = "Solve maths expressions from a file"
    )]
    pub(crate) file: Option<PathBuf>,

    #[clap(multiple_values = true)]
    pub(crate) expressions: Vec<String>,
}
