/*
 * mm-radio HAL (https://github.com/GloDroid/mm-radio)
 *
 * SPDX-License-Identifier: Apache-2.0
 * Copyright (C) 2023 The GloDroid Project
 */

use super::div_round_up;

pub(crate) fn gsm7_pdu_from_string(utf: &str) -> Result<String, Gsm7Error> {
    let vec = gsm7_from_string(utf)?;
    let mut result = String::new();
    for byte in vec {
        result.push_str(&format!("{byte:02X}"));
    }
    Ok(result)
}

pub(crate) fn gsm7_pdu_to_string(pdu: &str) -> Result<String, Gsm7Error> {
    let mut bytes = Vec::new();
    for i in 0..pdu.len() / 2 {
        let byte = u8::from_str_radix(&pdu[i * 2..i * 2 + 2], 16).unwrap();
        bytes.push(byte);
    }
    gsm7_to_string(&bytes)
}

// Ref: https://en.wikipedia.org/wiki/GSM_03.38

const GSM7_ESC: u8 = 0x1B;

#[rustfmt::skip]
static GSM7_CHARSET: [char; 128] = [
    '@', '£', '$', '¥', 'è', 'é', 'ù', 'ì',  'ò', 'Ç', '\n', 'Ø',    'ø', '\r', 'Å', 'å',
    'Δ', '_', 'Φ', 'Γ', 'Λ', 'Ω', 'Π', 'Ψ',  'Σ', 'Θ', 'Ξ',  '\x1B', 'Æ', 'æ',  'ß', 'É',
    ' ', '!', '"', '#', '¤', '%', '&', '\'', '(', ')', '*',  '+',    ',', '-',  '.', '/',
    '0', '1', '2', '3', '4', '5', '6', '7', '8',  '9', ':',  ';',    '<', '=',  '>', '?',
    '¡', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',  'I', 'J',  'K',    'L', 'M',  'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',  'Y', 'Z',  'Ä',    'Ö', 'Ñ',  'Ü', '§',
    '¿', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',  'i', 'j',  'k',    'l', 'm',  'n', 'o',
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',  'y', 'z',  'ä',    'ö', 'ñ',  'ü', 'à',
];

#[derive(Debug, PartialEq)]
pub enum Gsm7Error {
    InvalidData,
    InvalidEscapeSequence(u8),
    UnexpectedEndOfData,
}

fn gsm7_to_string(gsm7: &[u8]) -> Result<String, Gsm7Error> {
    let mut bit_start = 0;
    let mut result = String::new();
    while bit_start < gsm7.len() * 8 {
        if bit_start > gsm7.len() * 8 - 7 {
            break;
        }
        let septet = septet_read(bit_start, gsm7);
        bit_start += 7;
        if septet == GSM7_ESC {
            if bit_start > gsm7.len() * 8 - 7 {
                return Err(Gsm7Error::UnexpectedEndOfData);
            }
            let septet = septet_read(bit_start, gsm7);
            bit_start += 7;
            match septet {
                0x0A => result.push('\x0C'),
                0x14 => result.push('^'),
                0x28 => result.push('{'),
                0x29 => result.push('}'),
                0x2F => result.push('\\'),
                0x3C => result.push('['),
                0x3D => result.push('~'),
                0x3E => result.push(']'),
                0x40 => result.push('|'),
                0x65 => result.push('€'),
                _ => return Err(Gsm7Error::InvalidEscapeSequence(septet)),
            }
        } else if let Some(c) = GSM7_CHARSET.get(septet as usize) {
            result.push(*c);
        } else {
            return Err(Gsm7Error::InvalidData);
        }
    }
    Ok(result)
}

fn gsm7_from_string(utf8: &str) -> Result<Vec<u8>, Gsm7Error> {
    let mut bit_start = 0;
    let mut result = vec![0; div_round_up(utf8.len() * 7, 8)];
    for c in utf8.chars() {
        if let Some(septet) = GSM7_CHARSET.iter().position(|&x| x == c) {
            septet_write_with_inc(&mut bit_start, &mut result, septet as u8);
        } else {
            match c {
                '\x0C' => septet_write_esc_with_inc(&mut bit_start, &mut result, 0x0A),
                '^' => septet_write_esc_with_inc(&mut bit_start, &mut result, 0x14),
                '{' => septet_write_esc_with_inc(&mut bit_start, &mut result, 0x28),
                '}' => septet_write_esc_with_inc(&mut bit_start, &mut result, 0x29),
                '\\' => septet_write_esc_with_inc(&mut bit_start, &mut result, 0x2F),
                '[' => septet_write_esc_with_inc(&mut bit_start, &mut result, 0x3C),
                '~' => septet_write_esc_with_inc(&mut bit_start, &mut result, 0x3D),
                ']' => septet_write_esc_with_inc(&mut bit_start, &mut result, 0x3E),
                '|' => septet_write_esc_with_inc(&mut bit_start, &mut result, 0x40),
                '€' => septet_write_esc_with_inc(&mut bit_start, &mut result, 0x65),
                _ => return Err(Gsm7Error::InvalidEscapeSequence(c as u8)),
            }
        }
    }
    Ok(result)
}

