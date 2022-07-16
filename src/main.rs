use std::path::{Path, PathBuf};
use base64ct::{Base64, Encoding};
use clap::Parser;
use glob::glob;
use sha2::{Digest, Sha256};
use sha2::digest::Output;
use std::{fs,io};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, value_parser, default_value = ".")]
    source: String,

    #[clap(long, value_parser)]
    target: String,
}

fn main() {
    let args = Args::parse();
    let parsed_source = Path::new(&args.source);
    let parsed_target = Path::new(&args.target);
    if !parsed_source.exists() {
        panic!("source directory doesn't exist")
    }
    if !parsed_target.exists() {
        panic!("target directory doesn't exist")
    }
    println!("Comparing directory {} with {}", parsed_source.display(), parsed_target.display());

    let _all_source_files = get_all_files_in_directory(parsed_source);
    let _all_target_files = get_all_files_in_directory(parsed_target);
}

fn get_all_files_in_directory(parsed_source: &Path) -> Vec<(PathBuf, Output<Sha256>)> {
    let mut string = parsed_source.display().to_string();
    string.push_str("/**/*");
    let mut path_bufs = vec![];
    for entry in glob(string.as_str()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let mut file = fs::File::open(&path).unwrap();
                let mut hasher = Sha256::new();
                io::copy(&mut file, &mut hasher).unwrap();
                let hash = hasher.finalize();
                let input = hash.as_ref();
                println!("{:?} - {:?}", path.display(), Base64::encode_string(input));
                path_bufs.push((path, hash));
            },
            Err(e) => println!("{:?}", e),
        }
    }
    path_bufs
}
