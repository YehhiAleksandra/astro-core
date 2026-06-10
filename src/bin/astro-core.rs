use std::io::{self, Write};

use astro_core::{parse_date, profile, sun_sign, NumerologyProfile, SunSign};
use clap::{Parser, Subcommand};
use serde::Serialize;

#[derive(Parser)]
#[command(name = "astro-core", about = "Numerology and zodiac CLI for Digital Oracle")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Life path and personal year
    Numerology {
        date: String,
        #[arg(long, default_value_t = 2026)]
        year: u16,
        #[arg(long)]
        json: bool,
    },
    /// Western sun sign
    Zodiac {
        date: String,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Serialize)]
struct NumerologyOut {
    date: String,
    year: u16,
    life_path: u8,
    personal_year: u8,
    is_master: bool,
}

#[derive(Serialize)]
struct ZodiacOut {
    date: String,
    sign_index: usize,
    sign_ru: &'static str,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Numerology { date, year, json } => {
            let birth = parse_date(&date)?;
            let p = profile(birth, year);
            if json {
                print_json(&numerology_json(birth.to_string(), year, p))?;
            } else {
                print_numerology(birth.to_string(), year, p);
            }
        }
        Commands::Zodiac { date, json } => {
            let birth = parse_date(&date)?;
            let sign = sun_sign(birth.day, birth.month);
            if json {
                print_json(&zodiac_json(birth.to_string(), sign))?;
            } else {
                print_zodiac(birth.to_string(), sign);
            }
        }
    }
    Ok(())
}

fn numerology_json(date: String, year: u16, p: NumerologyProfile) -> NumerologyOut {
    NumerologyOut {
        date,
        year,
        life_path: p.life_path,
        personal_year: p.personal_year,
        is_master: p.is_master_life_path,
    }
}

fn zodiac_json(date: String, sign: SunSign) -> ZodiacOut {
    ZodiacOut {
        date,
        sign_index: sign.index,
        sign_ru: sign.name_ru,
    }
}

fn print_numerology(date: String, year: u16, p: NumerologyProfile) {
    println!("Date: {date}");
    println!("Life path: {}", p.life_path);
    println!("Personal year ({year}): {}", p.personal_year);
    if p.is_master_life_path {
        println!("Master number");
    }
}

fn print_zodiac(date: String, sign: SunSign) {
    println!("Date: {date}");
    println!("Sun sign: {}", sign.name_ru);
}

fn print_json<T: Serialize>(value: &T) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    serde_json::to_writer_pretty(&mut stdout, value)?;
    stdout.write_all(b"\n")?;
    Ok(())
}
