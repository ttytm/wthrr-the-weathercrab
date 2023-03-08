#[allow(unused_imports)]
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
		#[cfg(not(test))]
		let todays_index = Local::now().weekday().number_from_monday();

		#[cfg(test)]
		// We mock today to always be a Monday, as indices will be dynamically set based on the current day.
		// If I find a way to mock the system time instead of fixing a specific day, it will be integrated
		// as preferred method. Nevertheless, we will unit test `get_day_index` using other days as monday.
		let todays_index = Weekday::Mon.number_from_monday();

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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn index_form_day() {
		// Test to determine the index of the current day when the current day is Monday.
		assert!(get_day_index(Weekday::Mon.number_from_monday(), Weekday::Tue.number_from_monday()) == 1);
		assert!(get_day_index(Weekday::Mon.number_from_monday(), Weekday::Wed.number_from_monday()) == 2);
		assert!(get_day_index(Weekday::Mon.number_from_monday(), Weekday::Thu.number_from_monday()) == 3);
		assert!(get_day_index(Weekday::Mon.number_from_monday(), Weekday::Fri.number_from_monday()) == 4);
		assert!(get_day_index(Weekday::Mon.number_from_monday(), Weekday::Sat.number_from_monday()) == 5);
		assert!(get_day_index(Weekday::Mon.number_from_monday(), Weekday::Sun.number_from_monday()) == 6);
		// Test to determine the index of the current day when the current day is Saturday.
		assert!(get_day_index(Weekday::Sat.number_from_monday(), Weekday::Sun.number_from_monday()) == 1);
		assert!(get_day_index(Weekday::Sat.number_from_monday(), Weekday::Mon.number_from_monday()) == 2);
		assert!(get_day_index(Weekday::Sat.number_from_monday(), Weekday::Tue.number_from_monday()) == 3);
		assert!(get_day_index(Weekday::Sat.number_from_monday(), Weekday::Wed.number_from_monday()) == 4);
		assert!(get_day_index(Weekday::Sat.number_from_monday(), Weekday::Thu.number_from_monday()) == 5);
		assert!(get_day_index(Weekday::Sat.number_from_monday(), Weekday::Fri.number_from_monday()) == 6);
	}

	#[test]
	fn forecast_indices() {
		assert_eq!(
			get_indices(&HashSet::from([Forecast::tu, Forecast::we, Forecast::sa])),
			// [0] = current day; [1..7] = week days; [7] = week overview ; [8] = disable
			[false, true, true, false, false, true, false, false, false]
		);
		assert_eq!(
			get_indices(&HashSet::from([Forecast::mo])),
			get_indices(&HashSet::from([Forecast::day])),
		);
		assert_eq!(
			get_indices(&HashSet::from([Forecast::week])),
			[false, false, false, false, false, false, false, true, false]
		);
		assert_eq!(
			get_indices(&HashSet::from([Forecast::day, Forecast::week])),
			[true, false, false, false, false, false, false, true, false]
		);
		assert_eq!(
			get_indices(&HashSet::from([Forecast::disable])),
			[false, false, false, false, false, false, false, false, true]
		);
	}
}
