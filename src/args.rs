use bpaf::*;
use std::path::PathBuf;
#[derive(Clone, Debug)]
pub struct Opts {
    pub path: PathBuf,
}

pub fn parse() -> Opts {
    let path = short('c')
        .long("config")
        .help("Path to config file")
        .argument("PATH")
        .from_str();

    // combine parsers `speed` and `distance` parsers into a parser for Opts
    let parser = construct!(Opts { path });

    // define help message, attach it to parser, and run the results
    Info::default()
        .descr("a simple reverse proxy")
        .for_parser(parser)
        .run()
}
