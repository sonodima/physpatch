use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    /// Pattern to search for, in raw HEX string format and with '??' in place
    /// of wildcard bytes
    #[clap(last = true)]
    pub pattern: String,

    /// Data to write to all memory locations matched by the pattern, formatted
    /// as a HEX string
    ///
    /// If not specified or empty, the matches will only be printed and no further
    /// action will be performed
    #[clap(short, long, default_value = None)]
    pub patch: Option<String>,

    /// Name of the QEMU Virtual Machine to perform the scan on
    ///
    /// If not specified, the first instance found is utilized
    #[clap(short, long, default_value = None)]
    pub target: Option<String>,
}
