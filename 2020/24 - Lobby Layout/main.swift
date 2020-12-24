
import Foundation

let data = try! Data(contentsOf: URL(fileURLWithPath: "input"))
let string = String(decoding: data, as: UTF8.self)

let directions = string.components(separatedBy: "\n")
var flippedTiles: [Vector: Int] = [:]

for direction in directions {
    if direction == "" {
        continue
    }

    let components = Array(direction)

    var i = 0
    var totalVertical = 0
    var totalHorizontal = 0
    while i < components.count {
        let char = components[i]
        let vertical: Int
        let horizontal: Int

        if components[i] == "n" || components[i] == "s" {
            let nextChar = components[i + 1]
            vertical = char == "n" ? 1 : -1
            horizontal = nextChar == "e" ? 1 : -1
            i += 1
        } else {
            vertical = 0
            horizontal = char == "e" ? 2 : -2
        }

        totalVertical += vertical
        totalHorizontal += horizontal
        i += 1
    }

    let movement = Vector(x: totalHorizontal, y: totalVertical)

    if flippedTiles[movement] == nil {
        flippedTiles[movement] = 0
    }

    flippedTiles[movement]! += 1
}

var blackTiles: [Vector] = []

for (vector, flips) in flippedTiles {
    if flips % 2 == 1 {
        blackTiles.append(vector)
    }
}

print("There are \(flippedTiles.values.filter { $0 % 2 == 1 }.count) black tiles initially")

var floor = Floor(blackTiles: Set(blackTiles))
let nDays = 100

for _ in 1 ... nDays {
    floor = floor.iterate()
}

print("After \(nDays) days there are \(floor.blackTiles.count) black tiles")

struct Floor {
    let blackTiles: Set<Vector>

    func iterate() -> Floor {
        var iteratedBlackTiles: [Vector] = []
        let problemSpace = blackTiles.reduce([]) { $0 + $1.getNeighbourhood() }

        for vector in problemSpace {
            let isBlack = blackTiles.contains(vector)
            let nBlackNeighours = vector.getNeighbours().filter { blackTiles.contains($0) }.count

            if isBlack {
                // Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
                if nBlackNeighours == 1 || nBlackNeighours == 2 {
                    iteratedBlackTiles.append(vector)
                }
            } else {
                // Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
                if nBlackNeighours == 2 {
                    iteratedBlackTiles.append(vector)
                }
            }
        }

        return Floor(blackTiles: Set(iteratedBlackTiles))
    }
}

struct Vector: Hashable {
    let x, y: Int

    func getNeighbours() -> [Vector] {
        return [
            Vector(x: x + 1, y: y + 1), // ne
            Vector(x: x + 2, y: y), // e
            Vector(x: x + 1, y: y - 1), // se
            Vector(x: x - 1, y: y - 1), // sw
            Vector(x: x - 2, y: y), // w
            Vector(x: x - 1, y: y + 1), // nw
        ]
    }

    func getNeighbourhood() -> [Vector] {
        // neighbourhood includes itself
        return [Vector(x: x, y: y)] + getNeighbours()
    }
}
