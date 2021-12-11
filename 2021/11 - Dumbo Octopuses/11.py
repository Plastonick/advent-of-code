import numpy as np


def flash_cell(cell: tuple[int, int], grid: np.ndarray) -> np.ndarray:
    for di in [-1, 0, 1]:
        for dj in [-1, 0, 1]:
            i = cell[0] + di
            j = cell[1] + dj

            if i >= grid.shape[0] or j >= grid.shape[1] or i < 0 or j < 0:
                continue

            grid[i][j] += 1

    return grid


def process_flashes(grid: np.ndarray, flashed: np.ndarray) -> np.ndarray:
    global total_flashes

    overloaded = grid > 9
    processed_flash = False

    for i in range(grid.shape[0]):
        for j in range(grid.shape[1]):
            if flashed[i][j]:
                continue

            if overloaded[i][j]:
                grid = flash_cell((i, j), grid)
                flashed[i][j] = True
                processed_flash = True
                total_flashes += 1

    if processed_flash:
        return process_flashes(grid, flashed)
    else:
        # reset any that flashed to 0
        grid[flashed] = 0
        return grid


def iterate(grid: np.ndarray) -> np.ndarray:
    grid = grid + 1

    # initialise an array of all false
    flashed = np.zeros(grid.shape, dtype=bool)

    return process_flashes(grid, flashed)


g = np.asarray([[int(n) for n in string] for string in np.genfromtxt('input', dtype=str)])

total_flashes = 0
i = 0
while True:
    g = iterate(grid=g)
    if g.sum() == 0:
        # part 2 happens to be after 100 moves ¯\_(ツ)_/¯
        print("part2", i + 1)
        break

    if i == 100:
        print("part1", total_flashes)

    i += 1
