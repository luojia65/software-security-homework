// use std::collections::HashMap;
use std::str::Chars;
use std::iter::Peekable;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use clap::{Arg, App, SubCommand};

#[derive(Debug, Eq, PartialEq)]
pub enum Item {
    Number(String),
    Word(String),
    Symbol(String),
    String(String),
}

pub struct Items<'a> {
    // alias: HashMap<&'a str, &'a str>,
    line_begin: bool,
    char_indices: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Items<'a> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&ch) = self.char_indices.peek() {
            if ch == '\r' || ch == '\n' {
                self.line_begin = true;
                self.char_indices.next();
                continue;
            }
            if ch == ';' || ch == ' ' || ch == '\t' {
                self.char_indices.next();
                continue;
            }
            if ch == '#' {
                // todo: #define, #ifdef, #ifndef
                // println!("{}", self.line_begin);
            }
            self.line_begin = false;
            if ch == '"' { // 字符串
                let mut ans = String::new();
                self.char_indices.next(); // 跳过引号
                while let Some(&ch_nxt) = self.char_indices.peek() {
                    if ch_nxt == '"' {
                        self.char_indices.next(); // 跳过引号
                        break;
                    }
                    ans.push(ch_nxt);
                    self.char_indices.next();
                }
                return Some(Item::String(ans))
            } else if char_is_number(ch) {
                let mut ans = String::new();
                while let Some(&ch_nxt) = self.char_indices.peek() {
                    if !char_is_number(ch_nxt) {
                        break;
                    }
                    ans.push(ch_nxt);
                    self.char_indices.next();
                }
                return Some(Item::Number(ans))
            } else if char_is_letter(ch) {
                let mut ans = String::new();
                while let Some(&ch_nxt) = self.char_indices.peek() {
                    if !char_is_letter(ch_nxt) {
                        break;
                    }
                    ans.push(ch_nxt);
                    self.char_indices.next();
                }
                return Some(Item::Word(ans))
            } else if char_is_symbol(ch) {
                let mut ans = String::new();
                while let Some(&ch_nxt) = self.char_indices.peek() {
                    if !char_is_symbol(ch_nxt) {
                        break;
                    }
                    ans.push(ch_nxt);
                    self.char_indices.next();
                }
                if ans == "//" { // 注释
                    while let Some(&ch_nxt) = self.char_indices.peek() {
                        if ch_nxt == '\n' {
                            break;
                        }
                        self.char_indices.next();
                    }
                    continue;
                }
                return Some(Item::Symbol(ans))
            } else {
                panic!("compile error! char: {}", ch);
            }
        }
        // 已经到结束了
        return None;
    }
}

fn char_is_letter(a: char) -> bool {
    (a >= 'A' && a <= 'Z') || (a >= 'a' && a <= 'z') || a == '_'
}

fn char_is_number(a: char) -> bool {
    a >= '0' && a <= '9'
}

fn char_is_symbol(a: char) -> bool {
    match a {
        '[' | ']' | '{' | '}' | '(' | ')' | 
        '-' | '+' | '*' | '/' | ',' | '=' | '<' | '>' => true,
        _ => false,
    }
}

pub fn items(input: &str) -> Items {
    Items { 
        char_indices: input.chars().peekable(),
        line_begin: true,
    }
}

fn execute_r2(a: &str, b: &str) {
    let a = items(a).collect::<Vec<_>>();
    let b = items(b).collect::<Vec<_>>();

    // println!("{:?}", a);

    // LCS算法第一步，得到子序列索引数组
    let (la, lb) = (a.len(), b.len());
    let mut dp = vec![0; (a.len() + 1) * (b.len() + 1)];
    for (i, ca) in a.iter().enumerate() {
        for (j, cb) in b.iter().enumerate() {
            if i > 1 && j > 1 && ca == cb {
                dp[(i + 1)*(lb + 1) + j + 1] = dp[i*(lb + 1) + j] + 1
            } else if i > 0 && j > 0 {
                dp[(i + 1)*(lb + 1) + j + 1] = usize::max(
                    dp[i*(lb + 1) + j + 1], 
                    dp[(i + 1)*(lb + 1) + j]
                );
            }
        }
    }
    // for i in 0..=la {
    //     println!("{:?}", &dp[i*(lb+1)..(i+1)*(lb+1)]);
    // }

    let mut diff = 0;

    // LCS第二步，得到有差别的元素。注意的是这个算法是从后往前倒着输出的
    let mut sa = a.iter().rev().peekable();
    let mut sb = b.iter().rev().peekable();
    let mut i = la;
    let mut j = lb;
    while let (Some(ca), Some(cb)) = (sa.peek(), sb.peek()) {
        if i == 0 && j == 0 {
            break
        }
        if ca == cb {
            diff += 1;
            sa.next();
            sb.next();
            i -= 1;
            j -= 1;
        } else {
            if dp[i*(lb + 1) + j - 1] > dp[(i-1)*(lb + 1) + j] {
                // println!("B: {:?}", cb);
                sb.next();
                j -= 1;
            } else {
                // println!("A: {:?}", ca);
                sa.next();
                i -= 1;
            }
        }
    }

    let rate = diff as f32 / a.len() as f32;
    println!("重复率：{}%", 100.0 * rate);
}

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

        execute_r2(&content_a, &content_b);
    }
}
