use std::fs::read_to_string;
use std::collections::HashSet;

fn median(numbers: &mut [u64]) -> u64 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn gaus_sum(n: u64) -> u64{
    ((n * n) + n) /2
}

fn align_vec(crabs: Vec<u64>, median: u64) -> u64{
    crabs.iter().map(|x| (*x as i64 - median as i64).abs() as u64).sum()
}

fn align_vec_part2(crabs: Vec<u64>) -> u64{
    // create set of positions for unique iteration
    
    let mut unique_pos = HashSet::new();
    let crabs_clone = crabs.clone();
    for x in crabs_clone.iter(){
        unique_pos.insert(x);
    };

    unique_pos.iter().map(|v| {
        crabs.iter().map(|x| (gaus_sum((*x as i64 - **v as i64).abs() as u64))).sum()
    }).min().unwrap()
}

fn main() {
    let input_string = read_to_string("input.txt").expect("cannot read file");
    let mut crabs: Vec<u64> = input_string.split(",").map(|x| x.parse::<u64>().unwrap()).collect();

    // part 1 
    let median = median(&mut crabs);
    println!("median: {}", median);
    println!("aligned: {}", align_vec(crabs.clone(), median));
    println!("aligned part2: {}", align_vec_part2(crabs));

}
