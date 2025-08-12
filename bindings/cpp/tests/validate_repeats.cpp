#include <cassert>
#include <cctype>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <mk_pass.hpp>

uint16_t count(char* str_buf, uint16_t len) {
    uint16_t repeatsLength = 0;
    char* repeats = new char[len];
    for (uint16_t i = 0; i < len; ++i) {
        char ch = str_buf[i];
        for (uint16_t j = 0; j < i; ++j) {
            if (str_buf[j] == ch) {
                bool isNoted = false;
                for (uint16_t k = 0; k < repeatsLength; ++k) {
                    if (repeats[k] == ch) {
                        isNoted = true;
                        break;
                    }
                }
                if (!isNoted) {
                    repeats[repeatsLength] = ch;
                    repeatsLength += 1;
                }
            }
        }
    }
    return repeatsLength;
}

int main() {
    mk_pass::PasswordRequirements config = {
        20,   // length
        0,    // decimal
        18,   // specials
        true, // firstIsLetter
        true, // allowRepeats
    };
    // allocate a buffer on the heap. length + 1 allows for null terminator
    char* buf = new char[config.length + 1];
    // initialize the buf so that the null terminator is only at the end
    memset(buf, '\n', config.length);
    buf[config.length] = 0;

    uint16_t len = mk_pass::generatePassword(buf, config);
    std::cout << "Password: " << buf << std::endl;
    assert(len == config.length);

    uint16_t repeats = count(buf, config.length);
    delete[] buf;
    std::cout << "Repeats: " << repeats << std::endl;
    assert((repeats > 0) == config.allowRepeats);

    return 0;
}
