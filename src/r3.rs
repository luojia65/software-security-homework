use std::str::CharIndices;
use std::iter::Peekable;

#[derive(Debug)]
pub enum Token<'a> {
    StringLiteral(&'a str),
    Number(&'a str),
    Word(&'a str),
    Symbol(&'a str),
}

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
        while let Some(&(_cur_idx, ' ')) = self.iter.peek() { 
            self.iter.next();
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

#[derive(Debug, Copy, Clone)]
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

    pub fn next_operator(&mut self) -> Option<(usize, Operator<'a>)> {
        self.skip_blanks();
        if let Some((idx, Token::Symbol(s))) = self.next() {
            match s {
                "+" | "-" | "*" | "/" | "->" | "," | "=" | "<" | ">" | "<=" | ">=" 
                    => return Some((idx, Operator { repr: s })),
                _ => return None,
            }
        }
        None
    }

    pub fn next_expr(&mut self) -> Option<(usize, Expr<'a>)> {
        self.skip_blanks();
        let (mut a, mut b, mut c) = (0, 0, 0);
        let begin = if let Some((_idx, Token::Symbol(s))) = self.next() {
            match s {
                "(" => a += 1,
                "[" => b += 1,
                "{" => c += 1,
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
        while let Some((idx, tt)) = self.next() {
            if let Token::Symbol(s) = tt {
                match s {
                    "(" => a += 1, ")" => a -= 1,
                    "[" => b += 1, "]" => b -= 1,
                    "{" => c += 1, "}" => c -= 1,
                    _ => {}
                }
            }
            if a == 0 && b == 0 && c == 0 {
                self.iter.next();
                return Some((begin, Expr { content: &self.src[begin..idx] }))
            }
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
            typ
        } else {
            return None;
        };
        let ident = if let Some((_idx, ident)) = self.iter.next_ident() {
            ident
        } else {
            return None;
        };
        let params = if let Some((_idx, expr)) = self.iter.next_expr() {
            expr
        } else {
            return None;
        };
        let content = if let Some((_idx, expr)) = self.iter.next_expr() {
            expr
        } else {
            return None;
        };
        Some(Function { ret_type, ident, params, content })
    }
}

fn called_functions<'a>(expr: Expr<'a>) -> Vec<&'a str> {
    let mut ans = Vec::new();
    let mut iter = Tokens {
        src: expr.content,
        iter: expr.content.char_indices().peekable()
    };
    while let Some((_idx, expr)) = iter.next() {
        let pk = iter.iter.peek();
        if let (Token::Word(w), Some((_, '('))) = (expr, pk) {
            ans.push(w)
        }
    }
    ans
}

// fn main() {
//     let a = "int main(int a, int b) { return 0; }";
//     let fns = Functions {
//         iter: Tokens {
//             src: a,
//             iter: a.char_indices().peekable()
//         }
//     };
//     for f in fns {
//         println!("{:?}", f)
//     }
// }

fn execute_r3(a: &str, b: &str) {

}
