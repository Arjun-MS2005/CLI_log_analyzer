use std::error::Error;
use std::fs;
use log_analyzer::analyze_logs;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args{
    file_path: String,
}


fn main() -> Result<() , Box<dyn Error>> {
    let args = Args::parse();
    let file_path = args.file_path;
    run(file_path)?;

    Ok(())
}


fn run(file_path : String) -> Result<() , Box<dyn Error>>{
    let contents = fs::read_to_string(file_path)?;
    let map = analyze_logs(&contents);
    println!("-------------------------LOGS------------------------------\n");
    for (key , value) in &map{
        println!("{:<8} | {}", key, value);
    }
    Ok(())

}