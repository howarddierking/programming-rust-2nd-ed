use text_colorizer::*;
use std::env;
use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn print_usage(){
    eprintln!("{} - change occurrences of one string into another", 
              "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

fn parse_arguments() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!("{} wrong number of arguments: expected 4, got {}.",
            "Error".red().bold(),
            args.len());
        std::process::exit(1);
    }

    // question: do we need to clone here because the args vector gets 
    // cleaned up with this function exits?
    Arguments {
        target: args[0].clone(),    
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;    // question: what is the question mark syntax again?
    Ok(regex.replace_all(text, replacement).to_string())
}

fn main() {
    let args = parse_arguments();
    
    let data = match fs::read_to_string(&args.filename){
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to read from file '{}': {:?}",
                "Error".red().bold(),
                args.filename,  // dumb question here, but why not pass the reference like above?
                e);
            std::process::exit(1);
        }
    };

    let replaced_data = match replace(&args.target, &args.replacement, &data){
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to replace text {:?}",
                "Error".red().bold(),
                e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &replaced_data){
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} failed to write the file '{}': {:?}",
                "Error".red().bold(),
                args.output,
                e);
            std::process::exit(1);
        }
    };
}
