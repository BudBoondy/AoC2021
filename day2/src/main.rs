use std::fs::read_to_string;

#[derive(Debug)]
enum Direction{
    UP(i64),
    DOWN(i64),
    FORWARD(i64),
}

fn enum_from_string(cmd: &str) -> Direction{
    let cmd: Vec<&str> = cmd.split(" ").collect();
    match cmd[0]{
        "up" => Direction::UP(cmd[1].parse::<i64>().unwrap()),
        "down" => Direction::DOWN(cmd[1].parse::<i64>().unwrap()),
        "forward" => Direction::FORWARD(cmd[1].parse::<i64>().unwrap()),
        _ => Direction::UP(0)
    }
}

struct Position {
    horizontal: i64,
    depth: i64,
    aim: i64

}


fn main() {
    // read input to vec of lines
    let content = read_to_string("input.txt").expect("cannot read file");
    let vec_lines: Vec<&str> = content.split("\n").collect();
    // part 1
    let mut  initial_position = Position{horizontal: 0, depth: 0, aim: 0};
    for item in &vec_lines{
       let dir = enum_from_string(item);
       match dir {
            Direction::UP(m) => initial_position.depth -= m, 
            Direction::DOWN(m) => initial_position.depth += m, 
            Direction::FORWARD(m) => initial_position.horizontal += m, 
       }
    }    
    println!("Position: {}", initial_position.horizontal * initial_position.depth); 

    // part 2
    let mut  initial_position = Position{horizontal: 0, depth: 0, aim: 0};
    for item in &vec_lines{
       let dir = enum_from_string(item);
       match dir {
            Direction::UP(m) => initial_position.aim -= m, 
            Direction::DOWN(m) => initial_position.aim += m, 
            Direction::FORWARD(m) => {
                initial_position.horizontal += m; 
                initial_position.depth += m * initial_position.aim;
            }, 
       }
    }    
    println!("Position: {}", initial_position.horizontal * initial_position.depth); 

}
