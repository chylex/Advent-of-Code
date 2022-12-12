import itertools
import re

from utils.input import read_input_lines

lines = iter(read_input_lines())
initial_stack_configuration_lines = list(itertools.takewhile(lambda line: len(line) > 0, lines))[:-1]

stacks = list()

for line in reversed(initial_stack_configuration_lines):
    for stack_index, character_index in enumerate(range(1, len(line), 4)):
        crate = line[character_index]
        
        if crate == " ":
            continue
        
        for i in range(len(stacks), stack_index + 1):
            stacks.append(list())
        
        stacks[stack_index].append(crate)

for instruction in lines:
    parsed = re.search(r"move (\d+) from (\d+) to (\d+)", instruction)
    
    moved_count = int(parsed.group(1))
    from_column = int(parsed.group(2)) - 1
    to_column = int(parsed.group(3)) - 1
    
    for i in range(moved_count):
        moved_crate = stacks[from_column].pop()
        stacks[to_column].append(moved_crate)

print("Top of each stack: ", end = "")

for stack in stacks:
    print(stack[-1], end = "")
