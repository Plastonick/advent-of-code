import sys


def get_boundary(point: tuple[int, int], image: list[list[int]], unknown: int) -> int:
    output = [[unknown for _ in range(3)] for _ in range(3)]

    for i in range(point[0] - 1, point[0] + 2):
        if i < 0 or i >= len(image):
            continue

        for j in range(point[1] - 1, point[1] + 2):
            if j < 0 or j >= len(image[i]):
                continue

            output[i - point[0] + 1][j - point[1] + 1] = image[i][j]

    return output


def get_boundary_value(point: tuple[int, int], image: list[list[int]], unknown: int) -> int:
    boundary = get_boundary(point, image, unknown)
    bin_str = "".join(["".join([str(c) for c in line]) for line in boundary])
    return int(bin_str, 2)


def iterate_image(image: list[list[int]], enhancement: list[int], unknown: int) -> tuple[list[list[int]], list[list[int]]]:
    buffer = 3

    output = [[0 for _ in range(len(image) + (2 * buffer))] for _ in range(len(image[0]) + (2 * buffer))]

    for i in range(len(output)):
        for j in range(len(output[i])):
            value = get_boundary_value((i - buffer, j - buffer), image, unknown)
            map_value = enhancement[value]

            output[i][j] = map_value

    de_buffered = output[buffer-1:-buffer+1]
    de_buffered = [line[buffer-1:-buffer+1] for line in de_buffered]

    return de_buffered, output[0][0]


def number_lit(image: list[list[int]]) -> int:
    return sum([sum(line) for line in image])


def print_image(image: list[list[int]]) -> None:
    print("\n".join(["".join(['#' if c == 1 else '.' for c in line]) for line in image]))
    print()


with open(sys.argv[1]) as f:
    lines = f.read().strip().splitlines()

_enhancement = [0 if char == '.' else 1 for char in lines[0]]
_image = [[0 if char == '.' else 1 for char in line] for line in lines[2:]]
print_image(_image)
outside = 0
for _ in range(50):
    _image, outside = iterate_image(_image, _enhancement, outside)
    # print_image(_image)

print_image(_image)
print("part1", number_lit(_image))
