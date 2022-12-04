These are my solutions to [Advent of Code](https://adventofcode.com) puzzles!

This repository is likely going to be an utter mess of programming languages. If you're interested, read below for the list of used languages, and instructions to get them working on your machine.

Years and days are organized in folders. Every day contains a source code file named `main` with the solution, and the input file `input.txt`. Some years also contain a `utils` folder with shared code used in each day (usually to read the input file).

The solutions always look for the input file in the working directory, so make sure the working directory is set to the day's folder.

If you use JetBrains IDEs, you can open each year's folder in the appropriate IDE. I have included a Run Configuration for each solved day, so you should see them in the Run/Debug Configurations menu.

# Languages

## \[2022\] Python

The `2022` folder does not have any special project. You can run the `main.py` file in each day's folder directly using Python 3.11 or newer (older versions may also work, but this is not guaranteed).

You should be able to load the `2022` folder into [PyCharm](https://www.jetbrains.com/pycharm/).

## \[2021\] Kotlin

The `2021` folder contains a Gradle project (`build.gradle.kts`) that sets up every day as a source root and task that can be launched with `gradlew <day>` (for ex. `gradlew 01`).

You should be able to load the Gradle project into [IntelliJ IDEA](https://www.jetbrains.com/idea/).

## \[2020\] Rust

The `2020` folder contains a Cargo project (`Cargo.toml`) that sets up every day as a binary target that can be launched with `cargo run --bin <day>` (for ex. `cargo run --bin 01`).

You should be able to load the Cargo project into [CLion](https://www.jetbrains.com/clion/).

## \[2017\] PostgreSQL

The `2017` folder contains a Docker Compose file (`docker-compose.yml`) that launches a local PostgreSQL instance on `127.0.0.1:2017`, with the username `postgres` and password `aoc2017`. The container has the `2017` folder mounted to `/aoc`, so that PostgreSQL can see the input files.

To start the Docker container, enter the `2017` folder and run `docker compose up -d`. To stop and remove the Docker container and its data, run `docker compose down -v`.

You can execute the script for each day and get its output by running the following command. See [psql](https://www.postgresql.org/docs/current/app-psql.html) for the documentation of arguments and flags passed to the `psql` program.
```
# First, execute procedures.sql to set up procedures for turning input files into tables.
docker exec aoc-2017-postgres psql postgres postgres -f /aoc/utils/procedures.sql

# Substitute <day> for the specific day you want to run.
docker exec aoc-2017-postgres psql postgres postgres -Atqf /aoc/<day>/main.sql

# For example:
docker exec aoc-2017-postgres psql postgres postgres -Atqf /aoc/01/main.sql
```

**Every day's script begins by dropping all tables whose name begins with that day.** Don't execute these scripts on any database you care about.

You should be able to load the `2017` folder into [DataGrip](https://www.jetbrains.com/datagrip/), where you can attach the PostgreSQL data source, execute the scripts, and explore the tables created in the process.

## \[2015\] NASM x64 Assembly

The `2015` folder contains a CMake project (`CMakeLists.txt`), which sets up every day as a CMake subproject.

You should be able to load the CMake project into [CLion](https://www.jetbrains.com/clion/), as long as you have a toolchain named `Visual Studio x64` set to use the `amd64` architecture.

The entry point is in `utils/main.c`, which reads the whole input file into a buffer and passes it as a parameter to the `entryPoint` function defined in each day's `main.asm`.

Note that everything is targeted for Windows and assembly is not portable, so running on a different OS will most likely require some changes. To compile the code on Windows, you will need to:

1. Install [Visual Studio](https://visualstudio.microsoft.com/) with `MSVC x64/x86 Build Tools`
2. Install [NASM](https://www.nasm.us/pub/nasm/releasebuilds/?C=M;O=D) (the "Executable only" version will suffice, as long as you setup the system `%PATH%` environment variable to include the folder with `nasm.exe`)

The versions should not matter, but I used Visual Studio 2019 with `MSVC v142 (14.29)` and `NASM 2.15.05`.

# Solved Days

| Year | Day | Language |  /  | Year | Day | Language   |  /  | Year | Day | Language |
|-----:|----:|----------|-----|-----:|----:|------------|-----|-----:|----:|----------|
| 2015 |  01 | NASM x64 |  /  | 2017 |  01 | PostgreSQL |  /  | 2020 |  01 | Rust     |
| 2015 |  02 | NASM x64 |  /  | 2017 |  02 | PostgreSQL |  /  | 2020 |  02 | Rust     |
| 2015 |  03 | NASM x64 |  /  |      |     |            |  /  | 2020 |  03 | Rust     |
|      |     |          |  /  |      |     |            |  /  | 2020 |  04 | Rust     |
|      |     |          |  /  |      |     |            |  /  | 2020 |  05 | Rust     |
|      |     |          |  /  |      |     |            |  /  | 2020 |  06 | Rust     |
|      |     |          |  /  |      |     |            |  /  | 2020 |  07 | Rust     |
|      |     |          |  /  |      |     |            |  /  | 2020 |  08 | Rust     |

| Year | Day | Language |  /  | Year | Day | Language |
|-----:|----:|----------|-----|-----:|----:|----------|
| 2021 |  01 | Kotlin   |  /  | 2022 |  01 | Python   |
| 2021 |  02 | Kotlin   |  /  | 2022 |  02 | Python   |
| 2021 |  03 | Kotlin   |  /  | 2022 |  03 | Python   |
| 2021 |  04 | Kotlin   |  /  |      |     |          |
| 2021 |  05 | Kotlin   |  /  |      |     |          |
| 2021 |  06 | Kotlin   |  /  |      |     |          |
| 2021 |  07 | Kotlin   |  /  |      |     |          |
| 2021 |  08 | Kotlin   |  /  |      |     |          |
