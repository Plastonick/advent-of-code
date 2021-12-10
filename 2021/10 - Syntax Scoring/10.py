def char_score(char):
    if char == ')':
        return 3
    elif char == ']':
        return 57
    elif char == '}':
        return 1197
    elif char == '>':
        return 25137
    else:
        return 0


def running_score(running: list):
    running.reverse()

    total = 0

    for char in running:
        total *= 5

        if char == '(':
            total += 1
        elif char == '[':
            total += 2
        elif char == '{':
            total += 3
        elif char == '<':
            total += 4

    return total


def first_incorrect(line):
    opening_closing_map = {
        "(": ")",
        "[": "]",
        "{": "}",
        "<": ">"
    }

    running = []
    for char in line:
        if char in opening_closing_map.values():
            # if there are no opening characters, then the closing character is bad!
            if len(running) == 0:
                return char
            # if the last opening character isn't for this closing character, also bad!
            elif opening_closing_map[running[-1]] != char:
                return char
            # last opening character matches closing character, remove from the running
            else:
                running.pop(-1)
        else:
            running.append(char)

    return running


part1 = 0
part2 = []
with open("input") as f:
    for line in f:
        first = first_incorrect(line.strip())

        if not isinstance(first, list):
            part1 += char_score(first)
        #     we have our previous runnings
        else:
            part2.append(running_score(first))

part2.sort()

print("part1", part1)
print("part2", part2[int(len(part2) / 2)])
