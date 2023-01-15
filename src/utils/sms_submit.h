#pragma once

extern "C" {
int sms_submit_decode_c(const char* smsc_pdu, const char* pdu, char** out_smsc,
                        char** out_destination, char** out_message);
}

struct DecodeSmsSubmitArgs {
    std::string in_smsc_pdu;
    std::string in_pdu;
    std::string out_smsc;
    std::string out_destination;
    std::string out_message;
};

auto smsSubmitDecode(DecodeSmsSubmitArgs& args) -> int {
    char* out_destination = nullptr;
    char* out_message = nullptr;
    char* out_smsc = nullptr;
    auto result = sms_submit_decode_c(args.in_smsc_pdu.c_str(), args.in_pdu.c_str(), &out_smsc,
                                      &out_destination, &out_message);
    if (result != 0) return result;

    args.out_smsc = std::string(out_smsc);
    args.out_destination = std::string(out_destination);
    args.out_message = std::string(out_message);
    free(out_smsc);
    free(out_destination);
    free(out_message);

    return 0;
}
