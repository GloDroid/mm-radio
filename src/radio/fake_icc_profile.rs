/*
 * mm-radio HAL (https://github.com/GloDroid/mm-radio)
 *
 * SPDX-License-Identifier: Apache-2.0
 * Copyright (C) 2023 The GloDroid Project
 */

use android_hardware_radio_sim::aidl::android::hardware::radio::sim::IccIo::IccIo;
use android_hardware_radio_sim::aidl::android::hardware::radio::sim::IccIoResult::IccIoResult;
use log::info;

fn to_iccio(command: i32, file_id: i32, path: &str, p1: i32, p2: i32, p3: i32) -> IccIo {
    IccIo { command, fileId: file_id, path: path.to_string(), p1, p2, p3, ..Default::default() }
}

fn to_iccio_result(sw1: i32, sw2: i32, payload: &str) -> IccIoResult {
    IccIoResult { sw1, sw2, simResponse: payload.to_string() }
}

pub(crate) fn get_default_sim_io(input: &IccIo) -> IccIoResult {
    let icc_profile: Vec<(IccIo, IccIoResult)> = vec![
        (
            to_iccio(0xc0, 0x2f00, "3F00", 0x0, 0x0, 0x0f),
            to_iccio_result(144, 0, "621A8205422100300483022F008A01058B032F0601800200C08801F0"),
        ),
        (
            to_iccio(0xb2, 0x2f00, "3F00", 0x1, 0x4, 0x30),
            to_iccio_result(
                144,
                0,
                "61184F10A0000003431002FF86FF0389FFFFFFFF50044353494DFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
            ),
        ),
        (
            to_iccio(0xb2, 0x2f00, "3F00", 0x2, 0x4, 0x30),
            to_iccio_result(
                144,
                0,
                "61184F10A0000003431002FF86FF0389FFFFFFFF50044353494DFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
            ),
        ),
        (
            to_iccio(0xb2, 0x2f00, "3F00", 0x3, 0x4, 0x30),
            to_iccio_result(
                144,
                0,
                "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
            ),
        ),
        (
            to_iccio(0xb2, 0x2f00, "3F00", 0x4, 0x4, 0x30),
            to_iccio_result(
                144,
                0,
                "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
            ),
        ),
        (to_iccio(0xc0, 0x2fe2, "3F00", 0x0, 0x0, 0x0f), to_iccio_result(144, 0, "000082020000040000000000000000")),
        (to_iccio(0xb0, 0x2fe2, "3F00", 0x0, 0x0, 0x0a), to_iccio_result(144, 0, "9883")),
        (to_iccio(0xb0, 0x2fe2, "3F00", 0x0, 0x0, 33282), to_iccio_result(144, 0, "9883")),
        (
            to_iccio(0xc0, 0x2f05, "3F00", 0x0, 0x0, 0x0f),
            to_iccio_result(144, 0, "62178202412183022F058A01058B032F060280020004880128"),
        ),
        (to_iccio(0xb0, 0x2f05, "3F00", 0x0, 0x0, 0x04), to_iccio_result(144, 0, "FFFFFFFF")),
        // USIM
        (
            to_iccio(0xc0, 0x6f40, "3F007FFF", 0x0, 0x0, 0x0f),
            to_iccio_result(144, 0, "621982054221001C0283026F408A01058B036F0605800200388800"),
        ),
        (
            to_iccio(0xb2, 0x6f40, "3F007FFF", 0x1, 0x4, 0x1c),
            to_iccio_result(144, 0, "000000000000000000000000000007915155214365F7FFFFFFFFFFFF"),
        ),
        (
            to_iccio(0xc0, 0x6fc9, "3F007FFF", 0x0, 0x0, 0x0f),
            to_iccio_result(144, 0, "62198205422100050183026FC98A01058B036F0602800200058800"),
        ),
        (to_iccio(0xb2, 0x6fc9, "3F007FFF", 0x1, 0x4, 0x05), to_iccio_result(144, 0, "0100000000")),
        (
            to_iccio(0xc0, 0x6fc7, "3F007FFF", 0x0, 0x0, 0x0f),
            to_iccio_result(144, 0, "621982054221001C0283026F408A01058B036F06058002003E8800"),
        ),
        (
            to_iccio(0xb2, 0x6fc7, "3F007FFF", 0x1, 0x4, 0x1c),
            to_iccio_result(144, 0, "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF07915155674523F1FFFFFFFFFFFF"),
        ),
        (
            to_iccio(0xc0, 0x6FAD, "3F007FFF", 0x0, 0x0, 0x0f), // EF_AD
            to_iccio_result(144, 0, "62178202412183026FAD8A01058B036F060180020004880118"),
        ),
        (to_iccio(0xb2, 0x6FAD, "3F007FFF", 0x0, 0x0, 0x04), to_iccio_result(144, 0, "00000003")),
        (
            to_iccio(0xc0, 0x6FCA, "3F007FFF", 0x0, 0x0, 0x0f), // EF_MWIS
            to_iccio_result(144, 0, "62198205422100050183026FCA8A01058B036F060E800200058800"),
        ),
        (to_iccio(0xb2, 0x6FCA, "3F007FFF", 0x1, 0x4, 0x05), to_iccio_result(144, 0, "0000000000")),
        (
            to_iccio(0xc0, 0x6F11, "3F007FFF", 0x0, 0x0, 0x0f), // EF_VOICE_MAIL_INDICATOR_CPHS
            to_iccio_result(106, 130, ""),
        ),
        (
            to_iccio(0xc0, 0x6F7B, "3F007FFF", 0x0, 0x0, 0x0f), // EF_FPLMN
            to_iccio_result(144, 0, "621C8202412183026F7BA5038001718A01058B036F06038002001E880168"),
        ),
        (
            to_iccio(0xb2, 0x6F7B, "3F007FFF", 0x0, 0x0, 0x1e),
            to_iccio_result(144, 0, "64F00064F02064F04064F07064F080FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"),
        ),
        // Other
        (to_iccio(0xc0, 0x6f3c, "3F007FFF", 0x0, 0x0, 0x0f), to_iccio_result(144, 0, "0000820500000400000000000001B0")),
        (to_iccio(0xc0, 0x6f05, "3F007FFF", 0x0, 0x0, 0x0f), to_iccio_result(144, 0, "FFFF")),
        (to_iccio(0xc0, 0x6f07, "3F007FFF", 0x0, 0x0, 0x0f), to_iccio_result(144, 0, "0829")),
        (to_iccio(0xc0, 0x6f17, "3F007FFF", 0x0, 0x0, 0x0f), to_iccio_result(106, 130, "")),
        (to_iccio(0xc0, 0x2f05, "3F00", 0x0, 0x0, 0x0f), to_iccio_result(144, 0, "000082020000040000000000000000")),
    ];

    for (icc_io, icc_io_result) in icc_profile {
        if icc_io.command == input.command
            && icc_io.fileId == input.fileId
            && icc_io.path == input.path
            && icc_io.p1 == input.p1
            && icc_io.p2 == input.p2
            && icc_io.p3 == input.p3
        {
            //            info!("ICC IO found: {:?} -> {:?}", input, icc_io_result);
            return icc_io_result;
        }
    }

    info!("ICC IO not found"); //: {:?}", input);

    to_iccio_result(106, 130, "")
}
