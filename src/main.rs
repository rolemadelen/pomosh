use chrono::{Local, TimeDelta};
use colored::Colorize;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn read_string() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("[read_int()] failed to read an input");
    input
}

fn setup(focus_session: &mut i64, break_session: &mut i64) {
    let focus_bound = [5, 90];
    let break_bound = [2, 90];
    loop {
        print!(
            "How long is the focus session? ({lower}-{upper} minutes): ",
            lower = focus_bound[0],
            upper = focus_bound[1]
        );
        io::stdout().flush().unwrap();
        *focus_session = read_string()
            .trim()
            .parse()
            .expect("failed to parse a focus_session");

        if *focus_session >= 5 && *focus_session <= 90 {
            break;
        }
    }

    loop {
        print!(
            "How long is the short break? ({lower}-{upper} minutes): ",
            lower = break_bound[0],
            upper = break_bound[1]
        );
        io::stdout().flush().unwrap();
        *break_session = read_string()
            .trim()
            .parse()
            .expect("failed to parse a break_session");

        if *break_session >= 2 && *break_session <= 90 {
            break;
        }
    }
}

fn merge_and_print(a: &str, b: &str) {
    let a = a.to_string();
    let b = b.to_string();

    let a: Vec<&str> = a.split('\n').collect();
    let b: Vec<&str> = b.split('\n').collect();

    println!();
    for i in 0..5 {
        print!("{}  {}", a[i].blue(), b[i].blue());
        println!();
    }
    println!();
}

fn get_session_duration(duration: i64) -> (String, String) {
    let session_start = Local::now();
    let session_end = session_start + TimeDelta::try_minutes(duration).unwrap();
    let start_time_str = session_start.format("%H:%M").to_string();
    let end_time_str = session_end.format("%H:%M").to_string();
    (start_time_str, end_time_str)
}

fn main() {
    let ascii_art: [&str; 10] = [
        "000000\n00  00\n00  00\n00  00\n000000",
        "1111  \n  11  \n  11  \n  11  \n111111",
        "222222\n     2\n222222\n2     \n222222",
        "333333\n    33\n333333\n    33\n333333",
        "44  44\n44  44\n444444\n    44\n    44",
        "555555\n55    \n555555\n    55\n555555",
        "666666\n66    \n666666\n66  66\n666666",
        "777777\n    77\n    77\n    77\n    77",
        "888888\n88  88\n888888\n88  88\n888888",
        "999999\n99  99\n999999\n    99\n999999",
    ];

    let mut focus_session: i64 = 0;
    let mut break_session: i64 = 0;

    setup(&mut focus_session, &mut break_session);

    loop {
        print!("Is this correct? '{focus_session}/{break_session}' (y/N): ");
        io::stdout().flush().unwrap();
        match read_string().trim() {
            "y" | "Y" => break,
            "n" | "N" => setup(&mut focus_session, &mut break_session),
            _ => continue,
        }
    }

    let mut session_cnt = 0;

    loop {
        let mut focus_min = focus_session - 1;
        let mut break_min = break_session - 1;
        let mut long_break = false;

        let (start, end) = get_session_duration(focus_session);
        let books_emoji = [
            char::from_u32(0x1f4d5).unwrap(),
            char::from_u32(0x1f4d7).unwrap(),
            char::from_u32(0x1f4d8).unwrap(),
            char::from_u32(0x1f4d9).unwrap(),
        ];

        let square_emoji = char::from_u32(0x25fb).unwrap();
        while focus_min >= 0 {
            print!("\x1B[2J");
            for i in 0..=session_cnt {
                print!("{} ", books_emoji[i % 4]);
                if (i + 1) % 4 == 0 {
                    println!();
                }
            }
            for i in 0..4 - (session_cnt % 4) - 1 {
                print!("{} ", square_emoji);
                if (i + 1) % 4 == 0 {
                    println!();
                }
            }
            println!();
            println!();
            println!("focus: {focus_session} minutes / ({start} - {end})");

            let tens = (focus_min / 10) as usize;
            let ones = (focus_min % 10) as usize;
            merge_and_print(ascii_art[tens], ascii_art[ones]);

            sleep(Duration::new(1, 0));
            focus_min -= 1;
        }

        let (start, end) = get_session_duration(break_session);

        if (session_cnt + 1) % 4 == 0 {
            long_break = true;
            break_min <<= 1;
        }

        while break_min >= 0 {
            print!("\x1B[2J");
            if long_break {
                println!(
                    "long break: {} minutes / ({start} - {end})",
                    break_session << 1
                );
            } else {
                println!("short break: {break_session} minutes / ({start} - {end})");
            }

            let tens = (break_min / 10) as usize;
            let ones = (break_min % 10) as usize;
            merge_and_print(ascii_art[tens], ascii_art[ones]);

            sleep(Duration::new(1, 0));
            break_min -= 1;
        }

        session_cnt += 1;
    }
}
