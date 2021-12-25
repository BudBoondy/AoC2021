use std::fs::read_to_string;
use std::collections::HashMap;
use std::char;

#[derive(Default, Debug, Clone, Copy)]
struct Segment{
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: bool,
    f: bool,
    g: bool,
}

impl Segment{
    fn new() -> Self {
        Default::default()
    }

    fn equals(self, other: Self) -> bool{
        return self.a == other.a && 
                self.b == other.b && 
                self.c == other.c && 
                self.d == other.d &&
                self.e == other.e &&
                self.f == other.f && 
                self.g == other.g
    }

    fn segment_from_string(char_array: Vec<char>) -> Segment{
        let mut seg = Segment::new();
        for c in char_array.into_iter(){
            match c{
                'a' => seg.a = true,
                'b' => seg.b = true,
                'c' => seg.c = true,
                'd' => seg.d = true,
                'e' => seg.e = true,
                'f' => seg.f = true,
                'g' => seg.g = true,
                _ => {}
            }
        }
        seg
    }
}
    fn difference(a: &str, other: &str) -> usize{
        let mut counter = a.len();
        let a_vec: Vec<char> = a.chars().collect();
        let other_vec: Vec<char> = other.chars().collect();
        for s in  a_vec.iter(){
            match other_vec.iter().find(|x| *x == s){
                Some(_) => {counter -= 1},
                None => ()
            }
        }
        counter
    }


fn decode(input: Vec<&str>, output: Vec<&str>) -> u64{
    let mut string_number: String = "".to_string();


    let v = get_remaining_digits(&input);
    println!("{:?}", v);
    let v: HashMap<&str, usize> = reverse_hashmap(v);
    println!("{:?}", v);
    for q in output.iter(){
        let a = Segment::segment_from_string(q.chars().collect());
        let (_, c) = v.iter().find(|(x, _)| Segment::segment_from_string(x.chars().collect()).equals(a)).unwrap(); 
        string_number.push(char::from_digit(*c as u32, 10).unwrap());
    }
    println!("{}", string_number);
    string_number.parse::<u64>().unwrap()
}

fn sum_digit_output(input: Vec<&str>) -> u64 {
    input.iter().map(|v| {let spl: Vec<&str> = v.split(" | ").collect(); decode(spl[0].split(" ").collect(), spl[1].split(" ").collect())}).sum() 
}

fn identify_unique_digits(input: Vec<&str>) -> usize{
    let mut counter = 0;
    for i in input.iter(){
        counter += i.split(" ").filter_map(|f| match f.len(){
            2 | 3 | 4 | 7 => Some(1),
            _ => None 
        }).count()
    }
    counter
}


fn reverse_hashmap(hm: HashMap<usize, &str>) -> HashMap<&str, usize> {
    let mut result_hashmap = HashMap::new();
    for x in hm.keys(){
        result_hashmap.insert(*hm.get(x).unwrap(), *x);
    }
    result_hashmap
}

fn get_remaining_digits<'a>(input_vec: &'a Vec<&str>) -> HashMap<usize, &'a str>{
    let mut new_hashmap = HashMap::new();
    let mut input = input_vec.clone();

    // get easy identified digits 
    let one_index = input.iter().position(|x| x.len() == 2).unwrap(); 
    new_hashmap.insert(1, input.remove(one_index));

    let seven_index = input.iter().position(|x| x.len() == 3).unwrap(); 
    new_hashmap.insert(7, input.remove(seven_index));
    
    let four_index = input.iter().position(|x| x.len() == 4).unwrap(); 
    new_hashmap.insert(4, input.remove(four_index));
    
    let eight_index = input.iter().position(|x| x.len() == 7).unwrap(); 
    new_hashmap.insert(8, input.remove(eight_index));

    // get six
    let six_index: usize  = input.iter().position(|x| x.len() == 6 && difference(new_hashmap.get(&1).unwrap(), x) == 1).unwrap();
    new_hashmap.insert(6, input.remove(six_index));
    // get nine
    let nine_index: usize = input.iter().position(|x| x.len() == 6 && difference(new_hashmap.get(&4).unwrap(), x) == 0).unwrap();
    new_hashmap.insert(9, input.remove(nine_index));
    // get zero
    let zero_index: usize = input.iter().position(|x| x.len() == 6).unwrap();
    new_hashmap.insert(0, input.remove(zero_index));
    // get five
    let five_index: usize = input.iter().position(|x| difference(x,new_hashmap.get(&6).unwrap()) == 0).unwrap();
    new_hashmap.insert(5, input.remove(five_index));
    // get three
    let three_index: usize = input.iter().position(|x| difference(x, new_hashmap.get(&9).unwrap()) == 0).unwrap();
    new_hashmap.insert(3, input.remove(three_index));
    // get two
    let two_index: usize = input.iter().position(|x| x.len() == 5).unwrap();
    new_hashmap.insert(2, input.remove(two_index));
    new_hashmap 
}

fn main() {
    let input_string = read_to_string("input.txt").expect("cannot read file");
    let segment_input: Vec<&str> = input_string.split("\n").collect(); //.map(|x| x.clone().to_string().split("|").collect::<Vec<&str>>()[1]).collect::<Vec<&str>>();
    let segment_output: Vec<&str> = segment_input.iter().map(|x| x.split(" | ").collect::<Vec<&str>>()[1]).collect();

    // part 1 
    println!("part1 {}", identify_unique_digits(segment_output.clone()));
    // part 2
    println!("part2 {}", sum_digit_output(segment_input));

}

#[cfg(test)]
mod tests{
    use super::*; 
    static TEST_INPUT: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_identify_unique_digits(){
        let segment_input: Vec<&str> = TEST_INPUT.split("\n").collect(); //.map(|x| x.clone().to_string().split("|").collect::<Vec<&str>>()[1]).collect::<Vec<&str>>();
        let segment_output: Vec<&str> = segment_input.iter().map(|x| x.split(" | ").collect::<Vec<&str>>()[1]).collect();
        assert_eq!(26, identify_unique_digits(segment_output.clone()));
    }

    #[test]
    fn test_sum_digit_output(){
        let input = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        assert_eq!(5353, sum_digit_output(vec![input]));
    }

    #[test]
    fn test_dict_decode(){
        let test_data: Vec<&str> = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf".split(" | ").collect();
        let test_in: &str = test_data[0];
        let test_out: &str = test_data[1];
        
        let num: u64 = decode(test_in.split(" ").collect(), test_out.split(" ").collect());
        assert_eq!(5353, num);
    }

    #[test]
    fn test_difference(){ 
        assert_eq!(1, difference("abcdefg", "abcdef"));
        assert_eq!(7, difference("abcdefg", ""));
    }

    #[test]
    fn test_identified_digits(){
        let test_data: Vec<&str> = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf".split(" | ").collect();
        let test_in: Vec<&str> = test_data[0].split(" ").collect();
        let v = get_remaining_digits(&test_in);
        assert!(v.contains_key(&1));
        assert!(v.contains_key(&2));
        assert!(v.contains_key(&3));
        assert!(v.contains_key(&4));
        assert!(v.contains_key(&5));
        assert!(v.contains_key(&6));
        assert!(v.contains_key(&7));
        assert!(v.contains_key(&8));
        assert!(v.contains_key(&9));
        assert!(v.contains_key(&0));
    }

}