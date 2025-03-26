cmake_minimum_required(VERSION 3.10)
project(AlgorithmSolutions)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

set(CMAKE_C_LINK_LIBRARIES "-lm")

file(GLOB SOURCE_FILES *.c)

foreach(source_file ${SOURCE_FILES})
    get_filename_component(target_name ${source_file} NAME_WE)
    add_executable(${target_name} ${source_file})
    
endforeach()