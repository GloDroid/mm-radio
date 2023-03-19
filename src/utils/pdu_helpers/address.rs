/*
 * mm-radio HAL (https://github.com/GloDroid/mm-radio)
 *
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2023 The GloDroid Project
 */

/* Ref: https://en.wikipedia.org/wiki/GSM_03.40 */

use super::{align, div_round_up};
use crate::utils::{
    error::Error,
    pdu_helpers::gsm7::{gsm7_pdu_from_string, gsm7_pdu_to_string},
};

pub(crate) fn address_to_pdu(utf8: &str, is_smsc: bool) -> Result<String, Error> {
    let mut result = String::new();
    let (international, utf8) =
        if let Some(number) = utf8.strip_prefix('+') { (true, number) } else { (false, utf8) };

    let alphabetic = !utf8.chars().all(|c| c.is_ascii_digit());
    if alphabetic {
        let pdu = gsm7_pdu_from_string(utf8)?;
        if is_smsc {
            result.push_str(&format!("{:02X}", div_round_up(pdu.chars().count(), 2) + 1));
        } else {
            result.push_str(&format!("{:02X}", pdu.chars().count()));
        }
        result.push_str("D0");
        result.push_str(&pdu);
    } else {
        let pdu = address_numeric_to_pdu(utf8)?;
        if is_smsc {
            result.push_str(&format!("{:02X}", div_round_up(pdu.chars().count(), 2) + 1));
        } else {
            result.push_str(&format!("{:02X}", utf8.chars().count()));
        }
        let ext_ton_npi: u8 = if international { 0b1001_0001 } else { 0b1010_0001 };
        result.push_str(&format!("{ext_ton_npi:02X}"));
        result.push_str(&pdu);
    }

    Ok(result)
}

pub(crate) fn address_from_pdu(pdu: &str, is_smsc: bool) -> Result<(String, usize), Error> {
    let len = pdu.len();
    if len < 2 {
        return Err(Error::new("PDU is too short"));
    }
    let pdu_len = usize::from_str_radix(&pdu[0..2], 16)?;
    if pdu_len == 0 {
        if is_smsc {
            return Ok((String::new(), 2));
        } else {
            return Err(Error::new("PDU len can't be 0"));
        }
    }
    let pdu_type = u8::from_str_radix(&pdu[2..4], 16)?;
    let toa = (pdu_type & 0x70) >> 4;
    let _npi = pdu_type & 0x0F;
    let toa_alphabetic = 0b00000101;
    let toa_international = 0b00000001;

    let pdu_len_chars = if is_smsc {
        pdu_len * 2 + 2 /* 2 = Length octet */
    } else {
        align(pdu_len, 2) + 4 /* 4 = Length + type octets */
    };

    let pdu_number = &pdu[4..pdu_len_chars];

    let utf8 = if toa == toa_alphabetic {
        gsm7_pdu_to_string(pdu_number)?
    } else {
        let utf8 = address_numeric_from_pdu(pdu_number)?;
        if toa == toa_international {
            format!("+{utf8}")
        } else {
            utf8
        }
    };

    Ok((utf8, pdu_len_chars))
}

fn address_numeric_to_pdu(utf8: &str) -> Result<String, Error> {
    let mut result = String::new();
    for c in utf8.chars() {
        match c {
            '0'..='9' => result.push(c),
            _ => return Err(Error::new("Invalid number")),
        }
    }

    if result.len() % 2 == 1 {
        result.push('F');
    }
    let mut pdu = String::new();
    for i in (0..result.len()).step_by(2) {
        pdu.push(result.chars().nth(i + 1).ok_or(Error::noneopt())?);
        pdu.push(result.chars().nth(i).ok_or(Error::noneopt())?);
    }
    Ok(pdu)
}

fn address_numeric_from_pdu(pdu: &str) -> Result<String, Error> {
    let mut result = String::new();
    for c in pdu.chars() {
        match c {
            '0'..='9' | 'F' => result.push(c),
            _ => return Err(Error::new("Invalid HEX char")),
        }
    }
    if result.len() % 2 == 1 {
        return Err(Error::new("Length of HEX string is not even"));
    }
    let mut utf8 = String::new();
    for i in (0..result.len()).step_by(2) {
        utf8.push(result.chars().nth(i + 1).ok_or(Error::noneopt())?);
        utf8.push(result.chars().nth(i).ok_or(Error::noneopt())?);
    }
    if utf8.ends_with('F') {
        utf8.pop();
    }

    Ok(utf8)
}

#[cfg(test)]
mod tests {
    #[warn(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    const PHONE_TO_PDU: &[(&str, &str)] = &[
        ("+46708251358", "0B916407281553F8"),
        ("+467082513587", "0C91640728155378"),
        ("Hastalavista", "16D0C8F09C1E6687EDE9393D0C"),
    ];

    // SMSC address has different length encoding
    const PHONE_TO_PDU_SMSC: &[(&str, &str)] = &[
        ("+46708251358", "07916407281553F8"),
        ("+467082513587", "0791640728155378"),
        ("Hastalavista", "0CD0C8F09C1E6687EDE9393D0C"),
    ];

    #[test]
    fn test_address_converters() {
        for (phone, pdu) in PHONE_TO_PDU {
            assert_eq!(address_to_pdu(phone, false).unwrap(), *pdu);
            assert_eq!(address_from_pdu(pdu, false).unwrap().0, *phone);
        }
        for (phone, pdu) in PHONE_TO_PDU_SMSC {
            assert_eq!(address_to_pdu(phone, true).unwrap(), *pdu);
            assert_eq!(address_from_pdu(pdu, true).unwrap().0, *phone);
        }
    }
}
