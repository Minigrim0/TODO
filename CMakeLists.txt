cmake_minimum_required(VERSION 3.13)
project(TODO)
set(CMAKE_CXX_STANDARD 14)

set(SOURCE_FILES main.cpp src/parser.cpp)

# Add executable target with source files listed in SOURCE_FILES variable
add_executable(todo ${SOURCE_FILES})