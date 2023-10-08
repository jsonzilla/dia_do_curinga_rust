use chrono::{NaiveDate, Datelike, Duration};

enum Suit { Diamonds,	Clubs, Hearts, Spades }
enum CardNumber { One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King }
enum Joker { Single, Double }
enum Card {	CardNumber(CardNumber, Suit), Joker(Joker), }

impl CardNumber {
	fn from(card: usize) -> CardNumber {
		match card {
			0 => CardNumber::One,
			1 => CardNumber::Two,
			2 => CardNumber::Three,
			3 => CardNumber::Four,
			4 => CardNumber::Five,
			5 => CardNumber::Six,
			6 => CardNumber::Seven,
			7 => CardNumber::Eight,
			8 => CardNumber::Nine,
			9 => CardNumber::Ten,
			10 => CardNumber::Jack,
			11 => CardNumber::Queen,
			12 => CardNumber::King,
			_ => panic!("Invalid card number"),
		}
	}

	fn to_string(&self) -> &'static str {
		match *self {
			CardNumber::One => "1",
			CardNumber::Two => "2",
			CardNumber::Three => "3",
			CardNumber::Four => "4",
			CardNumber::Five => "5",
			CardNumber::Six => "6",
			CardNumber::Seven => "7",
			CardNumber::Eight => "8",
			CardNumber::Nine => "9",
			CardNumber::Ten => "10",
			CardNumber::Jack => "J",
			CardNumber::Queen => "Q",
			CardNumber::King => "K",
		}
	}
}

impl Suit {
	fn from(suit: usize) -> Suit {
		match suit {
			0 => Suit::Diamonds,
			1 => Suit::Clubs,
			2 => Suit::Hearts,
			3 => Suit::Spades,
			_ => panic!("Invalid suit"),
		}
	}

	fn to_string(&self) -> &'static str {
		match *self {
			Suit::Diamonds => "D",
			Suit::Clubs => "C",
			Suit::Hearts => "H",
			Suit::Spades => "S",
		}
	}
}

impl Joker {
	fn from(joker: usize) -> Joker {
		match joker {
			13 => Joker::Single,
			14 => Joker::Double,
			_ => panic!("Invalid joker"),
		}
	}

	fn to_string(&self) -> &'static str {
		match *self {
			Joker::Single => "Jo",
			Joker::Double => "Jd",
		}
	}
}

impl Card {
	fn from(card: usize, suit: usize) -> Card {
		match card {
			0..=12 => Card::CardNumber(CardNumber::from(card), Suit::from(suit)),
			13..=14 => Card::Joker(Joker::from(card)),
			_ => panic!("Invalid card"),
		}
	}

	fn to_string(&self) -> String {
		match *self {
			Card::CardNumber(ref card, ref suit) => format!("{}{}", card.to_string(), suit.to_string()),
			Card::Joker(ref joker) => format!("{}", joker.to_string()),
		}
	}
}

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

fn subtract_364_days(date: NaiveDate) -> i32 {
	let d = date - Duration::days(364);
	d.year() as i32
}

fn seasons(date: NaiveDate) -> usize {
	let leap: u32 = leap_year_int(subtract_364_days(date)) as u32;
	if date.day() <= (62 - leap) {
		return 1 as usize
	}
	if date.day() <= (154 - leap) {
		return 2 as usize
	}
	if date.day() <= (247 - leap) {
		return 3 as usize
	}
	if date.day() <= (338 - leap) {
		return 0 as usize
	}
	if date.day() <= (367 - leap) {
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

fn day_of_year(date: NaiveDate) -> u32 {
	count_days(date.day(), date.month(), date.year())
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


pub fn short_version(day: u32, month: u32, year: i32) -> String {
	let d: Option<NaiveDate> = NaiveDate::from_ymd_opt(year, month, day);
	if d.is_none() {
		return String::from("");
	}

	let date = d.unwrap();
	let days = fix_day(year, day_of_year(date));

	Card::from(card_day(days), suit_day(days)).to_string() +
		&Card::from(card_week(days), suit_week(days)).to_string() +
		&Card::from(card_month(days), seasons(date)).to_string() +
		&Card::from(card_year(year), suit_year(year)).to_string()
}



