import numpy as np
import matplotlib.pyplot as plt

def cost_to_move_linearly(crabs, position) -> int:
    return abs(crabs - position).sum()


def cost_to_move_non_linearly(crabs, position) -> int:
    diff = abs(crabs - position)

    # triangular number
    return (diff * (diff + 1) / 2).sum()


_crabs = np.genfromtxt('input', delimiter=",", dtype=int)

best = cost_to_move_non_linearly(_crabs, min(_crabs))
y = []

for i in range(min(_crabs), max(_crabs)):
    cost = cost_to_move_non_linearly(_crabs, i)
    y.append(cost)
    best = min(best, cost)

    i += 1

plt.plot(y)
plt.title("Fuel cost against position")
plt.ylabel("Costs")
plt.show()

print(best)
