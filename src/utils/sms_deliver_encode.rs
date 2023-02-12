/*
 * mm-radio HAL (https://github.com/GloDroid/mm-radio)
 *
 * SPDX-License-Identifier: Apache-2.0
 * Copyright (C) 2023 The GloDroid Project
 */

// Ref: http://www.sendsms.cn/download/SMS_PDU-mode.PDF

use crate::utils::pdu_helpers::address::address_to_pdu;
use crate::utils::pdu_helpers::time::Timestamp;

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

#[no_mangle]
pub extern "C" fn sms_deliver_encode_c(
    address: *const c_char,
    text: *const c_char,
    timestamp: *const c_char,
    out_pdu: *mut *mut c_char,
) -> c_int {
    assert!(!address.is_null());
    assert!(!text.is_null());
    assert!(!timestamp.is_null());
    let address = unsafe { CStr::from_ptr(address) }.to_str().unwrap();
    let text = unsafe { CStr::from_ptr(text) }.to_str().unwrap();
    let timestamp = unsafe { CStr::from_ptr(timestamp) }.to_str().unwrap();

    let pdu = sms_deliver_encode(address, text, timestamp);
    let pdu = CString::new(pdu).unwrap();

    unsafe {
        *out_pdu = pdu.into_raw();
    }

    0
}

pub(crate) fn sms_deliver_encode(address: &str, text: &str, timestamp: &str) -> String {
    let text_len = text.chars().count();

    let mut pdu = String::new();
    // SMS-DELIVER (TP-MTI = 0b00) (TP-MMS = 0b0) (TP-SRI = 0b0) (TP-UDHI = 0b0) (TP-RP = 0b0)
    pdu.push_str("00");
    // Address
    pdu.push_str(&address_to_pdu(address, false).unwrap());
    // Protocol identifier
    pdu.push_str("00");
    // Data coding scheme (UCS2)
    pdu.push_str("08");
    // Timestamp
    pdu.push_str(&Timestamp::from_mm_format(timestamp).to_pdu());
    // User data length
    pdu.push_str(&format!("{:02X}", text_len * 2));
    // User data
    for c in text.chars() {
        pdu.push_str(&format!("{:04X}", c as u16));
    }

    pdu
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sms_deliver_encode() {
        let address = "diafaan";
        let text = "diafaan.com";
        let timestamp = "2011-01-11 13:25:41+00";
        let pdu = sms_deliver_encode(address, text, timestamp);
        assert_eq!(
            pdu,
            "000ED0E474D81C0EBB0100081110113152140016006400690061006600610061006E002E0063006F006D"
        );

        let address = "123456789";
        let text = "Послуга тимчасово недоступна. Спробуйте будь ласка пізніше.";
        let timestamp = "2019-02-15 21:52:19+00";
        let pdu = sms_deliver_encode(address, text, timestamp);
        assert_eq!(pdu, "00099121436587F900089120511225910076041F043E0441043B044304330430002004420438043C044704300441043E0432043E0020043D04350434043E044104420443043F043D0430002E00200421043F0440043E043104430439044204350020043104430434044C0020043B04300441043A04300020043F04560437043D045604480435002E");
    }
}
