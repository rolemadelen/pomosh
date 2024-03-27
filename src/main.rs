use chrono::{Local, TimeDelta};
use colored::Colorize;
use rodio::{Decoder, OutputStream, Sink};
use std::{
    io::{self, BufReader, Write, Cursor},
    thread::{self, sleep},
    time::Duration,
};
use clap::Parser;


const CHIME_SOUND: &[u8] = include_bytes!("../assets/chime.mp3");

enum SquareEmoji {
    Open = 0x25fb,
    Closed = 0x25fc,
}
enum BookEmoji {
    Red = 0x1f4d5,
    Green = 0x1f4d7,
    Blue = 0x1f4d8,
    Orange = 0x1f4d9,
}

enum DurationType {
    Focus,
    Break,
    LongBreak,
}

struct SessionConfig {
    focus_duration: i64,
    break_duration: i64,
    long_break_duration: i64,
    mute: bool,
}

impl SessionConfig {
    fn new() -> Self {
        Self {
            focus_duration: 25,
            break_duration: 5,
            long_break_duration: 10,
            mute: false,
        }
    }

    fn prompt_for_settings(&mut self) {
        print!("\x1B[2J\x1b[1;1H");
        println!();

        self.focus_duration = self.prompt_for_duration("Focus duration");
        self.break_duration = self.prompt_for_duration("Short break duration");
        self.long_break_duration = self.prompt_for_duration("Long break duration");
        self.validate_config();
    }

    fn validate_config(&mut self) {
        print!("\x1B[2J\x1b[1;1H");
        println!("Focus duration: {} mins", self.focus_duration);
        println!("Break duration: {} mins", self.break_duration);
        println!("Long Break duration: {} mins", self.long_break_duration);
        if self.mute {
            println!("Chime: disabled");
        } else {
            println!("Chime: enabled");
        }
        print!("Start the session? (y/N): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input - validate_config");

        match input.trim() {
            "y" | "Y" => return,
            _ => self.prompt_for_settings(),
        }
    }

    fn prompt_for_duration(&self, prompt: &str) -> i64 {
        let duration_bound = [5, 90];

        loop {
            print!(
                "{} ({lower}-{upper} minutes): ",
                prompt,
                lower = duration_bound[0],
                upper = duration_bound[1]
            );
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input - prompt_for_duration");
            match input.trim().parse::<i64>() {
                Ok(t) => {
                    if t >= duration_bound[0] && t <= duration_bound[1] {
                        return t;
                    } else {
                        eprintln!(
                            "Invalid input: {}",
                            "invalid range was found".to_string().red()
                        )
                    }
                }
                Err(e) => eprintln!("Invalid input: {}", e.to_string().red()),
            }
        }
    }

    fn get_session_duration(&self, duration_type: DurationType) -> (String, String) {
        let duration: i64 = match duration_type {
            DurationType::Focus => self.focus_duration,
            DurationType::Break => self.break_duration,
            DurationType::LongBreak => self.long_break_duration,
        };

        let session_start = Local::now();
        let session_end = session_start + TimeDelta::try_minutes(duration).unwrap();
        let start_time_str = session_start.format("%H:%M").to_string();
        let end_time_str = session_end.format("%H:%M").to_string();
        (start_time_str, end_time_str)
    }
}

