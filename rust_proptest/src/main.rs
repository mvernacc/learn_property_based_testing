/* proptest example from
https://altsysrq.github.io/proptest-book/proptest/getting-started.html
*/

fn main() {
    println!("Hello, world!");
}

fn parse_date(s: &str) -> Option<(u32, u32, u32)> {
    if 10 != s.len() { return None; }
    if !s.is_ascii() { return None; }
    if "-" != &s[4..5] || "-" != &s[7..8] { return None; }

    let year = &s[0..4];
    let month = &s[5..7];
    let day = &s[8..10];

    year.parse::<u32>().ok().and_then(
        |y| month.parse::<u32>().ok().and_then(
            |m| day.parse::<u32>().ok().map(
                |d| (y, m, d)
            )
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_parse_date() {
        assert_eq!(None, parse_date("2017-06-1"));
        assert_eq!(None, parse_date("2017-06-170"));
        assert_eq!(None, parse_date("2017006-17"));
        assert_eq!(None, parse_date("2017-06017"));
        assert_eq!(Some((2017, 06, 17)), parse_date("2017-06-17"));
    }

    #[test]
    fn test_unicode_gibberish() {
        assert_eq!(None, parse_date("A00®aA㆐"))
    }

    #[test]
    fn test_october_first() {
        assert_eq!(Some((0, 10, 1)), parse_date("0000-10-01"));
    }

    proptest! {
        #[test]
        fn does_not_crash(s in "\\PC*") {
            parse_date(&s);
        }

        #[test]
        fn parses_all_valid_dates(s in "[0-9]{4}-[0-9]{2}-[0-9]{2}") {
            parse_date(&s).unwrap();
        }

        #[test]
        fn parses_date_back_to_original(
            y in 0u32..10000,
            m in 1u32..13,
            d in 1u32..32
        ) {
            let (y2, m2, d2) = parse_date(
                &format!("{:04}-{:02}-{:02}", y, m, d)
            ).unwrap();
            prop_assert_eq!((y, m, d), (y2, m2, d2));
        }
    }
}
