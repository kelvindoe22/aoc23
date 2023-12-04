struct FirstLast {
    f: (usize, usize),
    l: (usize, usize)
}

impl FirstLast{
    fn double_digit(&self) -> usize {
        (format!("{}{}", self.f.0, self.l.0)).parse::<usize>().expect("Parse Error.")
    }
}

fn main() {
    let whole_t = std::env::args().nth(1).expect("no arguments received");
    let  lines:Vec<&str> = whole_t.split('\n').collect();
    let mut total = 0;
    for l in lines {
        let fl = first_and_last(l).double_digit();
        total += fl;
        println!("{l} {fl}")
    }
    println!("{}", total);
}

fn get_num(word: &str) -> Option<usize> {
    match word {
        "one" |"1" => return Some(1),
        "two" | "2" => return Some(2),
        "three" | "3" => return Some(3),
        "four" | "4"=> return Some(4),
        "five" | "5" => return Some(5),
        "six" | "6"=>  return Some(6),
        "seven" | "7"=> return Some(7),
        "eight" | "8" => return Some(8),
        "nine" | "9"=> return Some(9),
        _ => None,
    }
}

fn first_and_last(word: &str) -> FirstLast {
    let looking_for = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
    ];
    let mut first = (0, usize::MAX);
    let mut last = (0, usize::MIN);
    for l in looking_for {
        let pos = word.find(l);
        if pos.is_some() {
            let position = pos.unwrap();
            if position <= first.1 {
                first = (get_num(l).unwrap(), position)
            }
            if position >= last.1 {
                last = (get_num(l).unwrap(), position)
            }
        }
        let pos = word.rfind(l);
        if pos.is_some() {
            let position = pos.unwrap();
            if position <= first.1 {
                first = (get_num(l).unwrap(), position)
            }
            if position >= last.1 {
                last = (get_num(l).unwrap(), position)
            }
        }
    }

    FirstLast{
        f : first,
        l : last
    }
}