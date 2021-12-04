import numpy as np
from typing import List


def boards_from_file(f):
    ret = []
    curr = []
    for line in f:
        if len(line) < 2:
            if len(curr) > 0:
                ret.append(curr)
            curr = []
        else:
            curr.append([int(n) + 1 for n in line.split(" ")])

    return ret


def a_row_is_complete(b: List[List[int]]) -> bool:
    for r in b:
        if sum(r) == 0:
            return True

    return False


def is_complete(b):
    return a_row_is_complete(b) or a_row_is_complete(list(map(list, zip(*b))))


def board_score(b, n):
    total = 0
    for r in b:
        for el in r:
            if el != 0:
                total += (el - 1)
    return total * n


numbers = [int(n) + 1 for n in np.genfromtxt("./numbers", delimiter=",")]

with open("./input") as file:
    boards = boards_from_file(file)

for num in numbers:
    for board in boards:
        for i in range(len(board)):
            board[i] = [0 if n == num else n for n in board[i]]

        if is_complete(board):
            print("score", board_score(board, num - 1))

    # remove any boards that have been completed
    boards = [b for b in boards if not is_complete(b)]
