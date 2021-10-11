fn main() {
    println!("Hello, world!");
}

fn parse_date(s: &str) -> Option<(u32, u32, u32)> {
    if 10 != s.len() { return None; }
    if "-" != &s[4..5] || "-" != &s[7..8] { return None; }

    let year = &s[0..4];
    let month = &s[6..7];
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

    proptest! {
        #[test]
        fn does_not_crash(s in "\\PC*") {
            parse_date(&s);
        }
    }
}
