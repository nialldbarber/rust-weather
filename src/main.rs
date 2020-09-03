use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
  city: String,
  country_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
  coord: Coord,
  weather: Weather,
  base: String,
  main: Temps,
  visibility: i32,
  wind: Wind,
  clouds: Clouds,
  dt: i32,
  sys: Sys,
  timezone: i32,
  id: i32,
  name: String,
  cod: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Coord {
  lon: f64,
  lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
  details: Details,
}

#[derive(Serialize, Deserialize, Debug)]
struct Details {
  id: i32,
  main: String,
  description: String,
  icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Temps {
  temp: f64,
  feels_like: f64,
  temp_min: f64,
  temp_max: f64,
  pressure: f64,
  humidity: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
  speed: f64,
  deg: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
  all: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys {
  r#type: f64,
  id: i32,
  country: String,
  sunrise: i32,
  sunset: i32,
}

impl Forecast {
  async fn get(city: &String, country_code: &String) -> Result<Self, ExitFailure> {
    let url: String = format!(
      "http://api.openweathermap.org/data/2.5/weather?q={},{}&appid=16b138abc604456541878135b02a4e57",
      city, country_code
    );
    let url = Url::parse(&*url)?;
    let resp = reqwest::get(url).await?.json::<Forecast>().await?;
    Ok(resp)
  }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
  let args = Cli::from_args();
  let resp = Forecast::get(&args.city, &args.country_code).await?;

  println!(
    "our city: {}\nour country code: {}\nHumidity: {}%",
    args.city, args.country_code, resp.main.humidity
  );
  Ok(())
}