fn septet_read(bit_start: usize, bytes: &[u8]) -> u8 {
    let byte_start = bit_start / 8;
    let bit_start_in_byte = bit_start % 8;
    let byte = bytes[byte_start];
    let bits = if bit_start_in_byte > 1 {
        let bits1 = byte >> bit_start_in_byte;
        let bits2 = bytes[byte_start + 1] << (8 - bit_start_in_byte);
        bits1 | bits2
    } else {
        byte >> bit_start_in_byte
    };
    bits & 0b01111111
}

fn septet_write(bit_start: usize, bytes: &mut [u8], septet: u8) {
    let byte_start = bit_start / 8;
    let bit_start_in_byte = bit_start % 8;
    if bit_start_in_byte > 1 {
        bytes[byte_start] &= !(0b01111111 << bit_start_in_byte);
        bytes[byte_start + 1] &= !(0b01111111 >> (8 - bit_start_in_byte));
        bytes[byte_start] |= septet << bit_start_in_byte;
        bytes[byte_start + 1] |= septet >> (8 - bit_start_in_byte);
    } else {
        bytes[byte_start] &= !(0b01111111 << bit_start_in_byte);
        bytes[byte_start] |= septet << bit_start_in_byte;
    }
}

fn septet_write_with_inc(bit_start: &mut usize, bytes: &mut [u8], septet: u8) {
    septet_write(*bit_start, bytes, septet);
    *bit_start += 7;
}

fn septet_write_esc_with_inc(bit_start: &mut usize, bytes: &mut [u8], septet: u8) {
    septet_write_with_inc(bit_start, bytes, GSM7_ESC);
    septet_write_with_inc(bit_start, bytes, septet);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_7_bits() {
        let bytes = [0b11011101, 0b11111111, 0b11101111, 0b11111111];
        assert_eq!(septet_read(0, &bytes), 0b01011101);
        assert_eq!(septet_read(1, &bytes), 0b01101110);
        assert_eq!(septet_read(2, &bytes), 0b01110111);
        assert_eq!(septet_read(15, &bytes), 0b01011111);
    }

    #[test]
    fn test_write_7_bits() {
        let mut bytes = vec![0b00000000, 0b00000000, 0b00000000, 0b00000000];
        septet_write(0, &mut bytes, 0b01011101);
        septet_write(1, &mut bytes, 0b01101110);
        septet_write(2, &mut bytes, 0b01110101);
        septet_write(15, &mut bytes, 0b01011111);
        assert_eq!(bytes, vec![0b11010101, 0b10000001, 0b00101111, 0b00000000]);
    }

    #[rustfmt::skip]
    const UFT8_TO_HEXSTRING_OK: &[(&str, &str)] = &[
        ("Hello rust!", "C8329BFD06C9EB737A08"),
        ("mm-radio HAL is here!", "ED764B1E26A7DF206490094ACF41E8B2BC1C02"),
    ];

    #[test]
    fn test_gsm7_pdu_to_string() {
        for (utf8, hexstring) in UFT8_TO_HEXSTRING_OK {
            assert_eq!(gsm7_pdu_to_string(hexstring).unwrap(), *utf8);
        }

        assert_eq!(gsm7_pdu_to_string("1B1B").unwrap_err(), Gsm7Error::InvalidEscapeSequence(54));
    }

    #[test]
    fn test_gsm7_pdu_from_string() {
        for (utf8, hexstring) in UFT8_TO_HEXSTRING_OK {
            assert_eq!(gsm7_pdu_from_string(utf8).unwrap(), *hexstring);
        }

        let utf8 = "Привіт, світ!";
        assert_eq!(gsm7_pdu_from_string(utf8).unwrap_err(), Gsm7Error::InvalidEscapeSequence(31));
    }
}
