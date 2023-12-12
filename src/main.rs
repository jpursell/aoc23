mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Usage: {} NUM", args[0]);
        return;
    }
    let num = args[1].parse::<u8>().unwrap();
    match num {
        1 => {
            day_1::day_1();
        }
        2 => {
            day_2::day_2();
        }
        3 => {
            day_3::day_3();
        }
        4 => {
            day_4::day_4();
        }
        5 => {
            day_5::day_5();
        }
        6 => {
            day_6::day_6();
        }
        7 => {
            day_7::day_7();
        }
        8 => {
            day_8::run();
        }
        _ => {
            panic!();
        }
    }
}
