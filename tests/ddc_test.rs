extern crate ddc;

use ddc::short_version;

#[test]
fn test_short_version() {
    assert_eq!(short_version(18, 7, 2017), "9C7P5P7P");
    assert_eq!(short_version(19, 7, 2017), "10C8P6P7P");
    assert_eq!(short_version(27, 2, 2011), "QEKEKP1P");
    assert_eq!(short_version(28, 2, 2011), "KE1O1P1P");
    assert_eq!(short_version(1, 3, 2011), "1O1O1P1P");
    assert_eq!(short_version(27, 2, 2012), "QEKEKP2P");
    assert_eq!(short_version(28, 2, 2012), "KE1O1P2P");
    assert_eq!(short_version(29, 2, 2012), "Jo_1O1P2P");
    assert_eq!(short_version(1, 3, 2012), "1O1O1P2P");
    assert_eq!(short_version(28, 2, 2013), "KE1O1P3P");
    assert_eq!(short_version(1, 3, 2013), "1O1O1P3P");
    assert_eq!(short_version(28, 2, 2014), "KE1O1P4P");
    assert_eq!(short_version(1, 3, 2014), "1O1O1P4P");
}