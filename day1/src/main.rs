use itertools::Itertools;
use std::fs::read_to_string;

fn main() {
    // part 1
    // read input to vec of lines
    let content = read_to_string("input.txt").expect("cannot read file");
    let vec_lines = content.split("\r\n").map(|s| s.parse::<i32>().unwrap());
    let i_count: i64 = vec_lines.clone().tuple_windows().map(|(a,b)| match a < b {
        true => 1,
        _ => 0
        }).sum();
    println!("{}", i_count);
    
    // part 2
    let i_count: i64 = vec_lines.tuple_windows::<(_,_,_)>().map(|(a,b,c)| a + b + c).tuple_windows().map(|(a,b)| match a < b {
        true => 1,
        _ => 0
        }).sum();
    println!("{}", i_count);
}
