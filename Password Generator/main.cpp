#include <iostream>
#include <ctime>
#include <cstdlib>
#include <string>
#include "Password.h"
using std::string;
using std::cout;
using std::endl;

int main() {
    srand(time(NULL)); //seed rand()
    int length = 8; //Enter password length
    Password p(true); // pass true to exclude special characters ($%^&)
    p.generatePass(length);
    cout << p.getPassword() << endl;

    return 0;
}
