mod build;
mod path_util;
mod repl;

use clap::Clap;
use goya::double_array::DoubleArray;
use goya::word_set::WordSet;
use goya_ipadic::ipadic::IPADic;
use path_util::PathUtil;
use repl::Format;
use rkyv::{archived_root, Deserialize, Infallible};
use std::fs;

#[derive(Clap)]
struct Opts {
    /// `~/.goya/dict` by default
    #[clap(short, long)]
    dicdir: Option<String>,
    #[clap(short, long, default_value = "plain")]
    format: Format,
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
    let opts: Opts = Opts::parse();
    let base_dir = dirs::home_dir().unwrap().join(".goya");
    let dicdir = opts
        .dicdir
        .unwrap_or_else(|| base_dir.join("dict").to_str().unwrap().to_string());
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
            let archived = unsafe { archived_root::<DoubleArray>(&encoded[..]) };
            let da = archived.deserialize(&mut Infallible).unwrap();

            let encoded = fs::read(util.dict_path()).expect("Failed to load vocabulary");
            let archived = unsafe { archived_root::<IPADic>(&encoded[..]) };
            let ipadic: IPADic = archived.deserialize(&mut Infallible).unwrap();

            let encoded = fs::read(util.features_path()).expect("Failed to load surfaces");
            let archived = unsafe { archived_root::<WordSet>(&encoded[..]) };
            let word_set = archived.deserialize(&mut Infallible).unwrap();

            repl::start(repl::ReplContext {
                da: &da,
                dict: &ipadic,
                word_set: &word_set,
                format: opts.format,
            })
            .unwrap();
            std::thread::spawn(move || drop(ipadic));
            std::thread::spawn(move || drop(da));
            std::thread::spawn(move || drop(word_set));
        }
    }
}
