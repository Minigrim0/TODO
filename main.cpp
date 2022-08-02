#include <iostream>
#include <sqlite3.h>

#include "src/parser.hpp"


int main(int argc, char *argv[])
{
    std::cout << "Hello, World!" << std::endl;
    std::map<std::string, std::string> parsed;
    parse_arguments(argc, argv, parsed);
    return 0;
}