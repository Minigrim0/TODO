#include "parser.hpp"
#include "exceptions.hpp"

#include <iostream>
#include <string.h>

/**
 * @brief Recursively parses the arguments passed as parameters, modifying the map of parsed arguments
 * 
 * @param args a vector of arguments to parse
 * @param parsed the map of parsed arguments, to be modified by the function
 */
void parse_loop(std::list<std::string>* args, std::list<std::string>* parsed){
    std::string arg = args->front();
    args->pop_front();

    if(arg == "-h" || arg == "--help"){
        parsed->push_back("help");
    }else if(arg == "-v" || arg == "--version"){
        parsed->push_back("version");
    }else if(arg == "-a" || arg == "--add"){
        parsed->push_back("add");
        // Check at least one parameter was passed (the name)
        if(args->size() == 0){
            char* exception_string = new char[100];
            sprintf(exception_string, MISSING_ARGUMENT_ERROR_STRING, "add");
            throw MissingArgumentsException(exception_string);
        } else if(args->front().at(0) == '-'){
            char* exception_string = new char[100];
            sprintf(exception_string, BAD_FORMATTING_ERROR_STRING, "add", args->front());
            throw BadFormatException(exception_string);
        }
        parsed->push_back(args->front());
        args->pop_front();
    }else if(arg == "-l" || arg == "--list"){
        parsed->push_back("list");
    }else if(arg == "-m" || arg == "--modify"){
        parsed->push_back("modify");
    }else if(arg == "-d" || arg == "--delete"){
        parsed->push_back("delete");
    }else if(arg == "-s" || arg == "--search"){
        parsed->push_back("search");
    }else if(arg == "-c" || arg == "--clear"){
        parsed->push_back("clear");
    }else{
        char* exception_string = new char[100];
        sprintf(exception_string, UNKNOWN_COMMAND_ERROR_STRING, args->front());
        throw BadFormatException(exception_string);
    }
    if(args->size() > 0){
        parse_loop(args, parsed);
    }
}

/**
 * @brief Prepares the arguments to be parsed
 * 
 * @param argc The amount of arguments to program received
 * @param argv The pointer to the array of args
 * @param parsed the list of parsed arguments, to be modified by the function
 */
void parse_arguments(int argc, char* argv[], std::list<std::string>* parsed){
    // Create a vector of arguments
    std::list<std::string> args;
    for(int i = 1; i < argc; i++){  // Start at 1 to skip the program name
        args.push_back(argv[i]);
    }
    // Check if there are arguments to parse, if not, default to 'help'
    if(args.size() == 0){
        parsed->push_back("--help");
    }
    parse_loop(&args, parsed);
}
