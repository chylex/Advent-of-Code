import collections

from utils.input import read_input_lines

input_sequence = read_input_lines()[0]

buffer = collections.deque(maxlen = 4)
for i, c in enumerate(input_sequence):
    buffer.append(c)
    if len(set(buffer)) == 4:
        print(f"Found start-of-packet marker at: {i + 1}")
        break
