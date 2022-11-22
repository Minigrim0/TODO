#pragma once
#include <exception>

const char* MISSING_ARGUMENT_ERROR_STRING = "Missing arguments for '%s'";
const char* BAD_FORMATTING_ERROR_STRING = "Bad formatting for '%s' (Received '%s')";
const char* UNKNOWN_COMMAND_ERROR_STRING = "Unknown command '%s'";


class ParameterException : public std::exception {
    public:
        ParameterException(char* msg):message(msg) {}
        char* what() {
            return message;
        }

    private:
        char* message;
};

class MissingArgumentsException : public ParameterException {
    public:
        MissingArgumentsException(char* msg):ParameterException(msg) {}
};

class BadFormatException : public ParameterException {
    public:
        BadFormatException(char* msg):ParameterException(msg) {}
};

class UnknownCommandException : public ParameterException {
    public:
        UnknownCommandException(char* msg):ParameterException(msg) {}
};
