set(PROJECT_NAME AOC_2015_01)

project(${PROJECT_NAME} C ASM_NASM)

add_executable(${PROJECT_NAME} main.asm ../utils/main.c ../utils/file.h ../utils/benchmark.h)
