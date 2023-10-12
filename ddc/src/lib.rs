use chrono::{Datelike, Duration, NaiveDate};

enum CardNumber {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

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

enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
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
enum Joker {
    Single,
    Double,
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

enum Card {
    CardNumber(CardNumber, Suit),
    Joker(Joker),
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
            Card::CardNumber(ref card, ref suit) => {
                format!("{}{}", card.to_string(), suit.to_string())
            }
            Card::Joker(ref joker) => format!("{}", joker.to_string()),
        }
    }
}

struct DDCDate {
    day: Card,
    week: Card,
    month: Card,
    year: Card,
}

impl DDCDate {
    fn from(day: u32, month: u32, year: i32) -> Result<DDCDate, &'static str> {
        let d: Option<NaiveDate> = NaiveDate::from_ymd_opt(year, month, day);
        if d.is_none() {
            return Err("Invalid date");
        }

        let date = d.unwrap();
        let days = Self::fix_day(year, date.ordinal());

        Ok(DDCDate {
            day: Card::from(Self::card_day(days), Self::suit_day(days)),
            week: Card::from(Self::card_week(days), Self::suit_week(days)),
            month: Card::from(Self::card_month(days), Self::seasons(date)),
            year: Card::from(Self::card_year(year), Self::suit_year(year)),
        })
    }

    fn to_string(&self) -> String {
        format!(
            "{}{}{}{}",
            self.day.to_string(),
            self.week.to_string(),
            self.month.to_string(),
            self.year.to_string()
        )
    }

    fn is_leap_year(year: i32) -> bool {
        (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)
    }

    fn fix_day(year: i32, day: u32) -> u32 {
        let leap: u32 = Self::is_leap_year(year) as u32;
        if day > 60 - leap {
            return day - 60;
        }
        day + 305
    }

    fn fix_year(year: i32) -> i32 {
				match year {
					1791..=9999 => year - 1790,
					_ => 1790 - year
				}
    }

    fn suit_year(year: i32) -> usize {
        ((Self::fix_year(year) / 13) % 4) as usize
    }

    fn card_year(year: i32) -> usize {
        (Self::fix_year(year) % 13) as usize
    }

    fn fix_season(date: NaiveDate) -> i32 {
        (date - Duration::days(364)).year()
    }

    fn seasons(date: NaiveDate) -> usize {
				if Self::is_leap_year(Self::fix_season(date)) {
					return match date.day() {
							1..=62 => 1 as usize,
							63..=154 => 2 as usize,
							155..=247 => 3 as usize,
							248..=338 => 0 as usize,
							339..=367 => 1 as usize,
							_ => 1,
						}
				}
				match date.day() {
					1..=61 => 1 as usize,
					62..=153 => 2 as usize,
					154..=246 => 3 as usize,
					247..=337 => 0 as usize,
					338..=366 => 1 as usize,
					_ => 1,
				}
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
				match day {
					0 => 4,
					_ => (((day - 1) / 13) % 4) as usize
				}
    }

    fn card_day(day: u32) -> usize {
				match day {
					0 => 13,
					_ => ((day - 1) % 13) as usize
				}
    }
}

pub fn short_version(day: u32, month: u32, year: i32) -> String {
    match DDCDate::from(day, month, year) {
        Ok(calendar) => calendar.to_string(),
        Err(error) => error.to_string(),
    }
}
