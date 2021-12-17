use std::fs::read_to_string;
use std::collections::HashMap;

fn evo_step_fast(lantern_vec: HashMap<u8, u64>) -> HashMap<u8, u64>{
    let mut new_step: HashMap<u8, u64> = HashMap::new();
    for k in lantern_vec.keys(){
        match k {
             x if *x == 0 => {let c = new_step.entry(6).or_insert(0); *c += *lantern_vec.get(x).unwrap(); new_step.insert(8, *lantern_vec.get(x).unwrap());},
             x if *x == 7 => {let c = new_step.entry(*x-1).or_insert(0); *c += *lantern_vec.get(x).unwrap();},
             x if *x <= 8 => {new_step.insert(x-1, *lantern_vec.get(x).unwrap());},
             x => {println!("{}", x);}
        }
    }
    new_step
}

fn evo_step(lantern_vec: Vec<u8>) -> Vec<u8>{
    let mut new_population: Vec<u8> = Vec::new();
    let mut new_count: u64 = 0;
    for l in lantern_vec.into_iter(){
        match l {
             x if x == 0 => {new_population.push(6); new_count += 1},
             x if x <= 8 => new_population.push(x-1),
             x => {println!("{}", x);}
        } 
    }
    while new_count != 0 {
        new_population.push(8);
        new_count -= 1;
    }
    new_population
}

fn parse_hashmap(v: Vec<u8>) -> HashMap<u8, u64>{
    let mut hash: HashMap<u8, u64> = HashMap::new();
    for e in v.into_iter(){
        let p = hash.entry(e).or_insert(0);
        *p += 1;
    }
    hash
}

fn main() {
    // read text input
    let input_string = read_to_string("input.txt").expect("cannot read file");
    let mut lantern_pop: Vec<u8> = input_string.split(",").map(|x| x.parse::<u8>().unwrap()).collect();
    let mut part2_lantern_pop: HashMap<u8, u64> = parse_hashmap(input_string.split(",").map(|x| x.parse::<u8>().unwrap()).collect());
    // part 1 
    for _ in 0..80{
        lantern_pop = evo_step(lantern_pop);
    }
    println!("{}", lantern_pop.iter().count());
    // part 2 
    for _ in 0..256{
        part2_lantern_pop = evo_step_fast(part2_lantern_pop);
    }
    println!("{}", part2_lantern_pop.iter().map(|(x,y)| *y as i64).fold(0, |acc, x| acc + x));
}



#[cfg(test)]
mod tests{
    use super::*; 
    static test_input: &str = "3,4,3,1,2";

    
    fn vec_compare(va: &[u8], vb: &[u8]) -> bool {
        (va.len() == vb.len()) &&  // zip stops at the shortest
         va.iter()
           .zip(vb)
           .all(|(a,b)| a == b)
    }
    #[test]
    fn test_evo_step(){
        let mut val: Vec<u8> = test_input.split(",").map(|x| x.parse::<u8>().unwrap()).collect();
        val = evo_step(val);
        assert_eq!(vec_compare(&vec![2,3,2,0,1], &val), true);
        val = evo_step(val);
        println!("{:?}", val);
        assert_eq!(vec_compare(&vec![1,2,1,6,0,8], &val), true);
    } 

    #[test]
    fn test_evo_step_fast(){
        let mut val: HashMap<u8, u64> = parse_hashmap(test_input.split(",").map(|x| x.parse::<u8>().unwrap()).collect());
        println!("{:?}", val);
        val = evo_step_fast(val);
        println!("{:?}", val);
        assert_eq!(5, val.iter().map(|(x,y)| *y as i64).fold(0, |acc, x| acc + x));
        val = evo_step_fast(val);
        println!("{:?}", val);
        assert_eq!(6, val.iter().map(|(x,y)| *y as i64).fold(0, |acc, x| acc + x));
    } 

    #[test]
    fn test_after_eighty_days(){
        let mut val: Vec<u8> = test_input.split(",").map(|x| x.parse::<u8>().unwrap()).collect();
        for _ in 0..80{
            val = evo_step(val);
        }
        assert_eq!(5934, val.iter().count());
    }
    #[test]
    fn test_after_80_days(){
        let mut val: HashMap<u8, u64> = parse_hashmap(test_input.split(",").map(|x| x.parse::<u8>().unwrap()).collect());
        for _ in 0..80{
            val = evo_step_fast(val);
            println!("{:?}", val);
        }
        println!("{:?}", val);
        assert_eq!(5934, val.iter().map(|(x,y)| *y as i64).fold(0, |acc, x| acc + x));
    }
    #[test]
    fn test_after_256_days(){
        let mut val: HashMap<u8, u64> = parse_hashmap(test_input.split(",").map(|x| x.parse::<u8>().unwrap()).collect());
        for _ in 0..256{
            val = evo_step_fast(val);
        }
        assert_eq!(26984457539, val.iter().map(|(x,y)| *y as i64).fold(0, |acc, x| acc + x));
    }
    
}
