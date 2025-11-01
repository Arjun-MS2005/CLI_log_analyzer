use std::error::Error;
use std::fs;
use std::env;
use std::process;
use std::path::Path;
use log_analyzer::analyze_logs;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = build_file_path(&args).unwrap_or_else(|error_message|{
        println!("error occured while building file path: {error_message}");
        process::exit(1);
    });
    if let Err(e) = run(file_path){
        println!("Application Error: {e}");
        process::exit(1);
    };
}

fn build_file_path(args: &[String]) -> Result<String , &'static str>{
    if args.len() < 2{
        return Err("Missing File Path...");
    }

    let file_path = args[1].clone();
    let path = Path::new(&file_path);
    if !path.exists(){
        return Err("File path Does not exist");
    }
    return Ok(file_path);
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