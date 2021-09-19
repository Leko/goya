mod build;
mod path_util;
mod repl;

use clap::Clap;
use morphological_analysis::ipadic::IPADic;
use path_util::PathUtil;
use std::fs;

#[derive(Clap)]
struct Opts {
    /// `~/.goya/dict` by default
    #[clap(short, long)]
    dicdir: Option<String>,
    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Clap)]
enum SubCommand {
    Compile(Compile),
}

/// A subcommand for controlling testing
#[derive(Clap)]
struct Compile {
    /// Path to the IPAdic directory
    dicpath: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    let base_dir = dirs::home_dir().unwrap().join(".goya");
    let dicdir = opts
        .dicdir
        .unwrap_or(base_dir.join("dict").to_str().unwrap().to_string());
    match opts.subcmd {
        Some(SubCommand::Compile(c)) => match build::build(&c.dicpath, &dicdir) {
            Ok(_) => {}
            Err(err) => {
                println!("{:?}", err);
            }
        },
        _ => {
            let util = PathUtil::from(dicdir);
            let encoded = fs::read(util.da_path()).expect("Failed to load dictionary");
            let da = bincode::deserialize(&encoded[..]).unwrap();

            let encoded = fs::read(util.dict_path()).expect("Failed to load vocabulary");
            let ipadic: IPADic = bincode::deserialize(&encoded[..]).unwrap();

            repl::start(&da, &ipadic)
        }
    }
}
