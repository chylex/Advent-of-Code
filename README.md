These are my solutions to [Advent of Code](https://adventofcode.com) puzzles!

This repository is likely going to be an utter mess of programming languages. If you're interested, read below for the list of used languages, and some info to hopefully get them working on your machine.

I have included Run Configurations for JetBrains IDEs, so if you use the appropriate IDE or IntelliJ language plugin, you should see each solved day in the Run/Debug Configurations menu.

# Languages

## Kotlin

The repository contains a Gradle project (`build.gradle.kts`) that sets up every day as a module. You should be able to load the Gradle project into [IntelliJ IDEA](https://www.jetbrains.com/idea/).

The source code is in `main.kt`. The run configuration executes the `main()` method in this file.

## NASM x86 Assembly

The repository contains a CMake project (`CMakeLists.txt`) that sets up every day as a CMake subproject. You should be able to load the CMake project into [CLion](https://www.jetbrains.com/clion/).

The source code is in `main.c` with some boilerplate that reads the input file, and calls functions in `main.asm` which implement the logic of each puzzle.

Note that everything is targeted for Windows x86 or WoW64, and assembly is not portable, so running on a different OS will most likely require some changes. You will need to:

1. Install [Visual Studio](https://visualstudio.microsoft.com/) with `MSVC x64/x86 Build Tools`
2. Install [NASM](https://www.nasm.us/pub/nasm/releasebuilds/?C=M;O=D) (the "Executable only" version will suffice, as long as you setup the system `%PATH%` environment variable to include the folder with `nasm.exe`)

The versions should not matter, but I used Visual Studio 2019 with `MSVC v142 (14.29)` and `NASM 2.15.05`.

# Solved Days

| Year | Day | Language |
|-----:|----:|----------|
| 2015 |  01 | NASM x86 |
| 2021 |  01 | Kotlin   |
| 2021 |  02 | Kotlin   |
| 2021 |  03 | Kotlin   |
| 2021 |  04 | Kotlin   |
