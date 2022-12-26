import collections

from utils.input import read_input_lines

input_sequence = read_input_lines()[0]


def find_position_with_distinct_characters(count: int) -> int:
    buffer = collections.deque(maxlen = count)
    
    for i, c in enumerate(input_sequence):
        buffer.append(c)
        if len(set(buffer)) == count:
            return i + 1
        
    return -1


print(f"Found start-of-packet marker at: {find_position_with_distinct_characters(4)}")
print(f"Found start-of-message marker at: {find_position_with_distinct_characters(14)}")
