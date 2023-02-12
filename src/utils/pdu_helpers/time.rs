/*
 * mm-radio HAL (https://github.com/GloDroid/mm-radio)
 *
 * SPDX-License-Identifier: Apache-2.0
 * Copyright (C) 2023 The GloDroid Project
 */

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
        let year = timestamp[0..4].parse::<u16>().unwrap();
        let month = timestamp[5..7].parse::<u8>().unwrap();
        let day = timestamp[8..10].parse::<u8>().unwrap();
        let hour = timestamp[11..13].parse::<u8>().unwrap();
        let minute = timestamp[14..16].parse::<u8>().unwrap();
        let second = timestamp[17..19].parse::<u8>().unwrap();
        let tz = timestamp[20..22].parse::<i8>().unwrap();
        Timestamp { year, month, day, hour, minute, second, tz }
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
