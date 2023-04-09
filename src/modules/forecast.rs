#[allow(unused_imports)]
use chrono::{Datelike, Local, Weekday};
use std::collections::HashSet;

use super::args::Forecast;

pub fn get_indices(forecast: &HashSet<Forecast>) -> Vec<usize> {
	// Indices for forecasts that should be rendered. 7 will be used as a special value
	// [0] = current day; [1..7] = week days; [7] = week overview
	// Until there is a more concise solution this is a working and fairly slim approach.
	let mut forecast_indices: Vec<usize> = vec![];

	#[cfg(not(test))]
	let curr_day_ref = Local::now().weekday().number_from_monday();

	#[cfg(test)]
	// For tests we mock the current day to always be a Monday. If I find a way to mock the system time
	// instead of fixing a specific day, it will be integrated as preferred method.
	// Nevertheless, we will test the `get_day_index` subunit with days other than Monday.
	let curr_day_ref = Weekday::Mon.number_from_monday();

	for val in forecast {
		match val {
			Forecast::day => forecast_indices.push(0),
			Forecast::week => forecast_indices.push(7),
			// Forecast weekdays
			Forecast::mo => forecast_indices.push(get_day_index(curr_day_ref, Weekday::Mon)),
			Forecast::tu => forecast_indices.push(get_day_index(curr_day_ref, Weekday::Tue)),
			Forecast::we => forecast_indices.push(get_day_index(curr_day_ref, Weekday::Wed)),
			Forecast::th => forecast_indices.push(get_day_index(curr_day_ref, Weekday::Thu)),
			Forecast::fr => forecast_indices.push(get_day_index(curr_day_ref, Weekday::Fri)),
			Forecast::sa => forecast_indices.push(get_day_index(curr_day_ref, Weekday::Sat)),
			Forecast::su => forecast_indices.push(get_day_index(curr_day_ref, Weekday::Sun)),
			Forecast::disable => (),
		}
	}

	forecast_indices.sort();
	forecast_indices
}

// Get the index of a requested day to navigate the api response based on the distance from the current day.
fn get_day_index(curr_day_ref: u32, forecast_day: Weekday) -> usize {
	(((forecast_day.number_from_monday() as i8 - curr_day_ref as i8) % 7 + 7) % 7)
		.try_into()
		.unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn index_form_day() {
		// Test to determine the index of the current day when the current day is Monday.
		assert!(get_day_index(Weekday::Mon.number_from_monday(), Weekday::Tue) == 1);
		assert!(get_day_index(Weekday::Mon.number_from_monday(), Weekday::Wed) == 2);
		assert!(get_day_index(Weekday::Mon.number_from_monday(), Weekday::Thu) == 3);
		assert!(get_day_index(Weekday::Mon.number_from_monday(), Weekday::Fri) == 4);
		assert!(get_day_index(Weekday::Mon.number_from_monday(), Weekday::Sat) == 5);
		assert!(get_day_index(Weekday::Mon.number_from_monday(), Weekday::Sun) == 6);
		// Test to determine the index of the current day when the current day is Saturday.
		assert!(get_day_index(Weekday::Sat.number_from_monday(), Weekday::Sun) == 1);
		assert!(get_day_index(Weekday::Sat.number_from_monday(), Weekday::Mon) == 2);
		assert!(get_day_index(Weekday::Sat.number_from_monday(), Weekday::Tue) == 3);
		assert!(get_day_index(Weekday::Sat.number_from_monday(), Weekday::Wed) == 4);
		assert!(get_day_index(Weekday::Sat.number_from_monday(), Weekday::Thu) == 5);
		assert!(get_day_index(Weekday::Sat.number_from_monday(), Weekday::Fri) == 6);
		// If these work, the same principle applies to any other day.
	}

	#[test]
	fn forecast_indices() {
		assert!(get_indices(&HashSet::from([Forecast::mo])) == get_indices(&HashSet::from([Forecast::day])));
		assert!(get_indices(&HashSet::from([Forecast::week])) == [7]);
		assert!(get_indices(&HashSet::from([Forecast::tu, Forecast::we, Forecast::sa])) == [1, 2, 5]);
		assert!(get_indices(&HashSet::from([Forecast::day, Forecast::week])) == [0, 7]);
	}
}
