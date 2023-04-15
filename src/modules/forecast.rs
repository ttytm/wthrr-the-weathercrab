#[allow(unused_imports)]
use chrono::{Datelike, Local, Weekday};
use std::collections::HashSet;

use super::args::Forecast;

pub fn get_forecast_indices(forecast: &HashSet<Forecast>) -> Vec<usize> {
	get_indices(forecast, Local::now().weekday())
}

fn get_indices(forecast: &HashSet<Forecast>, curr_day: Weekday) -> Vec<usize> {
	let days_from_ref_day = curr_day.number_from_monday();

	// Indices for forecasts that should be rendered. 7 will be used as a special value
	// [0] = current day; [1..7] = week days; [7] = week overview
	// Until there is a more concise solution this is a working and fairly slim approach.
	let mut forecast_indices: Vec<usize> = forecast
		.iter()
		.map(|val| match val {
			Forecast::week => 7,
			Forecast::mo => get_day_index(days_from_ref_day, Weekday::Mon),
			Forecast::tu => get_day_index(days_from_ref_day, Weekday::Tue),
			Forecast::we => get_day_index(days_from_ref_day, Weekday::Wed),
			Forecast::th => get_day_index(days_from_ref_day, Weekday::Thu),
			Forecast::fr => get_day_index(days_from_ref_day, Weekday::Fri),
			Forecast::sa => get_day_index(days_from_ref_day, Weekday::Sat),
			Forecast::su => get_day_index(days_from_ref_day, Weekday::Sun),
			_ => 0,
		})
		.collect();

	forecast_indices.sort();
	forecast_indices
}

// Get the index of a requested day to navigate the api response based on the distance from the current day.
fn get_day_index(days_from_ref_day: u32, forecast_day: Weekday) -> usize {
	(((forecast_day.number_from_monday() as i8 - days_from_ref_day as i8) % 7 + 7) % 7)
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
		let curr_day = Weekday::Mon;
		// Test weekday forcast for same day. E.g. `-f mo` on a monday.
		assert!(
			get_indices(&HashSet::from([Forecast::mo]), curr_day)
				== get_indices(&HashSet::from([Forecast::day]), curr_day)
		);
		// Test week overview combined with a weekday `-f w mo`. Should result in week overview.
		assert!(get_indices(&HashSet::from([Forecast::week]), curr_day) == [7]);
		// Test distance from current day until requested day.
		assert!(get_indices(&HashSet::from([Forecast::tu, Forecast::we, Forecast::sa]), curr_day) == [1, 2, 5]);
		assert!(get_indices(&HashSet::from([Forecast::day, Forecast::week]), curr_day) == [0, 7]);
	}
}
