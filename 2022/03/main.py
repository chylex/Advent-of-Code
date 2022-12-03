from itertools import chain

from utils.input import read_input_lines


def middle(iterable) -> int:
    return int(len(iterable) / 2)


lines = read_input_lines()
compartments = ((set(line[middle(line):]), set(line[:middle(line)])) for line in lines)
misplaced_item_sets = (set.intersection(*compartment) for compartment in compartments)
misplaced_items = chain.from_iterable(misplaced_item_sets)


def get_priority(item: chr) -> int:
    if 'a' <= item <= 'z':
        return ord(item) - ord('a') + 1
    elif 'A' <= item <= 'Z':
        return ord(item) - ord('A') + 27
    else:
        raise ValueError(f"Invalid item: {item}")


total_priority = sum(map(get_priority, misplaced_items))
print(f"Total priority: {total_priority}")
