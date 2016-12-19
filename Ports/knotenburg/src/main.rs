use std::io;

fn main()
{
	let lines = 5;
	let mut price = 0;
	loop
	{
		println!("Bitte Starthaltestelle eingeben:");
		let mut from_station = String::new();
		io::stdin().read_line(&mut from_station)
		.expect("Leseoperation fehlgeschlagen!");
		let from_station: u16 = match from_station.trim().parse()
		{
			Ok(num) => num,
			Err(_)  => continue,
		};
	
		println!("Bitte Zielhaltestelle eingeben:");
		let mut to_station = String::new();
		io::stdin().read_line(&mut to_station)
		.expect("Leseoperation fehlgeschlagen!");
		let to_station: u16 = match to_station.trim().parse()
		{
			Ok(num) => num,
			Err(_)  => continue,
		};
	
		let max_station = if from_station < to_station {to_station} else {from_station};
		let min_station = if from_station < to_station {from_station} else {to_station};
	
		let line_min = min_station / 10;
		let station_min = min_station % 10;
		let line_max = max_station / 10;
		let station_max = max_station % 10;
		
		println!("lineMin: {}, stationMin: {}", line_min, station_min);
		println!("lineMax: {}, stationMax: {}", line_max, station_max);
		
		
		break;
	}	
}
