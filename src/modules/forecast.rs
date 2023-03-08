use chrono::{Datelike, Local, Weekday};
use std::collections::HashSet;

use super::args::Forecast;

pub fn get_indices(forecast: &HashSet<Forecast>) -> [bool; 9] {
	// Create a map of indices for forecasts that should be rendered.
	// It mainly serves as navigator in the arrays of the api response.
	// [0] = current day; [1..7] = week days; [7] = week overview ; [8] = disable
	// Until there is a more concise solution this is a working and fairly slim approach.
	let mut forecast_indices = [false; 9];
	if !forecast.is_empty() {
		let todays_index = Local::now().weekday().number_from_monday();


		for val in forecast {
			match val {
				Forecast::disable => forecast_indices[8] = true,
				Forecast::day => forecast_indices[0] = true,
				Forecast::week => forecast_indices[7] = true,
				// Forecast weekdays
				Forecast::mo => {
					forecast_indices[get_day_index(todays_index, Weekday::Mon.number_from_monday())] = true;
				}
				Forecast::tu => {
					forecast_indices[get_day_index(todays_index, Weekday::Tue.number_from_monday())] = true;
				}
				Forecast::we => {
					forecast_indices[get_day_index(todays_index, Weekday::Wed.number_from_monday())] = true;
				}
				Forecast::th => {
					forecast_indices[get_day_index(todays_index, Weekday::Thu.number_from_monday())] = true;
				}
				Forecast::fr => {
					forecast_indices[get_day_index(todays_index, Weekday::Fri.number_from_monday())] = true;
				}
				Forecast::sa => {
					forecast_indices[get_day_index(todays_index, Weekday::Sat.number_from_monday())] = true;
				}
				Forecast::su => forecast_indices[get_day_index(todays_index, Weekday::Sun.number_from_monday())] = true,
			}
		}
	};

	forecast_indices
}

fn get_day_index(todays_index: u32, weekday_index: u32) -> usize {
	(((weekday_index as i8 - todays_index as i8) % 7 + 7) % 7).try_into().unwrap()
}

