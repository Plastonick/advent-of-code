import sys

sys.setrecursionlimit(100000000)


class State:
    def __init__(self, state: dict[tuple[int, int], str], cost: int):
        self.state = state
        self.cost = cost

    def move(self, a, b):
        if not self.move_is_valid(a, b):
            print("Attempted an invalid move!")
            exit(1)

        steps = a[1] + abs(a[0] - b[0]) + b[1]
        amphipod_type = self.state[a]
        move_cost = energy[amphipod_type] * steps

        new_state = self.state.copy()
        del new_state[a]
        new_state[b] = amphipod_type

        return State(new_state, self.cost + move_cost)

    def move_is_valid(self, a, b) -> bool:
        if a not in self.state:
            return False
        if b in self.state:
            return False
        if b[0] in [2, 4, 6, 8] and b[1] == 0:
            return False

        return True

    def is_complete(self):
        if len(self.state) != 8:
            print("Invalid state!")
            exit(1)

        prev = None
        for pos in self.state:
            amphipod_type = self.state[pos]
            if prev is not None and amphipod_type < prev:
                return False
            if pos[1] == 0:
                return False
            prev = amphipod_type

        return True

    def enumerate_legal_moves(self):
        moves: list[tuple[tuple[int, int]]] = []
        for pos in self.state:
            amphipod_type = self.state[pos]
            target_room = destinations[amphipod_type]

            # if it's in a hallway, it _must_ move to it's final room (as far inside as possible)
            if pos[1] == 0:
                # if the room isn't free to move to, this amphipod is stuck for now
                room_free = self.room_is_free_to_move(amphipod_type)
                if room_free is None:
                    continue
                # if we have a clear path to the room, let's consider going to it!
                elif self.hall_free_between(pos[0], target_room):
                    moves.append((pos, (target_room, room_free)))

                # if not, we're stuck until we have a clear path to the target room

            # it's in a room, we either need to move right into the room, or to an appropriate hallway spot
            else:
                # is it blocked? Ignore it and carry on.
                if pos[1] == 2 and (pos[0], 1) in self.state:
                    continue

                # is its room already completed? Ignore it and carry on.
                if self.room_is_complete(amphipod_type):
                    continue

                hall_min, hall_max = self.hall_free_min_max_around(pos[0])

                # enumerate all the good hall positions
                for i in range(hall_min, hall_max):
                    # can't stop outside a room
                    if i in [2, 4, 6, 8]:
                        continue

                    # this is a legal move into the hallway, add it to our list
                    moves.append((pos, (i, 0)))

                # we also want to consider moving from a room directly into the target room
                room_free = self.room_is_free_to_move(amphipod_type)
                if room_free is not None:
                    moves.append((pos, (target_room, room_free)))

        return moves

    def hall_free_min_max_around(self, a):
        left_positions: list[int] = []
        right_positions: list[int] = []
        for pos in self.state:
            if pos[1] == 0:
                if pos[0] < a:
                    left_positions.append(pos[0])
                else:
                    right_positions.append(pos[0])

        left_positions.sort()
        right_positions.sort()

        if len(left_positions) == 0:
            min = 0
        else:
            min = left_positions[-1] + 1

        if len(right_positions) == 0:
            max = 10
        else:
            max = right_positions[0] - 1

        return min, max

    def hall_free_between(self, a: int, b: int) -> bool:
        for i in range(min(a, b) + 1, max(a, b)):
            if (i, 0) in self.state:
                return False

        return True

    def room_is_complete(self, amphipod_type):
        amphipod_room = destinations[amphipod_type]

        for i in [1, 2]:
            if (amphipod_room, i) not in self.state:
                return False
            if self.state[(amphipod_room, i)] != amphipod_type:
                return False

        return True

    def room_is_free_to_move(self, amphipod_type):
        destination_room = destinations[amphipod_type]

        # if the entry to the room is blocked, then can't move!
        if (destination_room, 1) in self.state:
            return None

        # the back of the room is free, this room is good to go!
        if (destination_room, 2) not in self.state:
            return 2

        # the back of the room is taken, but it's the right type, we can move to the entrance!
        if self.state[(destination_room, 2)] == amphipod_type:
            return 1

        return None


def iterate_moves(state: State):
    global best_cost

    if state.is_complete():
        best_cost = state.cost
        print("Found new best cost", best_cost)
        exit()
        return

    print("Try new", best_cost)

    potential_moves = state.enumerate_legal_moves()
    for move_from, move_to in potential_moves:
        new_state = state.move(move_from, move_to)

        if new_state.cost <= best_cost:
            iterate_moves(new_state)


energy = {
    "A": 1,
    "B": 10,
    "C": 100,
    "D": 1000
}

destinations = {
    "A": 2,
    "B": 4,
    "C": 6,
    "D": 8,
}

if len(sys.argv) >= 2 and sys.argv[1] == 'example':
    starting_state = State(
        state={
            (2, 1): "B",
            (2, 2): "A",
            (4, 1): "C",
            (4, 2): "D",
            (6, 1): "B",
            (6, 2): "C",
            (8, 1): "D",
            (8, 2): "A",
        }, cost=0
    )
    print("expected", 12521)
else:
    starting_state = State(
        state={
            (2, 1): "B",
            (2, 2): "B",
            (4, 1): "C",
            (4, 2): "C",
            (6, 1): "A",
            (6, 2): "D",
            (8, 1): "D",
            (8, 2): "A",
        }, cost=0
    )

best_cost = 13521

iterate_moves(state=starting_state)

print("Finished, best cost:", best_cost)
