These are my solutions to [Advent of Code](https://adventofcode.com) puzzles!

This repository is likely going to be an utter mess of programming languages. If you're interested, read below for the list of used languages, and some info to hopefully get them working on your machine.

I have included Run Configurations for JetBrains IDEs, so if you use the appropriate IDE or IntelliJ language plugin, you should see each solved day in the Run/Debug Configurations menu.

# Languages

## \[2021\] Kotlin

The repository contains a Gradle project (`build.gradle.kts`) that sets up every day as a module. You should be able to load the Gradle project into [IntelliJ IDEA](https://www.jetbrains.com/idea/).

The source code is in `main.kt`. The run configuration executes the `main()` method in this file.

## \[2020\] Rust

The repository contains a Cargo project (`2020/Cargo.toml`) that sets up every day as a binary target that can be launched with `cargo run --bin <day>`, for ex. `cargo run --bin 01`. You should be able to load the Cargo project into [CLion](https://www.jetbrains.com/clion/).

The source code is in `main.rs`. The run configuration executes the `main()` function in this file.

## \[2015\] NASM x64 Assembly

The repository contains a CMake project (`2015/CMakeLists.txt`) in the respective year's folder, which sets up every day as a CMake subproject. You should be able to load the CMake project into [CLion](https://www.jetbrains.com/clion/), as long as you have a toolchain named `Visual Studio x64` set to use the `amd64` architecture.

The source code is in `main.c`, which is either in the puzzle's own folder, or in `utils` if no adjustments are needed. By default, `main.c` reads the whole input file into a buffer, and passes it as a parameter to the `entryPoint` function defined in `main.asm` which implements the logic and output of each puzzle.

Note that everything is targeted for Windows and assembly is not portable, so running on a different OS will most likely require some changes. You will need to:

1. Install [Visual Studio](https://visualstudio.microsoft.com/) with `MSVC x64/x86 Build Tools`
2. Install [NASM](https://www.nasm.us/pub/nasm/releasebuilds/?C=M;O=D) (the "Executable only" version will suffice, as long as you setup the system `%PATH%` environment variable to include the folder with `nasm.exe`)

The versions should not matter, but I used Visual Studio 2019 with `MSVC v142 (14.29)` and `NASM 2.15.05`.

# Solved Days

| Year | Day | Language |
|-----:|----:|----------|
| 2015 |  01 | NASM x64 |
| 2015 |  02 | NASM x64 |
| 2020 |  01 | Rust     |
| 2020 |  02 | Rust     |
| 2020 |  03 | Rust     |
| 2020 |  04 | Rust     |
| 2021 |  01 | Kotlin   |
| 2021 |  02 | Kotlin   |
| 2021 |  03 | Kotlin   |
| 2021 |  04 | Kotlin   |
| 2021 |  05 | Kotlin   |
| 2021 |  06 | Kotlin   |
| 2021 |  07 | Kotlin   |
| 2021 |  08 | Kotlin   |
