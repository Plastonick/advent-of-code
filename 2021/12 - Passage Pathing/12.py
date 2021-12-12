def all_small_nodes(graph: dict[str, list[str]]) -> list[str]:
    all_nodes = graph.keys()

    ret = []
    for node in all_nodes:
        if node in ["start", "end"]:
            continue

        if ord(node[0]) >= 97:
            ret.append(node)

    return ret


def number_occurrences_in_path(node: str, path: list[str]) -> int:
    filtered = [1 if n == node else 0 for n in path]

    return sum(filtered)


def branch_paths(curr: str, path: list[str], chosen_small: str) -> list[list[str]]:
    if curr == "end":
        return [path]

    paths = []
    for branch in network[curr]:
        chosen_occurrences = number_occurrences_in_path(chosen_small, path)

        # we allow our "chosen" small node to be chosen twice
        if branch == chosen_small:
            if chosen_occurrences >= 2:
                continue
        # else, if we've already visited it, and it's lower case, i.e. can only traverse once, continue
        elif branch in path and ord(branch[0]) >= 97:
            continue

        new_path = path.copy()
        new_path.append(branch)
        paths += branch_paths(branch, new_path, chosen_small)

    return paths


network = {}
with open('input') as f:
    for l in f.read().strip().split("\n"):
        left, right = l.split("-")
        if left not in network:
            network[left] = []
        if right not in network:
            network[right] = []

        # it's not a directed graph
        network[left].append(right)
        network[right].append(left)

all_paths = branch_paths("start", ["start"], "bleurgh")
print("part1", len(all_paths))

# represent the paths as strings this time
all_paths_with_multiple_small: set[str] = set()
for small in all_small_nodes(network):
    all_paths_for_small = branch_paths("start", ["start"], small)
    path_strings = set(["-".join(n) for n in all_paths_for_small])
    all_paths_with_multiple_small = all_paths_with_multiple_small.union(path_strings)

print("part2", len(all_paths_with_multiple_small))
