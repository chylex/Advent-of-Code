cmake_minimum_required(VERSION 3.20)

project(AOC)

enable_language(ASM_NASM)

set(CMAKE_CXX_FLAGS  "${CMAKE_CXX_FLAGS} /W4 /WX")
set(CMAKE_NASM_LINK_EXECUTABLE "ld <LINK_FLAGS> <OBJECTS> -o <TARGET> <LINK_LIBRARIES>")
set(CAN_USE_ASSEMBLER TRUE)

add_subdirectory(01)
add_subdirectory(02)
