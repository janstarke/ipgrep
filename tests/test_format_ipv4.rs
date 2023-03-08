use colored::Colorize;
use ipgrep::{format_ipv4, IpFilter};

#[test]
fn test_loopback1() {
    assert!(format_ipv4(
        &vec![][..],
        &vec![IpFilter::Loopback][..],
        &vec![][..],
        "127.0.0.1"
    )
    .is_some());
}

#[test]
fn test_loopback2() {
    #[allow(clippy::to_string_in_format_args)]
    let expected = format!("abc {}", "127.0.0.1".bright_purple().to_string());
    assert_eq!(
        format_ipv4(
            &vec![][..],
            &vec![IpFilter::Loopback][..],
            &vec![][..],
            "abc 127.0.0.1"
        ),
        Some(expected)
    );
}

#[test]
fn test_loopback3() {
    #[allow(clippy::to_string_in_format_args)]
    let expected = format!("abc {} def", "127.0.0.1".bright_purple().to_string());
    assert_eq!(
        format_ipv4(
            &vec![][..],
            &vec![IpFilter::Loopback][..],
            &vec![][..],
            "abc 127.0.0.1 def"
        ),
        Some(expected)
    );
}

#[test]
fn test_loopback4() {
    assert!(format_ipv4(
        &vec![][..],
        &vec![IpFilter::Loopback][..],
        &vec![][..],
        "hello, world"
    )
    .is_none());
}

#[test]
fn test_loopback5() {
    assert!(format_ipv4(
        &vec![][..],
        &vec![IpFilter::Loopback][..],
        &vec![][..],
        "192.168.0.1"
    )
    .is_none());
}

#[test]
fn test_loopback6() {
    #[allow(clippy::to_string_in_format_args)]
    let expected = format!(
        "abc {} def {}",
        "127.0.0.1".bright_purple().to_string(),
        "192.168.0.1".bright_purple().to_string()
    );
    assert_eq!(
        format_ipv4(
            &vec![][..],
            &vec![IpFilter::Loopback][..],
            &vec![][..],
            "abc 127.0.0.1 def 192.168.0.1"
        ),
        Some(expected)
    );
}

#[test]
fn test_loopback7() {
    assert!(format_ipv4(
        &vec![IpFilter::Private][..],
        &vec![IpFilter::Loopback][..],
        &vec![][..],
        "abc 127.0.0.1 def 192.168.0.1"
    )
    .is_none());
}
