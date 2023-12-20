mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

fn parse_error_message(args: &Vec<String>) {
    println!("Usage: {} NUM [-d/--debug]", args[0]);
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 && args.len() != 3 {
        parse_error_message(&args);
        return;
    }
    let num = args[1].parse::<u8>().unwrap();
    let debug = match args.len() {
        2 => false,
        3 => {
            if args[2] == "-d" || args[2] == "--debug" {
                true
            } else {
                parse_error_message(&args);
                return;
            }
        }
        _ => {
            parse_error_message(&args);
            return;
        }
    };
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
        9 => {
            day_9::run();
        }
        10 => {
            day_10::run();
        }
        11 => {
            day_11::run();
        }
        12 => {
            if debug {
                day_12::run(day_12::b::RunMode::Time);
            } else {
                day_12::run(day_12::b::RunMode::Fast);
            }
        }
        13 => {
            day_13::run();
        }
        14 => {
            day_14::run();
        }
        15 => {
            day_15::run();
        }
        16 => {
            day_16::run();
        }
        17 => {
            day_17::run();
        }
        _ => {
            panic!();
        }
    }
}
