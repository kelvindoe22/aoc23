use std::fs;

struct RGB{
    r: usize,
    g: usize,
    b: usize,
}

impl RGB {
    fn is_possible(&self) -> bool {
        self.r <= 12 && self.g <= 13 && self.b <= 14
    }
}



fn main() {
    let foo = fs::read_to_string("sample.txt").unwrap();
    let lines: Vec<&str> = foo.split('\n').collect();
    let mut total_game = 0;
    for l in lines {
        let game: Vec<&str> = l.split(':').collect();
        let game_number: Vec<&str> = game[0].split(' ').collect();
        let num = game_number[1].parse::<usize>().unwrap();
        let rgb = make_rgb(game[1]);
        if rgb {
            total_game += num
        }
    }
    println!("{total_game}")
}

fn make_rgb(line: &str) -> bool {

    let trimmed = line.trim();
    let lines: Vec<&str> = trimmed.split(';').collect();
    for l in lines {
        let ml: Vec<&str> = l.split(',').collect();
        for balls in ml {
            let b = balls.trim();
            let num_color :Vec<&str> = b.split(' ').collect();
            let num: usize = num_color[0].parse().expect("Parse Error");
            let mut rgb = RGB {
                r:0, g:0, b:0
            };
            match num_color[1] {
                "red" => {
                    rgb.r = num
                }
                "green" => {
                    rgb.g = num
                }
                "blue" => {
                    rgb.b = num
                }
                def => {
                    println!("{}", def)
                }
            }
            if !rgb.is_possible() {
                return false;
            }
            
        }
    }
    true
}