from itertools import chain, zip_longest

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


total_misplaced_priority = sum(map(get_priority, misplaced_items))
print(f"Total misplaced item priority: {total_misplaced_priority}")

line_iterator = iter(lines)
rucksacks_per_group = zip_longest(line_iterator, line_iterator, line_iterator)
rucksack_items_per_group = (list(set(item) for item in rucksack) for rucksack in rucksacks_per_group)
common_items_per_group = (set.intersection(*rucksack_items) for rucksack_items in rucksack_items_per_group)
badges = chain.from_iterable(common_items_per_group)
total_badge_priority = sum(map(get_priority, badges))
print(f"Total badge priority: {total_badge_priority}")
