#include "Password.h"

Password::Password(bool s){
    special = s;
    password = "";
    validity.fill(false);
    //lower , upper, number, special
    countTypes.fill((short)0);
}

string Password::getPassword()
{
    return password;
}

void Password::generatePass(const int &l)
{
    for (int i = 0; i < l; i++) {
        char temp = rand() % 94 + 33;
        while (!isalnum(temp) || (special ? !isgraph(temp) : false)) {
            temp = (char)(rand() % 94 + 33);
        }
        if (islower(temp)) {
            validity[0] = true;
            countTypes[0]++;
        }
        else if (isupper(temp)) {
            validity[1] = true;
            countTypes[1]++;
        }
        else if (isdigit(temp)) {
            validity[2] = true;
            countTypes[2]++;
        }
        else if (isgraph(temp)) {
            temp = specialChars[(int)temp % 15];//make valid special char
            validity[3] = true;
            countTypes[3]++;
        }
        if (i) { //avoid repetition
            if (password.find(temp) < password.length())
                i--;
            else password += temp;
        }
        else if (!i) password += temp;
    }
    validate();
}

bool Password::validate() {
    
    char newChar;
    size_t adjust;
    while (!validity[0] || !validity[2] || !validity[2] 
        || (special ? !validity[3] : false)) {
        //set adjust to index of disposable character
        if (getMaxCountType() == 0)
            adjust = findLastlower();
        else if (getMaxCountType() == 1)
            adjust = findLastUpper();
        else if (getMaxCountType() == 2)
            adjust = findLastDigit();
        else if (getMaxCountType() == 1)
            adjust = findLastSpecial();
        //generate needed type of character and validate accordingly
        if (!validity[0]) {
            newChar = (char)(rand() % 26 + 97);
            validity[0] = true;
        }
        else if (!validity[1]) {
            newChar = (char)(rand() % 26 + 65);
            validity[1] = true;
        }
        else if (!validity[2]) {
            newChar = (char)(rand() % 10 + 48);
            validity[2] = true;
        }
        else if (special && !validity[3]) {
            newChar = specialChars[rand() % 15];
            validity[3] = true;
        }
    }
    password[adjust] = newChar;
    return true;
}

size_t Password::getMaxCountType()
{
    size_t max = 0;
    for (int i = 0; i < 4; i++) {
        if (countTypes[max] < countTypes[i] && countTypes[i] > 1)
            max = i;
    }
    return max;
}

size_t Password::findLastlower()
{
    for (size_t i = password.length() - 1; i >= 0; i--) {
        if (islower(password[i]))
            return i;
    }
    return password.length();//if not found
}

size_t Password::findLastDigit()
{
    for (size_t i = password.length() - 1; i >= 0; i--) {
        if (isdigit(password[i]))
            return i;
    }
    return password.length();//if not found
}

size_t Password::findLastUpper()
{
    for (size_t i = password.length() - 1; i >= 0; i--) {
        if (isupper(password[i]))
            return i;
    }
    return password.length();//if not found
}

size_t Password::findLastSpecial()
{
    for (size_t i = password.length() - 1; i >= 0; i--) {
        if (isgraph(password[i]))
            return i;
    }
    return password.length();//if not found
}
