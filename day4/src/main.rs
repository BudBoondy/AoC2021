use std::fs::read_to_string;

#[derive(Debug)]
struct Value{
    val: u8,
    marked: bool
}

struct Field{
   board: Vec<Value> 
}

impl Field {
    fn check_row(&self) -> bool{
        let mut tmp: bool;
        for x in 0..5{
            tmp = true;
            for y in 0..5{
                if !self.board[y * 5 + x].marked{
                    tmp = false;
                }
            }
            if tmp{
                return true
            }
        }
        false
    }

    fn check_column(&self) -> bool{
        let mut tmp: bool;
        for y in 0..5{
            tmp = true;
            for x in 0..5{
                if !self.board[y * 5 + x].marked{
                    tmp = false;
                }
            }
            if tmp{
                return true
            }
        }
        false
    }

    fn has_won(&self) -> bool{
        self.check_row() || self.check_column()    
    }

    fn sum_non_marked(&self) -> u64{
        self.board.iter().filter(|x| !x.marked).fold(0, |acc, x| acc + x.val as u64)
    }

    fn call_number(&mut self, num: u8){
        for x in self.board.iter_mut(){
            if x.val == num{
                x.marked = true;
            }
        }
    }
}

fn read_input(input: String) -> (Vec<u8>, Vec<Field>){
   let mut lines: Vec<&str> = input.split("\n").collect();
   let draws: Vec<u8> = lines.remove(0).split(",").map(|x| x.parse::<u8>().unwrap()).collect();

   lines = lines.into_iter().filter(|x| x != &"").collect();

   let mut f: Vec<Field> = Vec::new();

   while lines.len() != 0{
        let next_five: Vec<&str> = lines.drain(0..5).collect();
        let five_int: Vec<Vec<u8>> = next_five.iter().map(|x| {
            let sp = x.split_whitespace();
            sp.map(|x| x.parse::<u8>().unwrap()).collect()
        }).collect();
        let five_int = five_int.concat();
        let val_vec = five_int.into_iter().map(|i| Value{val: i, marked: false}).collect();
        f.push(Field{board: val_vec});
   }
   (draws, f)
}

fn main() {
    // read text input
    let input_string = read_to_string("input.txt").expect("cannot read file");
    let (draws, mut fields) = read_input(input_string);
    // part 1
    for i in &draws{
        for f in fields.iter_mut(){
            f.call_number(*i);
            if f.has_won(){
                println!("Field won: {} * {} = {}", i, f.sum_non_marked(), *i as u64 * f.sum_non_marked());
                break 
            }
        }
    }
    for i in draws{
        let field_len = fields.len();
        for f in fields.iter_mut(){
            f.call_number(i);
            if field_len == 1 && f.has_won(){
                println!("Last field won: {} * {} = {}", i, f.sum_non_marked(), i as u64 * f.sum_non_marked());
                return 
            }
        }
        // iter over fields and remove field if that field has won.
        fields = fields.into_iter().filter(|x| !x.has_won()).collect::<Vec<Field>>();
    }
}


#[cfg(test)]
mod tests{
    use super::*; 

    static TEST_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
    8  2 23  4 24
    21  9 14 16  7
    6 10  3 18  5
    1 12 20 15 19

    3 15  0  2 22
    9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
    2  0 12  3  7";

    #[test]
    fn test_read_bingo(){
        let (draws, fields) = read_input(TEST_INPUT.to_string());
        assert_eq!(27, draws.len());
        assert_eq!(3, fields.len());
        assert_eq!(19, fields[0].board[24].val);
        assert_eq!(6, fields[1].board[24].val);
        assert_eq!(7, fields[2].board[24].val);
    }

    #[test]
    fn test_sum_marked(){
        let (_draws, mut fields) = read_input(TEST_INPUT.to_string());
        fields[2].call_number(7);
        fields[2].call_number(4);
        fields[2].call_number(9);
        fields[2].call_number(5);
        fields[2].call_number(11);
        fields[2].call_number(17);
        fields[2].call_number(23);
        fields[2].call_number(2);
        fields[2].call_number(0);
        fields[2].call_number(14);
        fields[2].call_number(21);
        fields[2].call_number(24);
        assert_eq!(188, fields[2].sum_non_marked());
    }

    #[test]
    fn test_check_column(){
        let (_, mut fields) = read_input(TEST_INPUT.to_string());
        fields[2].call_number(10);
        fields[2].call_number(16);
        fields[2].call_number(15);
        fields[2].call_number(9);
        assert_eq!(false, fields[2].check_column());
        fields[2].call_number(19);
        assert_eq!(true, fields[2].check_column());
   }

    #[test]
    fn test_check_row(){
        let (_, mut fields) = read_input(TEST_INPUT.to_string());
        fields[2].call_number(14);
        fields[2].call_number(10);
        fields[2].call_number(18);
        fields[2].call_number(22);
        assert_eq!(false, fields[2].check_row());
        fields[2].call_number(2);
        assert_eq!(true, fields[2].check_row()); 
    }
}
