#include <cassert>
#include <iostream>
#include <mk_pass.hpp>

void printConfig(mk_pass::PasswordRequirements &config) {
    std::cout << "Config: {\n\t"
              << "length = " << config.length << "\n\t"
              << "decimal = " << config.decimal << "\n\t"
              << "specials = " << config.specials << "\n\t"
              << "firstIsLetter = " << config.firstIsLetter << "\n}"
              << std::endl;
}

int main() {
    mk_pass::PasswordRequirements config = {
        16,   // length
        15,   // decimal
        15,   // specials
        true, // firstIsLetter
    };
    mk_pass::PasswordRequirements expected = {
        16,   // length
        13,   // decimal
        1,    // specials
        true, // firstIsLetter
    };
    mk_pass::PasswordRequirements validated =
        mk_pass::validateRequirements(&config);
    printConfig(config);
    printConfig(validated);
    assert(validated == expected);
    return 0;
}
