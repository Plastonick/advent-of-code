import re
import numpy as np


def create_rotation_matrices():
    return [
        [
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, 1],
        ],
        [
            [1, 0, 0],
            [0, 0, -1],
            [0, 1, 0],
        ],
        [
            [1, 0, 0],
            [0, -1, 0],
            [0, 0, -1],
        ],
        [
            [1, 0, 0],
            [0, 0, 1],
            [0, -1, 0],
        ],
        [
            [0, -1, 0],
            [1, 0, 0],
            [0, 0, 1],
        ],
        [
            [0, 0, 1],
            [1, 0, 0],
            [0, 1, 0],
        ],
        [
            [0, 1, 0],
            [1, 0, 0],
            [0, 0, -1],
        ],
        [
            [0, 0, -1],
            [1, 0, 0],
            [0, -1, 0],
        ],
        [
            [-1, 0, 0],
            [0, -1, 0],
            [0, 0, 1],
        ],
        [
            [-1, 0, 0],
            [0, 0, -1],
            [0, -1, 0],
        ],
        [
            [-1, 0, 0],
            [0, 1, 0],
            [0, 0, -1],
        ],
        [
            [-1, 0, 0],
            [0, 0, 1],
            [0, 1, 0],
        ],
        [
            [0, 1, 0],
            [-1, 0, 0],
            [0, 0, 1],
        ],
        [
            [0, 0, 1],
            [-1, 0, 0],
            [0, -1, 0],
        ],
        [
            [0, -1, 0],
            [-1, 0, 0],
            [0, 0, -1],
        ],
        [
            [0, 0, -1],
            [-1, 0, 0],
            [0, 1, 0],
        ],
        [
            [0, 0, -1],
            [0, 1, 0],
            [1, 0, 0],
        ],
        [
            [0, 1, 0],
            [0, 0, 1],
            [1, 0, 0],
        ],
        [
            [0, 0, 1],
            [0, -1, 0],
            [1, 0, 0],
        ],
        [
            [0, -1, 0],
            [0, 0, -1],
            [1, 0, 0],
        ],
        [
            [0, 0, -1],
            [0, -1, 0],
            [-1, 0, 0],
        ],
        [
            [0, -1, 0],
            [0, 0, 1],
            [-1, 0, 0],
        ],
        [
            [0, 0, 1],
            [0, 1, 0],
            [-1, 0, 0],
        ],
        [
            [0, 1, 0],
            [0, 0, -1],
            [-1, 0, 0],
        ],
    ]


def create_rotational_key(relative_beacons: np.ndarray) -> set[tuple[int, int, int]]:
    vectors = set()

    for rotation in _rotations:
        for a in relative_beacons:
            for b in relative_beacons:
                if np.array_equal(a, b):
                    continue

                key_arr = np.matmul(rotation, [a[i] - b[i] for i in range(3)])
                vectors.add(tuple(key_arr))

    return vectors


class Scanner:
    keys = None

    def __init__(self, beacons: np.ndarray):
        self.beacons = beacons

    def get_keys(self):
        if self.keys is None:
            self.keys = create_rotational_key(self.beacons)

        return self.keys


    def rotate(self, n: int):
        """
        rotates this scanner, n must be an integer between 1 and 24 inclusive,
        to represent all rotational symmetries of a cube

        :type n: int
        """

        rotation = _rotations[n]

        rotated_beacons = [np.matmul(rotation, b) for b in self.beacons]

        return Scanner(beacons=rotated_beacons)

    def translate(self, vector):
        translated = np.asarray([beacon + vector for beacon in self.beacons])

        return Scanner(beacons=translated)


def is_match(a: np.ndarray, b: np.ndarray) -> bool:
    a_keys = [tuple([int(n) for n in beacon]) for beacon in a]
    b_keys = [tuple([int(n) for n in beacon]) for beacon in b]

    if set(a_keys).intersection(set(b_keys)) != set(a_keys).intersection(b_keys):
        print("bad!")
        exit()

    return len(set(a_keys).intersection(b_keys)) >= 12


def attempt_match(a: Scanner, b: Scanner):
    for i in range(24):
        rotated = b.rotate(i)

        # now we must find the translation vector on b that maximises the number of shared points between a and b
        # consider ever possible translation between beacons in a and b
        for p1 in a.beacons:
            for p2 in rotated.beacons:
                pd = p1 - p2

                translated_b = np.asarray([bb + pd for bb in rotated.beacons])
                if is_match(a.beacons, translated_b):
                    return rotated.translate(pd)

    return None


def find_match(fixed_scanners: list[Scanner], unknown_scanners: list[Scanner]):
    for i in range(len(unknown_scanners)):
        unknown = unknown_scanners[i]
        for fixed in fixed_scanners:
            key_collisions = len(unknown.get_keys().intersection(fixed.get_keys()))

            if key_collisions < 24 * 11 * 11:
                continue

            match = attempt_match(fixed, unknown)

            if match is not None:
                return match, i

    return None, None


def retrieve_unique_beacons(scanners: list[Scanner]) -> set[tuple[int, int, int]]:
    known_beacons = set()
    for scanner in scanners:
        known_beacons = known_beacons.union(set([tuple(b) for b in scanner.beacons]))

    return known_beacons


def fix_scanners(scanners: list[Scanner]):
    # assume scanner 0 is fixed
    fixed_scanners = scanners[:1]
    unknown_scanners = scanners[1:]

    while len(unknown_scanners) > 0:
        match, i = find_match(fixed_scanners, unknown_scanners)

        if match is None:
            print("Did not find a match! This probably should not happen...")
            break
        else:
            print("Found a match!")
            fixed_scanners.append(match)
            del unknown_scanners[i]

    return fixed_scanners


_rotations = create_rotation_matrices()
_scanners = []

with open("input") as f:
    for scanner_str in re.sub(r"[^\n]*scan[^\n]*\n", "\n", f.read()).strip().split("\n\n"):
        scan_lines = np.asarray([line.split(",") for line in scanner_str.strip().split("\n")], dtype=int)
        _scanners.append(Scanner(scan_lines))

_fixed_scanners = fix_scanners(_scanners)
_fixed_beacons = retrieve_unique_beacons(_fixed_scanners)

print(len(_fixed_beacons))
