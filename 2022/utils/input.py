def read_input_lines() -> list[str]:
    with open("input.txt") as f:
        return [line.rstrip() for line in f.readlines()]
