/*
 * mm-radio HAL (https://github.com/GloDroid/mm-radio)
 *
 * SPDX-License-Identifier: Apache-2.0
 * Copyright (C) 2023 The GloDroid Project
 */

// Ref: http://www.sendsms.cn/download/SMS_PDU-mode.PDF
// Ref: https://en.wikipedia.org/wiki/Concatenated_SMS
// Ref: https://en.wikipedia.org/wiki/Short_Message_Peer-to-Peer

use crate::utils::pdu_helpers::address::address_from_pdu;
use crate::utils::pdu_helpers::div_round_up;
use crate::utils::pdu_helpers::gsm7::gsm7_pdu_to_string;

pub(crate) fn sms_submit_decode(in_pdu: &str) -> Option<(String /*number*/, String /*text*/)> {
    let mut pdu = in_pdu;
    let mut message = String::new();

    let first_octet = u8::from_str_radix(&pdu[..2], 16).unwrap();
    // bits 0-1 are TP-MTI (message type indicator)
    let tp_mti = first_octet & 0b00000011;
    // bit 2 is TP-RD (reject duplicates)
    let _tp_rd = (first_octet & 0b00000100) >> 2 != 0;
    // bits 3-4 are TP-VPF (validity period format)
    let tp_vpf = (first_octet & 0b00011000) >> 3;
    // bit 5 is TP-SRR (status report request)
    let _tp_srr = (first_octet & 0b00100000) >> 5 != 0;
    // bit 6 is TP-UDHI (user data header indicator)
    let tp_udhi = (first_octet & 0b01000000) >> 6 != 0;
    // bit 7 is TP-RP (reply path)
    let _tp_rp = (first_octet & 0b10000000) >> 7 != 0;

    if tp_mti != 0b00000001 {
        panic!("Not a SMS-SUBMIT");
    }

    if tp_udhi {
        panic!("User data header not supported");
    }

    pdu = &pdu[2..];
    // TP_MR (message reference - incremented for each message)
    let _tp_mr = u8::from_str_radix(&pdu[..2], 16).unwrap();

    pdu = &pdu[2..];
    // TP_DA
    let addr = address_from_pdu(pdu, false).unwrap();
    // first byte is length of address
    let destination = addr.0;
    pdu = &pdu[addr.1..];

    // TP_PID (protocol identifier)
    let _tp_pid = u8::from_str_radix(&pdu[..2], 16).unwrap();
    pdu = &pdu[2..];

    // TP_DCS (data coding scheme)
    let tp_dcs = u8::from_str_radix(&pdu[..2], 16).unwrap();
    pdu = &pdu[2..];

    if tp_vpf != 0b00000000 {
        // TP_VP (validity period), in some cases can take 7 octets, but we only support 1 octet
        let _tp_vp = u8::from_str_radix(&pdu[..2], 16).unwrap();
        pdu = &pdu[2..];
    }

    // TP_UDL (user data length) in septets
    let tp_udl = usize::from_str_radix(&pdu[..2], 16).unwrap();
    pdu = &pdu[2..];

    // bit 2-3 is encoding
    let tp_dcs_encoding = (tp_dcs & 0b00001100) >> 2;
    let encoding_is_gsm7 = tp_dcs_encoding == 0b00000000;
    let encoding_is_ucs2 = tp_dcs_encoding == 0b00000010;

    // TP_UD (user data)
    if encoding_is_gsm7 {
        let len = div_round_up(tp_udl * 7, 8) * 2;
        let tp_user_data = &pdu[..len];
        message = gsm7_pdu_to_string(tp_user_data).unwrap();
        message = message[..tp_udl].to_string();
        pdu = &pdu[len..];
    } else if encoding_is_ucs2 {
        for _ in 0..tp_udl / 2 {
            let ucs2_u16 = u16::from_str_radix(&pdu[..4], 16).unwrap();
            let ucs2_char = char::decode_utf16([ucs2_u16].iter().cloned()).next().unwrap().unwrap();
            message.push(ucs2_char);
            pdu = &pdu[4..];
        }
    } else {
        panic!("Unsupported encoding");
    }

    if !pdu.is_empty() {
        panic!("PDU not fully parsed");
    }

    Some((destination, message))
}

mod tests {
    #[test]
    fn test_sms_submit_decode() {
        use super::sms_submit_decode;

        let (destination, message) = sms_submit_decode(
            "01000C9183103254769800001DD4F29C0E6A97E7F3F0B90C32BFE5A076BB250F93D36F1032C804",
        )
        .unwrap();
        assert_eq!(destination, "380123456789");
        assert_eq!(message, "Test message for mm-radio HAL");

        let (destination, message) = sms_submit_decode(
            "01000C918310325476980008460422043504410442043E043204350020043F043E043204560434043E043C043B0435043D043D044F0020044300200444043E0440043C04300442045600200055004300530032"
        ).unwrap();
        assert_eq!(destination, "380123456789");
        assert_eq!(message, "Тестове повідомлення у форматі UCS2");

        let (destination, message) = sms_submit_decode("0100038146F3000003D3F61C").unwrap();
        assert_eq!(destination, "643");
        assert_eq!(message, "Sms");

        let (destination, message) =
            sms_submit_decode("01000A812143658709000007C434393D469701").unwrap();
        assert_eq!(destination, "1234567890");
        assert_eq!(message, "Didiche");
    }
}
