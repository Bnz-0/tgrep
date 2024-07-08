use clap::Parser;
use std::io;
use std::io::BufRead;
use std::str::FromStr;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration, Instant};
use colored::Colorize;
use colored::ColoredString;

#[derive(Clone, Copy)]
enum TimeUnit {
    Nano,
    Micro,
    Milli,
    Seconds,
}
impl TimeUnit {
    fn parse(option: &str) -> Result<TimeUnit, ()> {
        match option.to_lowercase().as_str() {
            "nano" | "ns" => Ok(TimeUnit::Nano),
            "micro" | "us" => Ok(TimeUnit::Micro),
            "milli" | "ms" => Ok(TimeUnit::Milli),
            "seconds" | "sec" | "s" => Ok(TimeUnit::Seconds),
            _ => Err(()),
        }
    }
}

impl FromStr for TimeUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s).map_err(|_| format!("invalid time unit variant: {s}"))
    }
}

impl clap::ValueEnum for TimeUnit {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Nano, Self::Micro, Self::Milli, Self::Seconds]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        use clap::builder::PossibleValue;
        Some(match self {
            Self::Nano => PossibleValue::new("ns"),
            Self::Micro => PossibleValue::new("us"),
            Self::Milli => PossibleValue::new("ms"),
            Self::Seconds => PossibleValue::new("s"),
        })
    }
}

fn format_duration(d: Duration, unit: &Option<TimeUnit>) -> ColoredString {
    match unit {
        None => match d.as_nanos() {
            0..=1099 => format_duration(d, &Some(TimeUnit::Nano)),
            1100..=1099999 => format_duration(d, &Some(TimeUnit::Micro)),
            1100000..=1099999999 => format_duration(d, &Some(TimeUnit::Milli)),
            _ => format_duration(d, &Some(TimeUnit::Seconds)),
        },
        Some(TimeUnit::Nano) => format!("{}ns", d.as_nanos()).cyan(),
        Some(TimeUnit::Micro) => format!("{:.3}us", d.as_nanos() as f64 / 1000.0).green(),
        Some(TimeUnit::Milli) => format!("{:.3}ms", d.as_micros() as f64 / 1000.0).yellow(),
        Some(TimeUnit::Seconds) => {
        	let s = format!("{:.3}s", d.as_millis() as f64 / 1000.0);
        	if d.as_secs() >= 10 {
        		s.red().bold()
        	} else {
        		s.red()
        	}
        }
    }
}

struct TimedLine {
    content: String,
    arrival_time: Instant,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from Cargo.toml
struct Cli {
    /// Ignore case distinctions in patterns and data
    #[arg(short = 'i', long = "ignore-case")]
    ignore_case: bool,

    /// Fix the unit time used while printing
    #[arg(short = 'u', long = "fix-unit")]
    time_unit: Option<TimeUnit>,

    pattern: Option<String>,
}

fn main() {
    let args = Cli::parse();

    let mut pattern = args.pattern.unwrap_or(String::from(""));
    if args.ignore_case {
        pattern = pattern.to_lowercase();
    }

    let (tx, rx): (Sender<TimedLine>, Receiver<TimedLine>) = mpsc::channel();
    // working thread
    let handle = std::thread::spawn(move || {
        let mut last_matched_time = Instant::now();
        for line in rx {
            if args.ignore_case && line.content.to_lowercase().contains(&pattern)
                || line.content.contains(&pattern)
            {
                println!(
                    "{duration:<0$}| {content}",
                    10,
                    duration=format_duration(line.arrival_time - last_matched_time, &args.time_unit),
                    content=line.content,
                );
                last_matched_time = line.arrival_time;
            }
        }
    });

    for line in io::stdin().lock().lines() {
        tx.send(TimedLine {
            content: line.unwrap(),
            arrival_time: Instant::now(),
        })
        .unwrap();
    }
    drop(tx);

    handle.join().unwrap();
}
