east = set()
south = set()

field = [*open(0)]

height = len(field)
width = len(field[0].strip())
for i in range(height):
    row = field[i]
    for j in range(width):
        if row[j] == '>':
            east.add((i, j))
        elif row[j] == 'v':
            south.add((i, j))

moves = 1
while True:
    new_east = set()
    has_moved = False
    for cucumber in east:
        move_to = (cucumber[0], (cucumber[1] + 1) % width)
        if move_to not in east and move_to not in south:
            has_moved = True
            new_east.add(move_to)
        else:
            new_east.add(cucumber)

    east = new_east
    new_south = set()

    for cucumber in south:
        move_to = ((cucumber[0] + 1) % height, cucumber[1])
        if move_to not in east and move_to not in south:
            has_moved = True
            new_south.add(move_to)
        else:
            new_south.add(cucumber)

    if not has_moved:
        break

    moves += 1
    south = new_south

print("total moves", moves)
