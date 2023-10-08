use chrono::{NaiveDate, Datelike, Duration};

fn is_leap_year(year: i32) -> bool {
	(year%400 == 0) || (year%4 == 0 && year%100 != 0)
}

fn fix_day(year: i32, day: u32) -> u32 {
	let leap: u32 = is_leap_year(year) as u32;
	if day > 60 - leap {
		return day - 60
	}
	day + 305
}

fn fix_year(year: i32) -> i32 {
	if year < 1790 {
		return 1790 - year
	}
	year - 1790
}

fn suit_year(year: i32) -> usize {
	((fix_year(year) / 13) % 4) as usize
}

fn card_year(year: i32) -> usize {
	(fix_year(year) % 13) as usize
}

fn leap_year_int(year: i32) -> u32 {
	if is_leap_year(year) {
		return 1
	}
	0
}

fn subtract_364_days(day: u32, month: u32, year: i32) -> i32 {
    let mut d: Option<NaiveDate> = NaiveDate::from_ymd_opt(year, month, day);
		if d.is_some() {
			d = d.map(|x| x - Duration::days(364));
			if d.is_some() {
				return d.unwrap().year() as i32
			}
		}
		return 0
}

fn seasons(day: u32, month: u32, year: i32) -> usize {
	let leap: u32 = leap_year_int(subtract_364_days(day, month, year)) as u32;
	if day <= (62 - leap) {
		return 1 as usize
	}
	if day <= (154 - leap) {
		return 2 as usize
	}
	if day <= (247 - leap) {
		return 3 as usize
	}
	if day <= (338 - leap) {
		return 0 as usize
	}
	if day <= (367 - leap) {
		return 1 as usize
	}
	1
}

fn card_month(day: u32) -> usize {
	((day / 28) % 13) as usize
}

fn suit_week(day: u32) -> usize {
	(((day / 7) / 13) % 4) as usize
}

fn card_week(day: u32) -> usize {
	((day / 7) % 13) as usize
}

fn suit_day(day: u32) -> usize {
	if day == 0 {
		return 4
	}
	(((day - 1) / 13) % 4) as usize
}

fn card_day(day: u32) -> usize {
	if day == 0 {
		return 13
	}
	((day - 1) % 13) as usize
}

fn feb(day: u32, year: i32) -> bool {
	day <= (28 + leap_year_int(year))
}

fn valid_date(day: u32, month: u32, year: i32) -> bool {
	if day < 1 || day > 31 || year == 0 || month < 1 || month > 12 {
		return false
	}
	if (month == 1 || month == 3 || month == 5 || month == 7 || month == 8 || month == 10 || month == 12) && day <= 31 {
		return true
	}
	if (month == 4 || month == 6 || month == 9 || month == 11) && day <= 30 {
		return true
	}
	if month == 2 {
		return feb(day, year)
	}
	false
}

fn day_of_year(day: u32, month: u32, year: i32) -> u32 {
	if !valid_date(day, month, year) {
		return 0
	}
	count_days(day, month, year)
}

fn count_days(day: u32, month: u32, year: i32) -> u32 {
	let leap: u32 = leap_year_int(year) as u32;
	if  month == 1 {
		return day
    }
	else if  month == 2 {
		return day + 31
    }
	else if  month == 3 {
		return day + 59 + leap
    }
	else if  month == 4 {
		return day + 90 + leap
    }
	else if  month == 5 {
		return day + 120 + leap
    }
	else if  month == 6 {
		return day + 151 + leap
    }
	else if  month == 7 {
		return day + 181 + leap
    }
	else if  month == 8 {
		return day + 212 + leap
    }
	else if  month == 9 {
		return day + 243 + leap
    }
	else if  month == 10 {
		return day + 273 + leap
    }
	else if  month == 11 {
		return day + 304 + leap
    }
	else if  month == 12 {
		return day + 334 + leap
    }
    0
}

//ShortVersion return abbr version of the frode calendar like 1O1O1P2P
pub fn short_version(day: u32, month: u32, year: i32) -> String {
	if !valid_date(day, month, year) {
		return String::from("");
	}

	let cards = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "Jo", "Jd"];
	let suits = vec!['O', 'P', 'C', 'E', '_'];
	let days: u32 = fix_day(year, day_of_year(day, month, year));
	let mut output = cards[card_day(days)].to_owned();
    output.push(suits[suit_day(days)]);
	output.push_str(cards[card_week(days)]);
    output.push(suits[suit_week(days)]);
	output.push_str(cards[card_month(days)]);
    output.push(suits[seasons(day, month, year)]);
	output.push_str(cards[card_year(year)]);
    output.push(suits[suit_year(year)]);
    output
}

//LongVersion return long version of the frode calendar portuguese language
pub fn long_version(day: u32, month: u32, year: i32) -> String {
	if !valid_date(day, month, year) {
		return "".to_string()
	}

	let cards = vec!["As", "Dois", "Tres", "Quatro", "Cinco", "Seis", "Sete", "Oito", "Nove", "Dez",
		            "Valete", "Dama", "Rei", "do Curinga"];
	let suites = vec![" de ouros", " de paus", " de copas", " de espadas"];

	let days = fix_day(year, day_of_year(day, month, year));

	let mut output: String = "\n\tDia ".to_string();
    output.push_str(cards[card_day(days)]);
    output.push_str(suites[suit_day(days)]);
	output.push_str("\n\tSemana ");
    output.push_str(cards[card_week(days)]);
    output.push_str(suites[suit_week(days)]);
	output.push_str("\n\tMes ");
    output.push_str(cards[card_month(days)]);
    output.push_str(" estacao");
    output.push_str(suites[seasons(day, month, year)]);
	output.push_str("\n\tAno ");
    output.push_str(cards[card_year(year)]);
    output.push_str(suites[suit_year(year)]);
	output.push_str("\n\t");
    output.push_str(&day.to_string());
    output.push_str("/");
    output.push_str(&month.to_string());
    output.push_str("/");
    output.push_str(&year.to_string());
    output.push_str(" e dia numero ");
    output.push_str(&days.to_string());
    output
}

