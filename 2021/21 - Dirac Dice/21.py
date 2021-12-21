import sys


class Die:
    def __init__(self):
        self.value = 1

    def roll(self, times: int):
        total = 0
        for _ in range(times):
            total += self.value
            self.value = (self.value % 100) + 1

        return total


class Player:
    def __init__(self, score: int, position: int):
        self.score = score

        # 0 index to make module arithmetic easier
        self.position = position - 1

    def get_position(self):
        return self.position + 1

    def move(self, pieces):
        self.position = (self.position + pieces) % 10
        self.score += self.position + 1


def build_dirac_frequency(choices: int, n_rolls: int, roll_frequency: dict[int, int]) -> dict[int, int]:
    if n_rolls == 0:
        return roll_frequency

    new_roll_freq = {}
    for c in range(1, choices + 1):
        for r in roll_frequency:
            if r + c not in new_roll_freq:
                new_roll_freq[r + c] = 0

            new_roll_freq[r + c] = roll_frequency[r] + 1

    return build_dirac_frequency(choices, n_rolls - 1, new_roll_freq)


def dirac_turn(players: tuple[Player, Player], turn: int) -> tuple[int, int]:
    max_score = 21

    if players[0].score >= max_score:
        return 1, 0
    elif players[1].score >= max_score:
        return 0, 1

    wins = [0, 0]
    p1 = players[0]
    p2 = players[1]

    # how many ways there are of rolling each dice
    freq = {
        3: 1,
        4: 3,
        5: 6,
        6: 7,
        7: 6,
        8: 3,
        9: 1
    }

    for r in freq:
        # construct new players, find out how many time they win
        new_p1 = Player(score=p1.score, position=p1.get_position())
        new_p2 = Player(score=p2.score, position=p2.get_position())

        if turn == 0:
            new_p1.move(r)
        else:
            new_p2.move(r)

        p1wins, p2wins = dirac_turn((new_p1, new_p2), (turn + 1) % 2)
        wins[0] += p1wins * freq[r]
        wins[1] += p2wins * freq[r]

    return wins[0], wins[1]


if len(sys.argv) > 1 and sys.argv[1] == "test":
    player_1_starting = 4
    player_2_starting = 8
else:
    player_1_starting = 3
    player_2_starting = 10

p1 = Player(score=0, position=player_1_starting)
p2 = Player(score=0, position=player_2_starting)
die = Die()

_players = [p1, p2]

_turn = 0
number_of_rolls = 0

while _players[0].score < 1000 and _players[1].score < 1000:
    roll = die.roll(times=3)
    _players[_turn].move(roll)

    _turn = (_turn + 1) % 2
    number_of_rolls += 3

print("part1", _players[_turn].score * number_of_rolls)

print(build_dirac_frequency(3, 3, {0: 0}))

print(
    dirac_turn(
        [
            Player(score=0, position=player_1_starting),
            Player(score=0, position=player_2_starting)
        ],
        0
    )
)
