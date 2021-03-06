use std::str::CharIndices;
use std::iter::Peekable;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub enum Token<'a> {
    StringLiteral(&'a str),
    Number(&'a str),
    Word(&'a str),
    Symbol(&'a str),
}

#[derive(Debug)]
pub struct Tokens<'a> {
    src: &'a str,
    iter: Peekable<CharIndices<'a>>,
}

impl<'a> Tokens<'a> {
    // pub fn as_str(&self) -> &'a str {
    //     self.src
    // }

    fn next_match_by<F, G>(&mut self, first_ok: F, content_ok: G, skip_start: bool, skip_end: bool) -> Option<(usize, &'a str)> 
    where
        F: Fn(char) -> bool,
        G: Fn(char) -> bool,
    {
        let start_idx = if let Some(&(cur_idx, cur_char)) = self.iter.peek() {
            if !first_ok(cur_char) {
                return None;
            }
            if skip_start {
                self.iter.next();
                if let Some(&(nxt_idx, _)) = self.iter.peek() {
                    nxt_idx
                } else {
                    return None;
                }
            } else {
                cur_idx
            }
        } else {
            return None;
        };
        let mut end_idx = start_idx;
        while let Some(&(cur_idx, cur_char)) = self.iter.peek() {
            end_idx = cur_idx;
            if !content_ok(cur_char) {
                break;
            }
            self.iter.next();
        }
        if self.iter.peek() == None {
            return Some((start_idx, &self.src[start_idx..]));
        }
        if skip_end {
            self.iter.next();
        }
        Some((start_idx, &self.src[start_idx..end_idx]))
    }

    pub fn next_word(&mut self) -> Option<(usize, &'a str)> {
        self.next_match_by(char_is_letter, char_is_letter_or_number, false, false)
    }
    
    pub fn next_number(&mut self) -> Option<(usize, &'a str)> {
        self.next_match_by(char_is_number, char_is_number, false, false)
    }
    
    pub fn next_string_literal(&mut self) -> Option<(usize, &'a str)> {
        self.next_match_by(|c| c == '"', |c| c != '"', true, true)
    }
    
    pub fn next_symbol(&mut self) -> Option<(usize, &'a str)> {
        self.next_match_by(char_is_symbol, char_is_symbol, false, false)
    }
    
    pub fn skip_blanks(&mut self) {
        while let Some(&(_cur_idx, ch)) = self.iter.peek() { 
            if ch == ' ' || ch == '\r' || ch == '\n' || ch == '\t' {
                self.iter.next();
            } else if ch == '/' {
                // dbg!(_cur_idx, ch, &self);
                self.iter.next();
                if let Some(&(_, nxt_ch)) = self.iter.peek() {
                    // println!("nxt_ch={}", nxt_ch);
                    if nxt_ch == '/' { // '//'
                        self.iter.next();
                        while let Some(&(_, ch)) = self.iter.peek() {
                            if ch == '\n' || ch == '\r' {
                                break;
                            }
                            self.iter.next();
                        }
                    }
                }
            } else {
                break
            }
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = (usize, Token<'a>);
    fn next(&mut self) -> Option<Self::Item> {
        self.skip_blanks();
        if let Some((idx, s)) = self.next_word() {
            return Some((idx, Token::Word(s)))
        }
        if let Some((idx, s)) = self.next_number() {
            return Some((idx, Token::Number(s)))
        }
        if let Some((idx, s)) = self.next_string_literal() {
            return Some((idx, Token::StringLiteral(s)))
        }
        if let Some((idx, s)) = self.next_symbol() {
            return Some((idx, Token::Symbol(s)))
        }
        return None;
    }
}

fn char_is_letter(a: char) -> bool {
    (a >= 'A' && a <= 'Z') || (a >= 'a' && a <= 'z') || a == '_'
}

fn char_is_number(a: char) -> bool {
    a >= '0' && a <= '9'
}

fn char_is_letter_or_number(a: char) -> bool {
    char_is_letter(a) || char_is_number(a)
}

fn char_is_symbol(a: char) -> bool {
    match a {
        '[' | ']' | '{' | '}' | '(' | ')' | 
        '-' | '+' | '*' | '/' | ',' | '=' | '<' | '>' | ';' => true,
        _ => false,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Type<'a> {
    name: &'a str,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Ident<'a> {
    name: &'a str,
}

#[derive(Debug, Copy, Clone)]
pub struct Operator<'a> {
    repr: &'a str,
}

#[derive(Debug, Copy, Clone)]
pub struct Expr<'a> {
    content: &'a str,
}

impl<'a> Tokens<'a> {
    pub fn next_type(&mut self) -> Option<(usize, Type<'a>)> {
        self.skip_blanks();
        if let Some((idx, Token::Word(w))) = self.next() {
            return Some((idx, Type { name: w }))
        }
        None
    }

    pub fn next_ident(&mut self) -> Option<(usize, Ident<'a>)> {
        self.skip_blanks();
        if let Some((idx, Token::Word(w))) = self.next() {
            return Some((idx, Ident { name: w }))
        }
        None
    }

    // pub fn next_operator(&mut self) -> Option<(usize, Operator<'a>)> {
    //     self.skip_blanks();
    //     if let Some((idx, Token::Symbol(s))) = self.next() {
    //         match s {
    //             "+" | "-" | "*" | "/" | "->" | "," | "=" | "<" | ">" | "<=" | ">=" 
    //                 => return Some((idx, Operator { repr: s })),
    //             _ => return None,
    //         }
    //     }
    //     None
    // }

    pub fn next_expr(&mut self) -> Option<(usize, Expr<'a>)> {
        self.skip_blanks();
        let (mut a, mut b, mut c) = (0, 0, 0);
        let begin = if let Some((idx, Token::Symbol(s))) = self.next() {
            match s {
                "(" => a += 1,
                "[" => b += 1,
                "{" => c += 1,
                "()" => return Some((idx, Expr { content: "" })),
                _ => {}
            }
            if let Some(&(idx, _)) = self.iter.peek() {
                idx
            } else {
                return None;
            }
        } else {
            return None;
        };
        self.skip_blanks();
        // println!("peek: {:?}", self.iter.peek());
        while let Some((idx, tt)) = self.next() {
            // println!("{:?} {:?}", idx, tt);
            if let Token::Symbol(s) = tt {
                match &s[..=0] {
                    "(" => a += 1, ")" => a -= 1,
                    "[" => b += 1, "]" => b -= 1,
                    "{" => c += 1, "}" => c -= 1,
                    _ => {}
                }
            }
            // println!("a={} b={}, c={}", a, b, c);
            if a == 0 && b == 0 && c == 0 {
                self.iter.next();
                return Some((begin, Expr { content: &self.src[begin..idx] }))
            }
            self.skip_blanks();
        }
        None
    }
}

#[derive(Debug)]
pub struct Function<'a> {
    ret_type: Type<'a>,
    ident: Ident<'a>,
    params: Expr<'a>,
    content: Expr<'a>,
}

pub struct Functions<'a> {
    iter: Tokens<'a>,
}

impl<'a> Iterator for Functions<'a> {
    type Item = Function<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let ret_type = if let Some((_idx, typ)) = self.iter.next_type() {
            // println!("type: {:?}", typ);
            typ
        } else {
            return None;
        };
        let ident = if let Some((_idx, ident)) = self.iter.next_ident() {
            // println!("ident: {:?}", ident);
            ident
        } else {
            return None;
        };
        let params = if let Some((_idx, expr)) = self.iter.next_expr() {
            // println!("params: {:?}", expr);
            expr
        } else {
            return None;
        };
        let content = if let Some((_idx, expr)) = self.iter.next_expr() {
            // println!("content: {:?}", expr);
            expr
        } else {
            return None;
        };
        Some(Function { ret_type, ident, params, content })
    }
}

fn tokens(a: &str) -> Tokens {
    Tokens {
        src: a,
        iter: a.char_indices().peekable()
    }
}

pub struct Lines<'a> {
    expr: Expr<'a>,
    iter: CharIndices<'a>,
    start: usize,
}

pub struct Line<'a> {
    idx: usize, 
    content: &'a str,
}

impl<'a> Iterator for Lines<'a> {
    type Item = Line<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((nxt_idx, nxt_chr)) = self.iter.next() {
            if nxt_chr == ';' {
                let ans = &self.expr.content[self.start..=nxt_idx];
                let idx = self.start;
                self.start = nxt_idx + 1;
                return Some(Line { idx, content: ans.trim() });
            }
        }
        None
    } 
}

