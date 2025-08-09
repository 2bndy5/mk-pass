#include <cassert>
#include <cctype>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <mk_pass.hpp>

enum CharType {
    Decimal,
    Lowercase,
    Uppercase,
    Special,
};

uint16_t count(char* str_buf, uint16_t len, CharType char_t) {
    uint16_t total = 0;
    for (uint16_t i = 0; i < len; ++i) {
        char ch = str_buf[i];
        switch (char_t) {
        case Decimal:
            total += static_cast<uint16_t>(static_cast<bool>(isdigit(ch)));
            break;
        case Lowercase:
            total += static_cast<uint16_t>(static_cast<bool>(islower(ch)));
            break;
        case Uppercase:
            total += static_cast<uint16_t>(static_cast<bool>(isupper(ch)));
            break;
        case Special:
            total += static_cast<uint16_t>(static_cast<bool>(ispunct(ch)));
            break;
        }
    }
    return total;
}

int main() {
    mk_pass::PasswordRequirements config = {
        16,   // length
        4,    // decimal
        4,    // specials
        true, // firstIsLetter
    };
    // allocate a buffer on the heap. length + 1 allows for null terminator
    char* buf = new char[config.length + 1];
    // initialize the buf so that the null terminator is only at the end
    memset(buf, '0', config.length);
    buf[config.length] = 0;

    uint16_t len = mk_pass::generatePassword(buf, config);
    std::cout << "Password: " << buf << std::endl;

    uint16_t decimal = count(buf, config.length, CharType::Decimal);
    uint16_t special = count(buf, config.length, CharType::Special);
    uint16_t lowercase = count(buf, config.length, CharType::Lowercase);
    uint16_t uppercase = count(buf, config.length, CharType::Uppercase);
    std::cout << "Decimal: " << decimal << ", "
              << "Specials: " << special << ", "
              << "Lowercase: " << lowercase << ", "
              << "Uppercase: " << uppercase << std::endl;

    assert(static_cast<bool>(islower(buf[0]))
           || static_cast<bool>(isupper(buf[0])));
    delete[] buf;
    assert(len == config.length);
    assert(decimal == config.decimal);
    assert(special == config.specials);
    uint16_t letters = lowercase + uppercase;
    assert(letters == (config.length - config.decimal - config.specials));
    return 0;
}
