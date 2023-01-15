#pragma once

#include <cstdlib>
#include <string>

struct SmsSubmitRs;
extern "C" {
// NOLINTNEXTLINE(readability-identifier-naming): Rust codestyle
int sms_deliver_encode_c(const char* address, const char* text, const char* timestamp,
                         char** outPdu);
}

inline auto smsDeliverEncode(const std::string& address, const std::string& text,
                             const std::string& timestamp) -> std::string {
    char* outPdu = nullptr;
    auto result = sms_deliver_encode_c(address.c_str(), text.c_str(), timestamp.c_str(), &outPdu);
    if (result != 0) return "";

    auto pdu = std::string(outPdu);
    auto toFree = std::unique_ptr<char>(outPdu);

    return pdu;
}
