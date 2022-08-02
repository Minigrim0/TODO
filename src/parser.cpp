#include "parser.hpp"

#include <iostream>

void parse_arguments(int argc, char* argv[], std::map<std::string, std::string>){
    for(int i=1;i<argc;i++){  // Skip first argument, which is the program name
        std::cout << argv[i] << std::endl;
    }
}