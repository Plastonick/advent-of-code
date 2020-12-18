
import Foundation

let stateString: String
let debug = false

if !debug {
    stateString = """
    #####...
    .#..##..
    ##.##.##
    ...####.
    #.#...##
    .##...#.
    .#.#.###
    #.#.#..#
    """
} else {
    stateString = """
    .#.
    ..#
    ###
    """
}

var activeCoordinates: [Coordinate] = []

for (x, row) in stateString.components(separatedBy: "\n").enumerated() {
    let rowStates = Array(row).map { String($0) }
    for (y, type) in rowStates.enumerated() {
        if type == "#" {
            activeCoordinates.append(Coordinate(x, y, 0, 0))
        }
    }
}

var system = System(activeCoordinates: Set(activeCoordinates))
print("There are \(system.activeCoordinates.count) active coords initially")

if debug {
    // for z in -2 ... 2 {
    //     print("z = \(z)")
    //     print(system.toString(depth: z))
    // }
}

for i in 1 ... 6 {
    system = system.iterate()

    print("There are \(system.activeCoordinates.count) active coords after cycle \(i)")

    if debug {
        // for z in -2 ... 2 {
        //     print("z = \(z)")
        //     print(system.toString(depth: z))
        // }
    }
}

struct System {
    let activeCoordinates: Set<Coordinate>

    func iterate() -> System {
        var newActiveCoordinates: [Coordinate] = []
        let range = buildRange()

        for coord in range {
            if shouldBeActive(coord) {
                newActiveCoordinates.append(coord)
            }
        }

        return System(activeCoordinates: Set(newActiveCoordinates))
    }

    // func toString(depth: Int) -> String {
    //     let origin = Coordinate(0, 0, depth)
    //     var range = Range(minD: origin, maxD: origin)
    //     for coord in activeCoordinates {
    //         range = range.addCoord(coord)
    //     }

    //     let min = range.minD
    //     let max = range.maxD

    //     var str = ""
    //     for x in min.x ... max.x {
    //         for y in min.y ... max.y {
    //             if isActive(Coordinate(x, y, depth)) {
    //                 str += "#"
    //             } else {
    //                 str += "."
    //             }
    //         }

    //         str += "\n"
    //     }

    //     return str
    // }

    func isActive(_ c: Coordinate) -> Bool {
        return activeCoordinates.contains(c)
    }

    func shouldBeActive(_ c: Coordinate) -> Bool {
        let exclusiveNeighbours = c.getNeighbourhood().filter { $0 != c }
        let numActiveNeighbours = exclusiveNeighbours.filter { self.isActive($0) }.count
        let isActive = self.isActive(c)

        if isActive {
            return numActiveNeighbours == 2 || numActiveNeighbours == 3
        } else {
            return numActiveNeighbours == 3
        }
    }

    func buildRange() -> [Coordinate] {
        var range: [Coordinate] = []

        for coord in activeCoordinates {
            range.append(contentsOf: coord.getNeighbourhood())
        }

        let uniqueRange = Array(Set(range))

        return uniqueRange
    }
}

struct Coordinate: Hashable {
    let x, y, z, w: Int

    init(_ x: Int, _ y: Int, _ z: Int, _ w: Int) {
        self.x = x
        self.y = y
        self.z = z
        self.w = w
    }

    func getNeighbourhood() -> [Coordinate] {
        var neighbours: [Coordinate] = []

        for xd in -1 ... 1 {
            for yd in -1 ... 1 {
                for zd in -1 ... 1 {
                    for wd in -1 ... 1 {
                        neighbours.append(Coordinate(x + xd, y + yd, z + zd, w + wd))
                    }
                }
            }
        }

        return neighbours
    }
}

// struct Range {
//     let minD, maxD: Coordinate

//     func addCoord(_ c: Coordinate) -> Range {
//         let minC = Coordinate(min(c.x, minD.x), min(c.y, minD.y), min(c.z, minD.z))
//         let maxC = Coordinate(max(c.x, maxD.x), max(c.y, maxD.y), max(c.z, maxD.z))

//         return Range(minD: minC, maxD: maxC)
//     }
// }
