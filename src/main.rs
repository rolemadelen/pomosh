use chrono::{Local, TimeDelta};
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
    print!("How long is the focus session? (minutes): ");
    io::stdout().flush().unwrap();
    *focus_session = read_string()
        .trim()
        .parse()
        .expect("failed to parse a focus_session");

    print!("How long is the break? (minutes): ");
    io::stdout().flush().unwrap();
    *break_session = read_string()
        .trim()
        .parse()
        .expect("failed to parse a break_session");
}

fn merge_and_print(a: &str, b: &str) {
    let a = a.to_string();
    let b = b.to_string();

    let a: Vec<&str> = a.split('\n').collect();
    let b: Vec<&str> = b.split('\n').collect();

    println!();
    for i in 0..5 {
        print!("{}   {}", a[i], b[i]);
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

    loop {
        let mut focus_min = focus_session - 1;
        let mut break_min = break_session - 1;

        let (start, end) = get_session_duration(focus_session);

        while focus_min >= 0 {
            print!("\x1B[2J");
            println!("focus for {focus_session} minute(s) ー ({start} - {end})");

            let tens = (focus_min / 10) as usize;
            let ones = (focus_min % 10) as usize;
            merge_and_print(ascii_art[tens], ascii_art[ones]);

            sleep(Duration::new(60, 0));
            focus_min -= 1;
        }

        let (start, end) = get_session_duration(break_session);

        while break_min >= 0 {
            print!("\x1B[2J");
            println!("take a break for {break_session} minute(s) ー ({start} - {end})");

            let tens = (break_min / 10) as usize;
            let ones = (break_min % 10) as usize;
            merge_and_print(ascii_art[tens], ascii_art[ones]);

            sleep(Duration::new(60, 0));
            break_min -= 1;
        }
    }
}
