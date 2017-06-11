#ifndef Password_H
#define Password_H
#include <ctime>
#include <cstdlib>
#include <string>
#include <cctype>
#include <array>
using std::islower;
using std::isupper;
using std::isdigit;
using std::isgraph;
using std::string;
using std::array;

class Password {
public:
    Password(bool s = false);
    string getPassword();
    void generatePass(const int &l);
private:
    bool validate(); //ensure password complies with general password constraints
    size_t getMaxCountType();
    size_t findLastlower();
    size_t findLastDigit();
    size_t findLastUpper();
    size_t findLastSpecial();
    string password;
    bool special; //flag to use special characters(@#$&)
    array<bool, 4> validity;
    array<short, 4> countTypes;
    //exceptable special characters
    array<char, 16> specialChars = { 
        '-', '.', '/', '\\', ':', '\'', '+', '&',
        ',', '@', '$', '!', '_' , '#', '%', '~'};
};

#endif // Password_H
