fn main() {
    let string = "大家好";
    for ch in string.chars() {
        println!("Char: {}", ch);
    }
    let mut iter = string.char_indices();
    
    while let Some((idx, ch)) = iter.next() {
        println!("a: {}, b: {}", idx, ch);
        if idx == 3 { break }
    }
    println!("rem: {}", iter.as_str());
}
