use clap::Parser;

// TODO: maybe add an "offset" property that is added to the match address
// before applying the patch.

/// Scan the entire kernel space of a Windows VM for a pattern and replace the
/// matches with a patch.
#[derive(Parser, Debug)]
#[clap(author, version, about, verbatim_doc_comment)]
pub struct Args {
    /// Pattern to search for in the memory
    ///
    /// Formats:
    /// - [IDA-style] Single bytes separated by spaces, with wildcard bytes represented by [?] or [??] (e.g. "55 8B EC ? ? 8B 45 08")
    /// - [HEX-raw] Non-spaced bytes with wildcard bytes represented by [??] (e.g. "558bec????8b4508")
    #[clap(last = true, verbatim_doc_comment)]
    pub pattern: String,

    /// Bytes to write to memory locations matched by the pattern
    ///
    /// This must be a valid Hex string, and it may contain spaces to split byte paris.
    ///
    /// If unspecified or empty, the memory locations will only be printed to
    /// the screen and no further action will be performed.
    #[clap(short, long, default_value = None, verbatim_doc_comment)]
    pub patch: Option<String>,

    /// Name of the QEMU Virtual Machine to scan
    ///
    /// If unspecified, the first instance found is utilized.
    #[clap(short = 'g', long, default_value = None, verbatim_doc_comment)]
    pub target: Option<String>,

    /// Number of worker threads used to perform signature scans
    ///
    /// Leave this parameter unspecified or set to 0 to use all available cores.
    /// If set to a value greater than the number of available cores, the execution will fail.
    #[clap(short, long, default_value = None, verbatim_doc_comment)]
    pub threads: Option<usize>,

    /// Output raw text instead of the pretty-printed colored output with emojis
    #[clap(short, long)]
    pub raw_output: bool,
}