fn run_session(config: &SessionConfig) {
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
    let mut session_cnt = 0;
    let mut round = 1;
    let mute = config.mute;

    let square_emoji = [
        char::from_u32(SquareEmoji::Open as u32).unwrap(),
        char::from_u32(SquareEmoji::Closed as u32).unwrap(),
    ];
    let books_emoji = [
        char::from_u32(BookEmoji::Red as u32).unwrap(),
        char::from_u32(BookEmoji::Green as u32).unwrap(),
        char::from_u32(BookEmoji::Blue as u32).unwrap(),
        char::from_u32(BookEmoji::Orange as u32).unwrap(),
    ];

    loop {
        let mut focus_min = config.focus_duration - 1;
        let mut break_min = config.break_duration - 1;
        let mut long_break = false;

        let (start, end) = config.get_session_duration(DurationType::Focus);

        while focus_min >= 0 {
            print!("\x1B[2J\x1b[1;1H");
            println!();
            for i in 0..=session_cnt {
                print!("{} ", books_emoji[i % 4]);
            }
            for _ in 0..(3 - session_cnt) {
                print!("{} ", square_emoji[0]);
            }
            println!(" (r{}.{})", round, session_cnt + 1);
            println!("\nfocus: {} mins\n({start} - {end})", config.focus_duration);

            let tens = (focus_min / 10) as usize;
            let ones = (focus_min % 10) as usize;
            merge_and_print(ascii_art[tens], ascii_art[ones]);

            sleep(Duration::new(1, 0));
            focus_min -= 1;
        }
        if !mute {
            thread::spawn(move || {
                play_audio();
            });
        }

        if (session_cnt + 1) % 4 == 0 {
            long_break = true;
            break_min <<= 1;
        }

        while break_min >= 0 {
            print!("\x1B[2J\x1b[1;1H");
            println!();
            for i in 0..=session_cnt {
                print!("{} ", books_emoji[i % 4]);
            }
            for i in 0..(3 - session_cnt) {
                if i == 0 {
                    print!("{} ", square_emoji[1]);
                } else {
                    print!("{} ", square_emoji[0]);
                }
            }
            println!(" (r{}.{})", round, session_cnt + 1);
            if long_break {
                let (start, end) = config.get_session_duration(DurationType::LongBreak);
                println!(
                    "\nlong break: {} mins\n({start} - {end})",
                    config.long_break_duration
                );
            } else {
                let (start, end) = config.get_session_duration(DurationType::Break);
                println!(
                    "\nshort break: {} mins\n({start} - {end})",
                    config.break_duration
                );
            }

            let tens = (break_min / 10) as usize;
            let ones = (break_min % 10) as usize;
            merge_and_print(ascii_art[tens], ascii_art[ones]);

            sleep(Duration::new(1, 0));
            break_min -= 1;
        }
        if !mute {
            thread::spawn(move || {
                play_audio();
            });
        }

        if (session_cnt + 1) % 4 == 0 {
            round += 1;
        }

        session_cnt = (session_cnt + 1) % 4;
    }
}

fn play_audio() {
    let cursor = Cursor::new(CHIME_SOUND);
    let source = Decoder::new(BufReader::new(cursor)).unwrap();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}

fn merge_and_print(a: &str, b: &str) {
    let a = a.to_string();
    let b = b.to_string();

    let a: Vec<&str> = a.split('\n').collect();
    let b: Vec<&str> = b.split('\n').collect();

    println!();
    for i in 0..5 {
        print!(" {}  {}", a[i].bright_blue(), b[i].bright_blue());
        println!();
    }
    println!();
}


#[derive(Parser)]
#[command(version, about = "Command line Pomodoro Timer", long_about = None)]
struct Args {
    #[arg(short, long, help="Preset pomodoro (focus/break/long break): 1) 25/5/10, 2) 50/10/20", value_name = "1|2" )]
    preset: Option<u32>,
    
    #[arg(short, long, help="Disable session/break complete chime")]
    mute: bool,
    
}

fn main() {
    let cli = Args::parse();
    let mut config = SessionConfig::new();
    config.mute = cli.mute;

    if let Some(v) = cli.preset {
        match v {
            1 => {
                run_session(&config);
            },
            2 => {
                config.focus_duration = 50;
                config.break_duration = 10;
                config.long_break_duration = 20;
                run_session(&config);
            },
            _ => {
                eprintln!("{}: '{}' is not a valid preset code.\n\nFor more information, try 'pomosh --help'", "error".red(), v);
            }
        }
    } else {
        config.prompt_for_settings();
        run_session(&config);
    }
}
