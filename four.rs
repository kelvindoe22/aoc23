use std::fs;

struct RGB{
    r: usize,
    g: usize,
    b: usize,
}

impl RGB {
    fn find_max(&mut self, other: &Self ) {
        if other.r > self.r {
            self.r = other.r
        }
        if other.g > self.g {
            self.g = other.g
        }
        if other.b > self.b {
            self.b = other.b
        }
    }
    fn power(&self) -> usize {
        self.r * self.g * self.b
    }
}



fn main() {
    let foo = fs::read_to_string("sample.txt").unwrap();
    let lines: Vec<&str> = foo.split('\n').collect();
    let mut total_power = 0;
    for l in lines {
        let game: Vec<&str> = l.split(':').collect();
        let rgb = make_rgb(game[1]);
        total_power += rgb.power();
    }
    println!("{total_power}")
}

fn make_rgb(line: &str) -> RGB {

    let trimmed = line.trim();
    let lines: Vec<&str> = trimmed.split(';').collect();
    let mut control_rgb = RGB {
        r:0, g:0, b:0
    };
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
            control_rgb.find_max(&rgb); 
        }
    }
    control_rgb
}