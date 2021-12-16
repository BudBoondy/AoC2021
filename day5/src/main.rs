use std::fs::read_to_string;
extern crate ndarray;

use ndarray::Array2;

#[derive(Debug, Clone)]
struct Point{
    x: usize,
    y: usize
}

fn find_board_size(line_segment: &Vec<&str>) -> (u64, u64){
    let mut x = 0;
    let mut y = 0;
    for l in line_segment.iter(){
        let l_split: Vec<&str> = l.split(" -> ").collect();
        for t in l_split.iter(){
            let xy_t: Vec<&str> = t.split(",").collect();
            if xy_t[0].parse::<u64>().unwrap() > x{
                x = xy_t[0].parse::<u64>().unwrap();
            }
            if xy_t[1].parse::<u64>().unwrap() > y{
                y = xy_t[1].parse::<u64>().unwrap();
            }
        }
    }
    (x+1, y+1)
}

fn generate_board(widht: usize, height: usize) -> Array2<u64>{
    Array2::<u64>::zeros((widht, height))
}

fn to_coordinates(coords: Vec<&str>) -> Point{
    Point{x: coords[0].parse::<usize>().unwrap(), y: coords[1].parse::<usize>().unwrap()}
}

fn parse_coordinates(pair: &str) -> Point {
    to_coordinates(pair.split(",").collect())
}

fn generate_intermediate_points(p1: Point, p2: Point) -> Vec<Point> {
    let mut line_segment: Vec<Point> = Vec::new();
    // push first point
    let mut p1 = p1.clone();
    
    if p1.x == p2.x || p1.y == p2.y{
        return generate_intermediate_hv_points(p1, p2);
    }

    while p1.x != p2.x && p1.y != p2.y{
        line_segment.push(Point{x: p1.x, y: p1.y});
        match (p1.x as i64 - p2.x as i64, p1.y as i64- p2.y as i64){
            (x,y) if x < 0 && y < 0 => {p1.x += 1; p1.y += 1},
            (x,y) if x < 0 && y > 0 => {p1.x += 1; p1.y -= 1},
            (x,y) if x > 0 && y < 0 => {p1.x -= 1; p1.y += 1},
            (x,y) if x > 0 && y > 0 => {p1.x -= 1; p1.y -= 1},
            _ => {}
        }
    }
    // push last point
    line_segment.push(p2);
    line_segment
}

fn generate_intermediate_hv_points(p1: Point, p2: Point) -> Vec<Point>{
    let mut line_segment: Vec<Point> = Vec::new();
    // push first point
    let mut p1 = p1.clone();

    if p1.x != p2.x && p1.y != p2.y{
        return Vec::new();
    }

    while p1.x != p2.x{
        line_segment.push(Point{x: p1.x, y: p1.y});
        match p1.x as i64 - p2.x as i64{
            x if x < 0 => {p1.x += 1;},
            x if x > 0 => {p1.x -= 1;},
            _ => {}
        }
    }
    while p1.y != p2.y{
        line_segment.push(Point{x: p1.x, y: p1.y});
        match p1.y as i64 - p2.y as i64{
            x if x < 0 => {p1.y += 1;},
            x if x > 0 => {p1.y -= 1;},
            _ => {}
        }

    }
    // push last point
    line_segment.push(p2);
    line_segment
}

fn process_lines(lines: Vec<Point>, board: Array2<u64>) ->  Array2<u64>{
    let mut  board = board.clone();
    for l in lines.iter(){
        board[[l.x, l.y]] += 1;
    }
    board
}

fn line_segment(line_segment: Vec<&str>, board: Array2<u64>, part2: bool) -> Array2<u64> {
    let mut new_board: Array2<u64> = board;
    for l in line_segment.iter(){
        let l_split: Vec<&str> = l.split(" -> ").collect();
        let p_l = parse_coordinates(l_split[0]);
        let p_r = parse_coordinates(l_split[1]);
        let points: Vec<Point>;
        if part2{
            points = generate_intermediate_points(p_l, p_r); 
        } else {
            points = generate_intermediate_hv_points(p_l, p_r); 
        }
        new_board = process_lines(points, new_board);
    } 
    new_board
}

fn sum_board(board: Array2<u64>) -> usize{
    board.iter().filter(|x|  **x > 1_u64).count()
}

fn main() {
    // read text input
    let input_string = read_to_string("input.txt").expect("cannot read file");
    let line_s = input_string.split("\n").collect();
    let (widht, height) = find_board_size(&line_s);
    // part 1
    let l_s = line_segment(line_s.clone(), generate_board(widht.try_into().unwrap(), height.try_into().unwrap()), false);
    println!("sum {}", sum_board(l_s));

    // part 2
    let l_s = line_segment(line_s, generate_board(widht.try_into().unwrap(), height.try_into().unwrap()), true);
    println!("sum diagonal {}", sum_board(l_s));

}


#[cfg(test)]
mod tests{
    use super::*; 

    static test_input: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    #[test]
    fn test_find_board_size(){
        assert_eq!((10,10), find_board_size(&test_input.split("\n").collect()))
    }

    #[test]
    fn test_sum_board(){
        let line_input = test_input.split("\n").collect();
        let (widht, height) = find_board_size(&line_input);

        // part1
        let l_s = line_segment(line_input.clone(), generate_board(widht.try_into().unwrap(), height.try_into().unwrap()), false);
        assert_eq!(5, sum_board(l_s));

        // part2
        let l_s = line_segment(line_input, generate_board(widht.try_into().unwrap(), height.try_into().unwrap()), true);
        println!("{:?}", l_s);
        assert_eq!(12, sum_board(l_s));
    }

    fn vec_compare(va: &[Point], vb: &[Point]) -> bool {
        (va.len() == vb.len()) &&  // zip stops at the shortest
         va.iter()
           .zip(vb)
           .all(|(a,b)| a.x == b.x && a.y == b.y)
    }

    #[test]
    fn test_generate_intermediate_board(){
        let p1 = Point{x:1, y:1}; //1,1 -> 1,3
        let p2 = Point{x:1, y:3}; //1,1 -> 1,3
        let i_p = generate_intermediate_hv_points(p1, p2);
        assert_eq!(vec_compare(&vec![Point{x:1, y:1}, Point{x:1, y:2}, Point{x:1, y:3}], &i_p), true);
        let p3 = Point{x:9, y:7}; 
        let p4 = Point{x:7, y:7};
        let i_p = generate_intermediate_hv_points(p3, p4);
        // 9,7 -> 7,7
        assert_eq!(vec_compare(&vec![Point{x:9, y:7}, Point{x:8, y:7}, Point{x:7, y:7}], &i_p), true);
    }

    #[test]
    fn test_generate_intermediate_diagonal_board(){
        let p1 = Point{x:1, y:1}; //1,1 -> 1,3
        let p2 = Point{x:3, y:3}; //1,1 -> 1,3
        let i_p = generate_intermediate_points(p1, p2);
        println!("{:?}", i_p);
        assert_eq!(vec_compare(&vec![Point{x:1, y:1}, Point{x:2, y:2}, Point{x:3, y:3}], &i_p), true);
        let p3 = Point{x:9, y:7}; 
        let p4 = Point{x:7, y:9};
        let i_p = generate_intermediate_points(p3, p4);
        println!("{:?}", i_p);
        // 9,7 -> 7,7
        assert_eq!(vec_compare(&vec![Point{x:9, y:7}, Point{x:8, y:8}, Point{x:7, y:9}], &i_p), true);
    }
}
