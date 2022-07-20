use std::path::{Path, PathBuf};
use base64ct::{Base64, Encoding};
use clap::Parser;
use glob::glob;
use sha2::{Digest, Sha256};
use sha2::digest::Output;
use std::{fs, io};
use std::collections::{HashMap};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, value_parser, default_value = ".")]
    base: String,

    #[clap(short, long, value_parser)]
    additional: Vec<String>,
}

fn main() {
    let args: Args = Args::parse();
    let parsed_source = Path::new(&args.base);
    let parsed_targets = &args.additional.iter().map(|path| Path::new(path)).collect::<Vec<&Path>>();
    if !parsed_source.exists() {
        panic!("source directory doesn't exist")
    }
    let non_existing_directories: Vec<&Path> = parsed_targets.iter().filter(|path| !path.exists()).map(|path| *path).collect();
    if !non_existing_directories.is_empty() {
        panic!("The supplied target directories {:?} don't exist", non_existing_directories)
    }
    println!("Comparing directory {} and {:?}", parsed_source.display(), parsed_targets);

    let mut all_files = HashMap::new();
    add_all_files_in_directory(parsed_source, &mut all_files);
    parsed_targets.iter().for_each(|parsed_target| add_all_files_in_directory(parsed_target, &mut all_files));
    let mapped: Vec<(&Output<Sha256>, &Vec<PathBuf>)> = all_files.iter()
        .filter(|(key, value)| !value.is_empty())
        .collect();

    println!("hash: {:?}", mapped);
}

fn add_all_files_in_directory(parsed_source: &Path, all_files: &mut HashMap<Output<Sha256>, Vec<PathBuf>>) {
    let mut string = parsed_source.display().to_string();
    string.push_str("/**/*");
    for entry in glob(string.as_str()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if path.is_dir() {
                    return;
                }
                let mut file = fs::File::open(&path).unwrap();
                let mut hasher = Sha256::new();
                io::copy(&mut file, &mut hasher).unwrap();
                let hash = hasher.finalize();
                let input = hash.as_ref();
                println!(" Discovered file: {:?} - {:?}", path.display(), Base64::encode_string(input));
                all_files.entry(hash).or_insert(Vec::new()).push(path);
            },
            Err(e) => println!("{:?}", e),
        }
    }
}
