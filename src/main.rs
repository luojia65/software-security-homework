use std::path::Path;
use std::fs::{OpenOptions, File};
use std::io::Read;
use clap::{Arg, App, SubCommand};

mod r2;
mod r3;
mod r4;
mod r5;
mod a1;
mod b2;
mod b3;
mod b4;
mod b5;
mod gen;

fn main() {
    let matches = App::new("Software safety homework")
        .version("1.0")
        .author("Luo Jia <U201814857>")
        .subcommand(SubCommand::with_name("r2")
            .about("compare C code file using LCS algorithm")
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
            .arg(Arg::with_name("A")
                .help("Sets the first input file to use")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("B")
                .help("Sets the second input file to use")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("File")
                .help("Sets the CFG output file")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("a1")
            .about("compare C code file and Rust code file")
            .arg(Arg::with_name("A")
                .help("Sets the first input file to use")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("B")
                .help("Sets the second input file to use")
                .required(true)
                .takes_value(true)))
        .subcommand(SubCommand::with_name("r4")
            .about("check stack overflow")
            .arg(Arg::with_name("A")
                .help("Sets the input file to use")
                .required(true)
                .takes_value(true)))
        .subcommand(SubCommand::with_name("r5")
            .about("check string format vulnerbilities")
            .arg(Arg::with_name("A")
                .help("Sets the input file to use")
                .required(true)
                .takes_value(true)))
        .subcommand(SubCommand::with_name("b2")
            .about("check heap overflow")
            .arg(Arg::with_name("A")
                .help("Sets the input file to use")
                .required(true)
                .takes_value(true)))
        .subcommand(SubCommand::with_name("b3")
            .about("check for integer width overflow")
            .arg(Arg::with_name("A")
                .help("Sets the input file to use")
                .required(true)
                .takes_value(true)))
        .subcommand(SubCommand::with_name("b4")
            .about("check for integer arithmetic overflow")
            .arg(Arg::with_name("A")
                .help("Sets the input file to use")
                .required(true)
                .takes_value(true)))
        .subcommand(SubCommand::with_name("b5")
            .about("check null pointer unreferences")
            .arg(Arg::with_name("A")
                .help("Sets the input file to use")
                .required(true)
                .takes_value(true)))
        .subcommand(SubCommand::with_name("gen")
            .about("generate vulnerable samples"))
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

        let ans = r3::execute_r3(&content_a, &content_b);
        if let Some(out_path) = matches.value_of("File") {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(out_path).expect("open output file");
            for (a, b) in ans {
                use std::io::Write;
                file.write(a.as_bytes()).unwrap();
                file.write(&[b'\n']).unwrap();
                file.write(b.as_bytes()).unwrap();
                file.write(&[b'\n']).unwrap();
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("a1") { 
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

        a1::compare_language(&content_a, &content_b);
    } else if let Some(matches) = matches.subcommand_matches("r4") { 
        let file_a = matches.value_of("A").unwrap();
        println!("Using file: {}", file_a);
        
        let path = Path::new(&file_a);
        let content_a = if path.is_file() {
            let mut file = File::open(path).expect("open file");
            let mut content = String::new();
            file.read_to_string(&mut content).expect("read file");
            content
        } else { panic!("failed to open as file") };

        r4::execute_r4(&content_a);
    } else if let Some(matches) = matches.subcommand_matches("r5") { 
        let file_a = matches.value_of("A").unwrap();
        println!("Using file: {}", file_a);
        
        let path = Path::new(&file_a);
        let content_a = if path.is_file() {
            let mut file = File::open(path).expect("open file");
            let mut content = String::new();
            file.read_to_string(&mut content).expect("read file");
            content
        } else { panic!("failed to open as file") };

        r5::execute_r5(&content_a);
    } else if let Some(matches) = matches.subcommand_matches("b2") { 
        let file_a = matches.value_of("A").unwrap();
        println!("Using file: {}", file_a);
        
        let path = Path::new(&file_a);
        let content_a = if path.is_file() {
            let mut file = File::open(path).expect("open file");
            let mut content = String::new();
            file.read_to_string(&mut content).expect("read file");
            content
        } else { panic!("failed to open as file") };

        b2::execute_b2(&content_a);
    } else if let Some(matches) = matches.subcommand_matches("b3") { 
        let file_a = matches.value_of("A").unwrap();
        println!("Using file: {}", file_a);
        
        let path = Path::new(&file_a);
        let content_a = if path.is_file() {
            let mut file = File::open(path).expect("open file");
            let mut content = String::new();
            file.read_to_string(&mut content).expect("read file");
            content
        } else { panic!("failed to open as file") };

        b3::execute_b3(&content_a);
    } else if let Some(matches) = matches.subcommand_matches("b4") { 
        let file_a = matches.value_of("A").unwrap();
        println!("Using file: {}", file_a);
        
        let path = Path::new(&file_a);
        let content_a = if path.is_file() {
            let mut file = File::open(path).expect("open file");
            let mut content = String::new();
            file.read_to_string(&mut content).expect("read file");
            content
        } else { panic!("failed to open as file") };

        b4::execute_b4(&content_a);
    } else if let Some(matches) = matches.subcommand_matches("b5") { 
        let file_a = matches.value_of("A").unwrap();
        println!("Using file: {}", file_a);
        
        let path = Path::new(&file_a);
        let content_a = if path.is_file() {
            let mut file = File::open(path).expect("open file");
            let mut content = String::new();
            file.read_to_string(&mut content).expect("read file");
            content
        } else { panic!("failed to open as file") };

        b5::execute_b5(&content_a);
    }
}
