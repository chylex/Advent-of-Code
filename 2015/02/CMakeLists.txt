set(PROJECT_NAME 2015_02)

project(${PROJECT_NAME} C ASM_NASM)

add_executable(${PROJECT_NAME} main.asm ../utils/main.c ../utils/file.h ../utils/benchmark.h)
