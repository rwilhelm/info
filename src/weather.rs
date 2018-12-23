extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::io::Read;
use serde_json::Error;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(rename_all = "snake_case")]
struct Currently {
    //time: i32,
    summary: String,
    icon: String,
    //precipIntensity: f64,
    //precipProbability: f64,
    temperature: f64,
    //apparentTemperature: f64,
    //dewPoint: f64,
    //humidity: f64,
    //pressure: f64,
    //windSpeed: f64,
    //windGust: f64,
    //windBearing: f64,
    //cloudCover: f64,
    //uvIndex: f64,
    //visibility: f64,
    //ozone: f64,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Weather {
    currently: Currently
}

fn get_weather() -> Result<Weather, Error> {
    let mut resp = reqwest::get("https://api.darksky.net/forecast/93070100ecdc26faea48be30aa155d75/50.778494,7.9506802?units=si").unwrap();
    assert!(resp.status().is_success());
    let mut content = String::new();
    let _res = resp.read_to_string(&mut content);
    //println!("{:?}", r);

    let w: Weather = serde_json::from_str(content.as_str())?;
    Ok(w)
}

fn main() {

    // A machine-readable text summary of this data point, suitable for selecting an icon for
    // display. If defined, this property will have one of the following values: clear-day,
    // clear-night, rain, snow, sleet, wind, fog, cloudy, partly-cloudy-day, or
    // partly-cloudy-night.  (Developers should ensure that a sensible default is defined, as
    // additional values, such as hail, thunderstorm, or tornado, may be defined in the future.)

    let w: Weather = get_weather().unwrap();
    let mut color: String = format!("%{{F#BEB6AE}}");
    if w.currently.icon == "rain" {
        color = format!("%{{F#5975ff}}")
    }
    if w.currently.icon == "snow" {
        color = format!("%{{F#F1F0EE}}")
    }
    if w.currently.icon == "clear-day" {
        color = format!("%{{F#CCB647}}")
    }
    println!(" {}{:?} {}%{{F-}} ", color, w.currently.temperature, w.currently.summary);
}
