cmake_minimum_required(VERSION 3.13)

project(machine_learning)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_FLAGS "-O2 -march=native -mavx -funroll-loops")

file(GLOB SOURCES "src/*.cpp")

add_executable(machine_learning ${SOURCES})
