from typing import Callable, Iterable

from utils.input import read_input_lines

lines = read_input_lines()
grid = list(map(lambda line: list(map(int, line)), lines))
rows = len(grid)
cols = len(grid[0])


def is_visible(x: int, y: int) -> bool:
    if x <= 0 or y <= 0:
        return True
    if x >= cols - 1 or y >= rows - 1:
        return True
    
    height = grid[y][x]
    
    if all(grid[y][x_adj] < height for x_adj in range(0, x)):
        return True
    
    if all(grid[y][x_adj] < height for x_adj in range(x + 1, cols)):
        return True
    
    if all(grid[y_adj][x] < height for y_adj in range(0, y)):
        return True
    
    if all(grid[y_adj][x] < height for y_adj in range(y + 1, rows)):
        return True
    
    return False


def count_until_and_including(predicate: Callable, iterable: Iterable) -> int:
    count = 0
    
    for item in iterable:
        count += 1
        if not predicate(item):
            break
            
    return count


def calculate_scenic_score(x: int, y: int) -> int:
    height = grid[y][x]
    
    visible_l = count_until_and_including(lambda x_adj: grid[y][x_adj] < height, reversed(range(0, x)))
    visible_r = count_until_and_including(lambda x_adj: grid[y][x_adj] < height, range(x + 1, cols))
    
    visible_u = count_until_and_including(lambda y_adj: grid[y_adj][x] < height, reversed(range(0, y)))
    visible_d = count_until_and_including(lambda y_adj: grid[y_adj][x] < height, range(y + 1, rows))
    
    return visible_l * visible_r * visible_u * visible_d


visible_trees = sum(is_visible(x, y) for y in range(rows) for x in range(cols))
print(f"Visible trees: {visible_trees}")

highest_scenic_score = max(calculate_scenic_score(x, y) for y in range(rows) for x in range(cols))
print(f"Highest scenic score: {highest_scenic_score}")
