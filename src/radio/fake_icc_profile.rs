/*
 * mm-radio HAL (https://github.com/GloDroid/mm-radio)
 *
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2023 The GloDroid Project
 */

use android_hardware_radio_sim::aidl::android::hardware::radio::sim::IccIo::IccIo;
use android_hardware_radio_sim::aidl::android::hardware::radio::sim::IccIoResult::IccIoResult;

fn to_iccio(command: i32, file_id: i32, path: &str, p1: i32, p2: i32) -> IccIo {
    IccIo { command, fileId: file_id, path: path.to_string(), p1, p2, ..Default::default() }
}

fn to_iccio_result(sw1: i32, sw2: i32, payload: &str) -> IccIoResult {
    IccIoResult { sw1, sw2, simResponse: payload.to_string() }
}

pub(crate) fn get_default_sim_io(input: &IccIo) -> IccIoResult {
    let icc_profile: Vec<(IccIo, IccIoResult)> = vec![
        (
            to_iccio(0xc0, 0x2fe2, "3F00", 0x0, 0x0),
            to_iccio_result(144, 0, "0000000a2fe204000fa0aa01020000"),
        ),
        (to_iccio(0xb0, 0x2fe2, "3F00", 0x0, 0x0), to_iccio_result(144, 0, "")), /* ICCID Bytes Swapped */
    ];

    for (icc_io, icc_io_result) in icc_profile {
        if icc_io.command == input.command
            && icc_io.fileId == input.fileId
            && icc_io.path == input.path
            && icc_io.p1 == input.p1
            && icc_io.p2 == input.p2
        {
            let mut result = icc_io_result;
            let len = std::cmp::min(input.p3 as usize * 2, result.simResponse.len());
            result.simResponse = result.simResponse.as_str()[..len].to_string();
            return result;
        }
    }

    to_iccio_result(106, 130, "")
}
