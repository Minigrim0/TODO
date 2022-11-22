#include "exceptions.hpp"

ParameterException::ParameterException(char* msg):message(msg) {}

char* ParameterException::what() {
    return message;
}
