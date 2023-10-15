extern crate ddc;

use ddc::short_version;

#[test]
fn test_new_short_version() {
    assert_eq!(short_version(18, 7, 2017), "9♥7♣5♣7♣");
    assert_eq!(short_version(19, 7, 2017), "10♥8♣6♣7♣");
    assert_eq!(short_version(27, 2, 2011), "Q♠K♠K♣A♣");
    assert_eq!(short_version(28, 2, 2011), "K♠A♦A♣A♣");
    assert_eq!(short_version(1, 3, 2011), "A♦A♦A♣A♣");
    assert_eq!(short_version(27, 2, 2012), "Q♠K♠K♣2♣");
    assert_eq!(short_version(28, 2, 2012), "K♠A♦A♣2♣");
    assert_eq!(short_version(29, 2, 2012), "JoA♦A♣2♣");
    assert_eq!(short_version(1, 3, 2012), "A♦A♦A♣2♣");
    assert_eq!(short_version(28, 2, 2013), "K♠A♦A♣3♣");
    assert_eq!(short_version(1, 3, 2013), "A♦A♦A♣3♣");
    assert_eq!(short_version(28, 2, 2014), "K♠A♦A♣4♣");
    assert_eq!(short_version(1, 3, 2014), "A♦A♦A♣4♣");
    assert_eq!(short_version(28, 2, 2015), "K♠A♦A♣5♣");
}

#[test]
fn test_invalid_date() {
    assert_eq!(short_version(29, 2, 2015), "Invalid date");
    assert_eq!(short_version(30, 22, 2015), "Invalid date");
}
