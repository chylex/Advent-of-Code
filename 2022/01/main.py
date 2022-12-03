from itertools import groupby

from utils.input import read_input_lines

lines = read_input_lines()
groups = [list(group) for is_group, group in groupby(lines, lambda x: x != '') if is_group]

carrying_calories = ((int(item) for item in group) for group in groups)
carrying_total_calories = [sum(group) for group in carrying_calories]
most_calories_carried = max(carrying_total_calories)

print(f"Most calories carried: {most_calories_carried}")

top3_most_calories_carried = sorted(carrying_total_calories, reverse=True)[:3]
top3_most_calories_carried_total = sum(top3_most_calories_carried)

print(f"Total calories carried by the top 3: {top3_most_calories_carried_total}")
