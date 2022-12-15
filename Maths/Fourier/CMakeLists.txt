cmake_minimum_required(VERSION 3.10)

project(fourier LANGUAGES CXX)

list(APPEND CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR}/cmake)

set(SFML_INCLUDE_DIR "C:/SFML-2.5.1/include")
set(SFML_DIR "C:/SFML-2.5.1")
set(SFML_LIBS_DIR "C:/SFML-2.5.1/lib")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -O2 -march=native")

add_executable(fourier "src/main.cpp")

find_package(SFML 2.5 
        COMPONENTS 
            system window graphics network audio REQUIRED)

target_link_libraries(fourier PRIVATE
    sfml-graphics
    sfml-window
    sfml-system
    sfml-audio
    sfml-network
)

include_directories(${SFML_INCLUDE_DIR})