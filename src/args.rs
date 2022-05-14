use bpaf::*;
use std::path::PathBuf;

// main args struct
#[derive(Clone, Debug)]
pub struct Opts {
    pub path: PathBuf,
}

// parse cli args
pub fn parse() -> Opts {
    // config path
    let path = short('c')
        .long("config")
        .help("Path to config file - defaults to ./cobalt.toml")
        .argument("PATH")
        .fallback("./cobalt.toml".to_string())
        .from_str();

    // combine parsers `speed` and `distance` parsers into a parser for Opts
    let parser = construct!(Opts { path });

    // define help message, attach it to parser, and run the results
    Info::default()
        .descr("cobalt - a simple reverse proxy by blobcode")
        .for_parser(parser)
        .run()
}
