#include "stdafx.h"
#include "utility.h"

std::string utility::sha256(const std::string str)
{
    unsigned char hash[SHA256_DIGEST_LENGTH];
    SHA256_CTX sha256;
    SHA256_Init(&sha256);
    SHA256_Update(&sha256, str.c_str(), str.size());
    SHA256_Final(hash, &sha256);
    std::stringstream ss;
    for (int i = 0; i < SHA256_DIGEST_LENGTH; i++)
    {
        ss << std::hex << std::setw(2) << std::setfill('0') << (int)hash[i];
    }
    return ss.str();
}

#if defined(_WIN32)
std::string utility::random128()
{
    char buf[64] = { 0 };
    GUID guid;

    if (FAILED(CoCreateGuid(&guid)))
        return std::move(std::string(""));

    sprintf_s(buf,
        "%08X%04X%04x%02X%02X%02X%02X%02X%02X%02X%02X",
        guid.Data1, guid.Data2, guid.Data3,
        guid.Data4[0], guid.Data4[1], guid.Data4[2],
        guid.Data4[3], guid.Data4[4], guid.Data4[5],
        guid.Data4[6], guid.Data4[7]);

    return std::move(std::string(buf));
}
#elif defined(__linux__)
std::string utility::random128()
{
    char buf[64] = { 0 };

    uuid_t uu;
    uuid_generate(uu);

    int32_t index = 0;
    for (int32_t i = 0; i < 16; i++)
    {
        int32_t len = sprintf_s(buf + index, "%02X", uu[i]);
        if (len < 0)
            return std::move(std::string(""));
        index += len;
    }

    return std::move(std::string(buf));
}
#endif
