use std::fs::read_to_string;

fn bit_criteria(s: &[&str], index: usize) -> (char, char){
    let count = s.iter().map(|x| (x.as_bytes()[index] as char).to_digit(10).unwrap()).sum::<u32>();
    match (count as i32) - ((s.len() as i32) - (count as i32)){
        x if x >= 0 => ('1', '0'),
        x if x < 0 => ('0', '1'),
        _ => ('0', '0')
    }
}

fn main() {
    // read input to vec of lines
    let content = read_to_string("input.txt").expect("cannot read file");
    let vec_lines: Vec<&str> = content.split("\r\n").collect();
    if vec_lines.is_empty(){
        return
    }
    //
    // part 1
    let mut gamma: String = "".to_string();
    let mut epsilon: String = "".to_string();
    
    let first: &str = vec_lines[0];
    for i in 0..first.len()-1{
        let count = vec_lines.iter().map(|x| (x.as_bytes()[i] as char).to_digit(10).unwrap()).sum::<u32>();
        match count < (vec_lines.len()-count as usize).try_into().unwrap(){
            true => {gamma += "1"; epsilon += "0";},
            false => {gamma += "0"; epsilon += "1";},
        }
    }
    let gamma_int = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_int = isize::from_str_radix(&epsilon, 2).unwrap();
    println!("PC: {}", gamma_int * epsilon_int);
    
    // part 2
    let mut vec_lines_oxy = vec_lines.clone();
    let mut index: usize = 0;
    while vec_lines_oxy.len() != 1 {
        let (lcb, _) = bit_criteria(&vec_lines_oxy, index.rem_euclid(first.len()));
        vec_lines_oxy = vec_lines_oxy.into_iter().filter(|x| (x.as_bytes()[index.rem_euclid(first.len())] as char) == lcb).collect();
        index += 1
    }
    let ogr: &str = vec_lines_oxy[0];
    // cO2
    let mut vec_lines_co2 = vec_lines.clone();
    let mut index: usize = 0;
    while vec_lines_co2.len() != 1 {
        let (_, nlcb) = bit_criteria(&vec_lines_co2, index.rem_euclid(first.len()));
        vec_lines_co2 = vec_lines_co2.into_iter().filter(|x| (x.as_bytes()[index.rem_euclid(first.len())] as char) == nlcb).collect();
        index += 1;
    }
    let co2: &str = vec_lines_co2[0];
    let oxy_int = isize::from_str_radix(ogr, 2).unwrap();
    let co2_int = isize::from_str_radix(co2, 2).unwrap();
    println!("LSR: {}", oxy_int * co2_int);
}
