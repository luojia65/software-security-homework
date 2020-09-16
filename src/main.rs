use std::path::Path;
use std::fs::File;
use std::io::Read;
use clap::{Arg, App, SubCommand};

mod r2;
mod r3;

fn main() {
    let matches = App::new("Software safety homework")
        .version("1.0")
        .author("Luo Jia <U201814857>")
        .subcommand(SubCommand::with_name("r2")
            .about("compare C code file using LCS algorithm")
            .version("1.0")
            .author("Luo Jia <U201814857>")
            .arg(Arg::with_name("A")
                .help("Sets the first input file to use")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("B")
                .help("Sets the second input file to use")
                .required(true)
                .takes_value(true)))
        .subcommand(SubCommand::with_name("r3")
            .about("compare C code file using call graph")
            .version("1.0")
            .author("Luo Jia <U201814857>")
            .arg(Arg::with_name("A")
                .help("Sets the first input file to use")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("B")
                .help("Sets the second input file to use")
                .required(true)
                .takes_value(true)))
        .get_matches();
    
    if let Some(matches) = matches.subcommand_matches("r2") {
        let file_a = matches.value_of("A").unwrap();
        let file_b = matches.value_of("B").unwrap();
        println!("Comparing files: {}, {}", file_a, file_b);
        
        let path = Path::new(&file_a);
        let content_a = if path.is_file() {
            let mut file = File::open(path).expect("open file");
            let mut content = String::new();
            file.read_to_string(&mut content).expect("read file");
            content
        } else { panic!("failed to open as file") };
        let path = Path::new(&file_b);
        let content_b = if path.is_file() {
            let mut file = File::open(path).expect("open file");
            let mut content = String::new();
            file.read_to_string(&mut content).expect("read file");
            content
        } else { panic!("failed to open as file") };

        r2::execute_r2(&content_a, &content_b);
    } else if let Some(matches) = matches.subcommand_matches("r3") { 
        let file_a = matches.value_of("A").unwrap();
        let file_b = matches.value_of("B").unwrap();
        println!("Comparing files: {}, {}", file_a, file_b);
        
        let path = Path::new(&file_a);
        let content_a = if path.is_file() {
            let mut file = File::open(path).expect("open file");
            let mut content = String::new();
            file.read_to_string(&mut content).expect("read file");
            content
        } else { panic!("failed to open as file") };
        let path = Path::new(&file_b);
        let content_b = if path.is_file() {
            let mut file = File::open(path).expect("open file");
            let mut content = String::new();
            file.read_to_string(&mut content).expect("read file");
            content
        } else { panic!("failed to open as file") };

        r3::execute_r3(&content_a, &content_b);
    }
}
