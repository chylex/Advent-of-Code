set(PROJECT_NAME 2015_03)

project(${PROJECT_NAME} C ASM_NASM)

add_executable(${PROJECT_NAME} main.asm part1.asm part2.asm ../utils/main.c ../utils/file.h ../utils/benchmark.h)
