mod extract;
mod help;

use clap::{command, Parser};

#[derive(Parser)]
#[command(name = "extract",about = "A CLI tools for extract file", long_about = None)]
struct Args{
    #[arg(short = 's', long = "source", help = "Path to the source file")]
    source: String,

    
}
fn main() {
    let args = Args::parse();
    let source = help::convert_relative_path_to_absolute_path(args.source.as_str()).unwrap();
    
    

    if source.ends_with(".zip") {
        let _ = extract::extract_archive_zip(source.as_str());
    }

    else if source.ends_with(".tar.gz") {
        let _ = extract::extract_tar_gz(source.as_str());
        
    }

    else {
        println!("Unsupported file format");
    }
    
}
