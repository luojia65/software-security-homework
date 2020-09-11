// use std::collections::HashMap;
use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, Eq, PartialEq)]
pub enum Item {
    Number(String),
    Word(String),
    Symbol(String)
}

pub struct Items<'a> {
    // define: HashMap<&'a str, &'a str>,
    // line_begin: bool,
    char_indices: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Items<'a> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&ch) = self.char_indices.peek() {
            if ch == ';' || ch == ' ' || ch == '\r' || ch == '\n' {
                self.char_indices.next();
                continue;
            }
            if char_is_number(ch) {
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
                return Some(Item::Symbol(ans))
            } else {
                panic!("compile error!")
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
        '[' | ']' | '{' | '}' | '(' | ')' | '-' | '+' | '*' | '/' => true,
        _ => false,
    }
}

pub fn items(input: &str) -> Items {
    Items { 
        char_indices: input.chars().peekable()
    }
}

fn main() {
    let a = "11111 789 [999 ]123";
    let b = "12345 789 [aaaaa ] 123 456";
    // for i in items(a) {
    //     println!("{:?}", i);
    // }
    let a = items(a).collect::<Vec<_>>();
    let b = items(b).collect::<Vec<_>>();

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
            sa.next();
            sb.next();
            i -= 1;
            j -= 1;
        } else {
            if dp[i*(lb + 1) + j - 1] > dp[(i-1)*(lb + 1) + j] {
                // println!("B: {:?}", cb);
                diff += 1;
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
    println!("重复率：{}%", 100.0 * (1.0 - rate));
}
