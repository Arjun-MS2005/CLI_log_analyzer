use std::{error::Error};
use std::fs;
use log_analyzer::{analyze_logs , Level};
use clap::{Parser};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args{
    file_path: String,
    #[arg(short , long , ignore_case = true)]
    level: Option<Level>,
}


fn main() -> Result<() , Box<dyn Error>> {
    let args = Args::parse();
    let file_path = args.file_path;
    run(file_path , &args.level)?;

    Ok(())
}

fn run(file_path : String , level_filter: &Option<Level>) -> Result<() , Box<dyn Error>>{
    let contents = fs::read_to_string(file_path)?;
    let map = analyze_logs(&contents);

    println!("-------------------------LOGS------------------------------\n");
    if let Some(level) = level_filter {
        let key = match level{
            Level::Error => "ERROR",
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Warn => "WARN"  
        };
        let count = map.get(key).unwrap_or(&0);
        println!("{:<8} | {}" , key , count);
    }
    else{
        let all_levels = ["ERROR" , "WARN" , "DEBUG" , "INFO"];
        for key in all_levels{
            let count = map.get(key).unwrap_or(&0);
            println!("{:<8} | {}" , key , count);
        }
    }

    Ok(())
}