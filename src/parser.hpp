#pragma once

#include <string>
#include <map>


/**
 * @brief Parses the arguments passed as parameters, and returns a map of the arguments parsed
 * 
 * @param argc The amount of arguments to program received
 * @param argv The array of args
 * @param parsed The map of parsed arguments, to be modified by the function
 */
void parse_arguments(int argc, char* argv[], std::map<std::string, std::string> parsed);