import numpy as np
from scipy.sparse import csr_matrix
from scipy.sparse.csgraph import dijkstra
from scipy.sparse.csgraph import shortest_path

from pathfinding.core.diagonal_movement import DiagonalMovement
from pathfinding.core.grid import Grid
from pathfinding.finder.a_star import AStarFinder


def iterate_template(template, times):
    return (template + times - 1) % 9 + 1


def expand_path(template, times=4):
    ret = template.copy()

    for i in range(times):
        ret = np.concatenate((ret, iterate_template(template, i + 1)))

    column = ret.copy()

    for j in range(times):
        ret = np.concatenate((ret, iterate_template(column, j + 1)), axis=1)

    return ret


def to_dijkstra_matrix(path):
    dim = len(path)
    ret = np.zeros((dim ** 2, dim ** 2))

    for i in range(dim):
        for j in range(dim):
            # build possible targets
            t1 = (i + 1, j)
            t2 = (i - 1, j)
            t3 = (i, j + 1)
            t4 = (i, j - 1)

            for t in [t1, t2, t3, t4]:
                if t[0] < 0 or t[0] >= dim or t[1] < 0 or t[1] >= dim:
                    continue

                source = (dim * i) + j
                target = (dim * t[0]) + t[1]

                # cost of going from source to target = path[t[0]][t[1]]
                ret[source][target] = path[t[0]][t[1]]

    return ret


path = []

with open('input') as f:
    for line in f.read().strip().split("\n"):
        path.append([int(n) for n in line])

path = np.asarray(path)

# for part 2
path = expand_path(path)

path_dim = len(path)

if True:
    grid = Grid(matrix=path)
    start = grid.node(0, 0)
    end = grid.node(path_dim - 1, path_dim - 1)

    finder = AStarFinder(diagonal_movement=DiagonalMovement.never, weight=0)
    short_path, runs = finder.find_path(start, end, grid)

    # print('operations:', runs, 'path length:', len(short_path))
    # print(grid.grid_str(path=short_path, start=start, end=end))

    total = -path[0][0]
    for p in short_path:
        cost = path[p[1]][p[0]]
        total += cost

    print(total)

    a = 1
else:
    mat = to_dijkstra_matrix(path)

    graph = csr_matrix(mat)
    dist_matrix = shortest_path(csgraph=graph, directed=True, indices=0)

    print("part1", dist_matrix[path_dim ** 2 - 1])
