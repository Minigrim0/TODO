#include <iostream>
#include <sqlite3.h>
#include <list>

#include "src/parser.hpp"


int main(int argc, char *argv[]){
    std::list<std::string> parsed;
    parse_arguments(argc, argv, &parsed);
    std::cout << "Parsed arguments: " << std::endl;
    for(std::string arg : parsed){
        std::cout << arg << std::endl;
    }
    return 0;
}
