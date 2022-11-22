#include "commands.hpp"

#include <iostream>

void print_help(){
    std::cout << "TODO:" << std::endl;
    std::cout << "  -h,    --help: Prints this help message" << std::endl;
    std::cout << "  -v, --version: Prints the version of the program" << std::endl;
    std::cout << "  -a,     --add: Adds a new task to the database" << std::endl;
    std::cout << "  -l,    --list: Lists all the tasks in the database" << std::endl;
    std::cout << "  -m,  --modify: Modifies a task in the database" << std::endl;
    std::cout << "  -d,  --delete: Deletes a task from the database" << std::endl;
    std::cout << "  -s,  --search: Searches for a task in the database" << std::endl;
    std::cout << "  -c,   --clear: Clears the database" << std::endl;
}
