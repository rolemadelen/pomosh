use std::io::{self, Write};
use std::time::{Duration, SystemTime};
use std::thread::sleep;

fn read_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("[read_int()] failed to read an input");
    input
}

fn setup(focus_session: &mut i32, break_session: &mut i32) {
    print!("How long is the focus session? (minutes): ");
    io::stdout().flush().unwrap();
    *focus_session = read_string().trim().parse().expect("faild to parse a focus_session");

    print!("How long is the break? (minutes): ");
    io::stdout().flush().unwrap();
    *break_session = read_string().trim().parse().expect("failed to parse a break_session");
}

fn main() {
    let mut focus_session: i32 = 0;
    let mut break_session: i32 = 0;

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

    let mut focus_min = focus_session;
    let mut break_min = break_session;
    
    loop {
        while focus_min > 0 {
            println!("focus: {focus_min}");
            sleep(Duration::new(60, 0));
            focus_min -= 1;
        }
        focus_min = focus_session;

        while break_min > 0 {
            println!("break: {break_min}");
            sleep(Duration::new(60, 0));
            break_min -= 1;
        }
        break_min = break_session;
    }
}