mod build;
mod path_util;
mod repl;

use clap::Clap;
use env_logger::Builder;
use morphological_analysis::ipadic::IPADic;
use path_util::PathUtil;
use std::fs;
use std::io::Write;

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
    Clean,
}

/// A subcommand for controlling testing
#[derive(Clap)]
struct Compile {
    /// Path to the IPAdic directory
    dicpath: String,
}

fn main() {
    let mut log_builder = Builder::from_default_env();
    log_builder
        .format(|buf, record| writeln!(buf, "{} {}", record.level(), record.args()))
        .init();

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
        Some(SubCommand::Clean) => {
            let util = PathUtil::from(dicdir);
            fs::remove_file(util.da_path()).expect("Failed to delete file");
            fs::remove_file(util.dict_path()).expect("Failed to delete file");
        }
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
