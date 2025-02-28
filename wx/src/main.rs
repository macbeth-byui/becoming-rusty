use chrono::{DateTime, Local};
use textplots::{Chart, ColorPlot, Plot, Shape};
use serde::{Deserialize};

const API_KEY : &str = "84a62aebb145110885ccb012335cfb4a";

const GREEN: rgb::RGB8 = rgb::RGB8::new(0x00, 0xFF, 0x00);

#[derive(Deserialize, Debug)]
struct ForecastWx {
    pub list : Vec<CurrentWx>,
}

#[derive(Deserialize, Debug)]
struct CurrentWx {
    pub weather : Vec<CurrentWxWeather>,
    pub main : CurrentWxMain,
    pub wind : CurrentWxWind,
    pub dt : i64,
}

#[derive(Deserialize, Debug)]
struct CurrentWxWeather {
    pub description : String,
}

#[derive(Deserialize, Debug)]
struct CurrentWxMain {
    pub temp : f64,
    pub feels_like : f64,
    pub pressure : f64,
    pub humidity : f64,
}

#[derive(Deserialize, Debug)]
struct CurrentWxWind {
    pub speed : f64,
    pub deg : f64,
    pub gust : Option<f64>,
}

async fn get_current_wx(latitude : f64, longitude : f64) -> Result<CurrentWx, Box<dyn std::error::Error>> {
    let url = format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units=imperial", latitude, longitude, API_KEY);
    Ok(reqwest::get(url)
        .await?
        .json::<CurrentWx>()
        .await?)
}

async fn get_forecast_wx(latitude : f64, longitude : f64) -> Result<ForecastWx, Box<dyn std::error::Error>> {
    let url = format!("https://api.openweathermap.org/data/2.5/forecast?lat={}&lon={}&appid={}&units=imperial", latitude, longitude, API_KEY);
    Ok(reqwest::get(url)
        .await?
        .json::<ForecastWx>()
        .await?)
}

fn round_f64(temp : f64) -> i32 {
    let temp_round = temp.round() as i32;
    if temp_round == 0 { 0 } else { temp_round }
}

fn print_current_wx(wx : &CurrentWx) {
    let press_inhg = wx.main.pressure / 33.8639;

    let wind_dir = match wx.wind.deg {
        n if n < 22.5 => "N ",
        n if n < 67.5 => "NE",
        n if n < 112.5 => "E ",
        n if n < 157.5 => "SE",
        n if n < 202.5 => "S ",
        n if n < 247.5 => "SW",
        n if n < 292.5 => "W ",
        n if n < 337.5 => "NW",
        _ => "N "
    };

    let gust = match wx.wind.gust {
        Some(gust) => format!("{}",round_f64(gust)),
        None => "-".to_string()
    };

    println!("Current Conditions");
    println!("===========================================");
    for weather in wx.weather.iter() {
        println!("{}", weather.description);
    }
    println!("Temp    : {} F        Feels Like: {} F", round_f64(wx.main.temp), round_f64(wx.main.feels_like));
    println!("Pressure: {:.2} inHg  Humidity  : {}%", press_inhg, wx.main.humidity);
    println!("Wind    : {} mph {}    Gust      : {} mph", round_f64(wx.wind.speed), wind_dir, gust);
    println!("===========================================");
    println!();
}

fn print_forecast_wx(wx_all : &ForecastWx) {
    println!("Forecast\tTemp\tWeather");
    println!("=========\t=====\t==============");
    for wx in wx_all.list.iter() {
        let dt = DateTime::from_timestamp(wx.dt, 0)
        .expect("Invalid timestamp");

        let dt_local = dt.with_timezone(&Local);
        let dt_local_str = dt_local.format("%a %l %P").to_string();

        print!("{}\t{}\t", dt_local_str, round_f64(wx.main.temp));
        for weather in wx.weather.iter() {
            print!("{} ", weather.description);
        } 
        println!();
    }
    let temps : Vec<(f32, f32)> = wx_all.list.iter().enumerate().map(|a| (a.0 as f32, a.1.main.temp as f32)).collect();

    let mut chart = Chart::new(200, 50, 0.0, wx_all.list.len() as f32);
    let binding = Shape::Lines(&temps);
    let plot = chart.linecolorplot(&binding, GREEN);
    plot.axis();
    plot.nice();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current = get_current_wx(43.825386, -111.792824).await?;
    print_current_wx(&current);
    let forecast = get_forecast_wx(43.825386, -111.792824).await?;
    print_forecast_wx(&forecast);
    Ok(())
}