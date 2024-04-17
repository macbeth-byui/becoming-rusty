use clap::{Parser, ValueEnum};

// Command Line Setup

#[derive(Parser, Debug)]
#[command(version, about = "Temp Converter")]
struct Args {
    #[clap(value_enum, help = "Convert From")]
    from: TempUnit,

    #[clap(value_enum, help = "Convert To")]
    to: TempUnit,

    #[clap(help = "Temperature to Convert")]
    temp: f64,
}

#[derive(Clone, Debug, ValueEnum)]
enum TempUnit {
    C,
    F,
    K,
}

fn convert_c_to_f(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn convert_f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn convert_c_to_k(c: f64) -> f64 {
    c + 273.15
}

fn convert_k_to_c(k: f64) -> f64 {
    k - 273.15
}

fn convert_f_to_k(f: f64) -> f64 {
    convert_c_to_k(convert_f_to_c(f))
}

fn convert_k_to_f(k: f64) -> f64 {
    convert_c_to_f(convert_k_to_c(k))
}

fn main() {
    let args = Args::parse();
    let result = match (&args.from, &args.to) {
        (TempUnit::C, TempUnit::F) => convert_c_to_f(args.temp),
        (TempUnit::F, TempUnit::C) => convert_f_to_c(args.temp),
        (TempUnit::C, TempUnit::K) => convert_c_to_k(args.temp),
        (TempUnit::K, TempUnit::C) => convert_k_to_c(args.temp),
        (TempUnit::F, TempUnit::K) => convert_f_to_k(args.temp),
        (TempUnit::K, TempUnit::F) => convert_k_to_f(args.temp),
        _ => args.temp,
    };
    println!("{} {:?}", result, args.to);
}
