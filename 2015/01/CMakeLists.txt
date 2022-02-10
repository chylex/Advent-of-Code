set(PROJECT_NAME AOC_2015_1)

project(${PROJECT_NAME} C ASM_NASM)
enable_language(ASM_NASM)

set(CMAKE_NASM_LINK_EXECUTABLE "ld <LINK_FLAGS> <OBJECTS> -o <TARGET> <LINK_LIBRARIES>")
set(CAN_USE_ASSEMBLER TRUE)

add_executable(${PROJECT_NAME} main.asm ../utils/main.c ../utils/file.h)