fn lines(a: Expr) -> Lines {
    Lines {
        expr: a,
        iter: a.content.char_indices(),
        start: 0,
    }
}

fn line_number_from_line_idx(a: &str, i: usize) -> (usize, usize) {
    let mut ln = 1;
    let mut col = 0;
    let a = if i >= a.len() { a } else { &a[..i] };
    for ch in a.chars() {
        col += 1;
        if ch == '\n' {
            col = 0;
            ln += 1;
        }
    }
    (ln, col)
}

pub fn execute_r4(a: &str) {
    let mut type_size = HashMap::new();
    type_size.insert("int", 8);
    type_size.insert("char", 1);
    // println!("a: {}", a);
    let fns = Functions { iter: tokens(a) };
    for f in fns {
        let mut var_size = HashMap::new();
        // println!("Function: {:?}", f);
        // 扫描所有行，得到变量和它的内存占用大小
        for line in lines(f.content) {
            let mut tk = tokens(line.content);
            let mut size = 0;
            if let Some((_idx, Token::Word(word))) = tk.next() {
                if type_size.contains_key(&word) {
                    // println!("word: {}", word);
                    size = type_size[word];
                } else {
                    continue
                }
            } 
            let var_name = if let Some((_idx, Token::Word(word))) = tk.next() {
                word
            } else {
                continue
            };
            let nxt = if let Some(a) = tk.next() { a } else { continue; };
            if nxt.1 == Token::Symbol("[") {
                // 数组
                let num = if let Some((_, t)) = tk.next() { t } else { continue };
                if let Token::Number(n) = num { 
                    let digit: i32 = n.parse().unwrap();
                    size *= digit;
                }
                tk.next(); // skip ']'
            } else if nxt.1 == Token::Symbol("=") {
                // 单个变量
                // 这里什么也不做
            }
            // println!("{}: size = {}", var_name, size);
            // println!("line: {:?}", line.content);
            var_size.insert(var_name, size);
        }
        for (idx, token) in tokens(a) {
            if let Token::Word(w) = token {
                match w {
                    "strcpy" | "strncpy" | "memcpy" | "memncpy" | "strcat" | "strncat" | 
                    "sprintf" | "vsprintf" | "gets" | "getchar" | "fgetc" | "getc" | 
                    "read" | "sscanf" | "fscanf" | "vfscanf" | "vscanf" | "vsscanf" => {
                        let (l, c) = line_number_from_line_idx(a, idx);
                        println!("possible senstive function {} at line {}, col {} !", w, l, c);
                    }
                    _ => {}
                }
            }
        }
        // 扫描所有行，得到使用这些变量的情况
        for line in lines(f.content) {
            let mut tok = tokens(line.content);
            while let Some((_idx, tk)) = tok.next() {
                match tk {
                    Token::Word("strncpy") => {
                        tok.next(); // (
                        let (_, dest) = if let Some(a) = tok.next() { a } else { continue }; // param 1: dest
                        tok.next(); // ,
                        tok.next(); // param 2: src
                        tok.next(); // ,
                        let (_, cnt) = if let Some(a) = tok.next() { a } else { continue };
                        if let (Token::Word(d), Token::Number(n)) = (dest, cnt) {
                            let size = if let Some(&val) = var_size.get(d) { val } else { continue };
                            let digit: i32 = n.parse().unwrap();
                            if size < digit {
                                let (l, c) = line_number_from_line_idx(line.content, line.idx);
                                println!("stack overflow for strncpy at line {}, col {}", l, c);
                            }
                        }
                    },
                    Token::Word("strcpy") => {
                        tok.next(); // (
                        let (_, dest) = if let Some(a) = tok.next() { a } else { continue }; // param 1: dest
                        tok.next(); // ,
                        let (_, src) = if let Some(a) = tok.next() { a } else { continue }; // param 2: src
                        if let (Token::Word(d), Token::StringLiteral(s)) = (dest, src) {
                            let size = if let Some(&val) = var_size.get(d) { val } else { continue };
                            if (size as usize) < s.len() {
                                let (l, c) = line_number_from_line_idx(line.content, line.idx);
                                println!("stack overflow for strcpy at line {}, col {}", l, c);
                            }
                        }
                    },
                    _ => continue,
                }
            }
        }
    }
}
