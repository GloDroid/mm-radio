pub(crate) struct Timestamp {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    tz: i8,
}

impl Timestamp {
    pub(crate) fn from_mm_format(timestamp: &str) -> Timestamp {
        let year = u16::from_str_radix(&timestamp[0..4], 10).unwrap();
        let month = u8::from_str_radix(&timestamp[5..7], 10).unwrap();
        let day = u8::from_str_radix(&timestamp[8..10], 10).unwrap();
        let hour = u8::from_str_radix(&timestamp[11..13], 10).unwrap();
        let minute = u8::from_str_radix(&timestamp[14..16], 10).unwrap();
        let second = u8::from_str_radix(&timestamp[17..19], 10).unwrap();
        let tz = i8::from_str_radix(&timestamp[20..22], 10).unwrap();
        Timestamp {
            year,
            month,
            day,
            hour,
            minute,
            second,
            tz,
        }
    }

    pub(crate) fn to_pdu(&self) -> String {
        let year = format!("{:02}", self.year % 100);
        let month = format!("{:02}", self.month);
        let day = format!("{:02}", self.day);
        let hour = format!("{:02}", self.hour);
        let minute = format!("{:02}", self.minute);
        let second = format!("{:02}", self.second);
        let tz = format!("{:02}", self.tz);
        format!(
            "{}{}{}{}{}{}{}",
            year.chars().rev().collect::<String>(),
            month.chars().rev().collect::<String>(),
            day.chars().rev().collect::<String>(),
            hour.chars().rev().collect::<String>(),
            minute.chars().rev().collect::<String>(),
            second.chars().rev().collect::<String>(),
            tz.chars().rev().collect::<String>()
        )
    }
}

#[test]
fn test_decode_timestamp() {
    let timestamp = Timestamp::from_mm_format("2023-01-17T00:33:51+02");
    assert_eq!(timestamp.year, 2023);
    assert_eq!(timestamp.month, 1);
    assert_eq!(timestamp.day, 17);
    assert_eq!(timestamp.hour, 0);
    assert_eq!(timestamp.minute, 33);
    assert_eq!(timestamp.second, 51);
    assert_eq!(timestamp.tz, 2);
    // assert_eq!(timestamp.to_tp_scts(), "32107100331520");
}
