# Comprehensive Password Generator
Program to generate a password that:
	does not repeat characters
	makes sure at least one of each type of character is present (uppercase lowercase, number, and exceptable special character if needed)
	
It should work cross-platform (tested only on windows).
There is 1 known issue: 
	time(NULL) returns unsigned int (probably due to the year 2038 problem with UNIX timestamp). srand() is expecting a signed int. Compiler will spit out a warning, but this issue will not break the program until unix timestamp exceeds unsigned int range
