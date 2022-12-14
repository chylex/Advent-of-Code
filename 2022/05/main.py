import itertools
import re

from utils.input import read_input_lines


class Instruction:
    def __init__(self, line: str):
        parsed = re.search(r"move (\d+) from (\d+) to (\d+)", line)
        
        self.moved_count = int(parsed.group(1))
        self.from_column = int(parsed.group(2)) - 1
        self.to_column = int(parsed.group(3)) - 1


lines = iter(read_input_lines())

initial_stack_configuration_lines = list(itertools.takewhile(lambda line: len(line) > 0, lines))[:-1]
instructions = [Instruction(line) for line in lines]

initial_stacks = list()

for line in reversed(initial_stack_configuration_lines):
    for stack_index, character_index in enumerate(range(1, len(line), 4)):
        crate = line[character_index]
        
        if crate == " ":
            continue
        
        for i in range(len(initial_stacks), stack_index + 1):
            initial_stacks.append(list())
        
        initial_stacks[stack_index].append(crate)


def copy_initial_stacks() -> list[list[str]]:
    return [stack.copy() for stack in initial_stacks]


def print_stacks(title: str, stacks: list[list[str]]):
    print(f"Top of each stack in {title}: ", end = "")
    
    for stack in stacks:
        print(stack[-1], end = "")
    
    print()


stacks = copy_initial_stacks()

for instruction in instructions:
    for _ in range(instruction.moved_count):
        moved_crate = stacks[instruction.from_column].pop()
        stacks[instruction.to_column].append(moved_crate)

print_stacks("part 1", stacks)


stacks = copy_initial_stacks()

for instruction in instructions:
    taken_crates = list()
    
    for _ in range(instruction.moved_count):
        taken_crates.append(stacks[instruction.from_column].pop())
    
    for crate in reversed(taken_crates):
        stacks[instruction.to_column].append(crate)

print_stacks("part 2", stacks)
