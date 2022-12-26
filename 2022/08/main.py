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
    
    height = grid[x][y]
    
    if all(grid[x_adj][y] < height for x_adj in range(0, x)):
        return True
    
    if all(grid[x_adj][y] < height for x_adj in range(x + 1, cols)):
        return True
    
    if all(grid[x][y_adj] < height for y_adj in range(0, y)):
        return True
    
    if all(grid[x][y_adj] < height for y_adj in range(y + 1, rows)):
        return True
    
    return False


visible_trees = sum(is_visible(x, y) for y in range(rows) for x in range(cols))
print(f"Visible trees: {visible_trees}")
