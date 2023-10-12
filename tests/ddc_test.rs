extern crate ddc;

use ddc::short_version;

#[test]
fn test_new_short_version() {
    assert_eq!(short_version(18, 7, 2017), "9H7C5C7C");
    assert_eq!(short_version(19, 7, 2017), "10H8C6C7C");
    assert_eq!(short_version(27, 2, 2011), "QSKSKC1C");
    assert_eq!(short_version(28, 2, 2011), "KS1D1C1C");
    assert_eq!(short_version(1, 3, 2011), "1D1D1C1C");
    assert_eq!(short_version(27, 2, 2012), "QSKSKC2C");
    assert_eq!(short_version(28, 2, 2012), "KS1D1C2C");
    assert_eq!(short_version(29, 2, 2012), "Jo1D1C2C");
    assert_eq!(short_version(1, 3, 2012), "1D1D1C2C");
    assert_eq!(short_version(28, 2, 2013), "KS1D1C3C");
    assert_eq!(short_version(1, 3, 2013), "1D1D1C3C");
    assert_eq!(short_version(28, 2, 2014), "KS1D1C4C");
    assert_eq!(short_version(1, 3, 2014), "1D1D1C4C");
    assert_eq!(short_version(28, 2, 2015), "KS1D1C5C");
}

#[test]
fn test_invalid_date() {
    assert_eq!(short_version(29, 2, 2015), "Invalid date");
    assert_eq!(short_version(30, 22, 2015), "Invalid date");
}
