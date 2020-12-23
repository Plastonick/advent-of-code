
import Foundation

let data = try! Data(contentsOf: URL(fileURLWithPath: "input"))
let string = String(decoding: data, as: UTF8.self)
let tileSize = 10
var tiles: [Tile] = []

for tileString in string.components(separatedBy: "\n\n") {
    let tile = buildTileFromString(tileString)

    tiles.append(tile)
}

var allLooseKeys: [Int] = []
var cornerTiles: [Tile] = []
var edgeTiles: [Tile] = []
var centreTiles: [Tile] = []

for tile in tiles {
    allLooseKeys.append(contentsOf: tile.keys)

    var outsideEdges = 0
    for key in tile.keys {
        let matches = numMatches(key: key, tiles: tiles)
        if matches == 1 {
            // it only matches itself, it must be on the edge!
            outsideEdges += 1
        }
    }

    if outsideEdges == 2 {
        cornerTiles.append(tile)
    } else if outsideEdges == 1 {
        edgeTiles.append(tile)
    } else {
        centreTiles.append(tile)
    }
}

print("Corner Tiles \(cornerTiles.map { $0.id })")
print("Edge Tiles \(edgeTiles.map { $0.id })")

var randomTile = cornerTiles.first!

// print(randomTile.toString())
// randomTile = randomTile.rotated(3)
// print(randomTile.toString())

if Set(allLooseKeys).count == 312 {
    print("Loose keys are sufficient to determine a match! Woohoo! This also implies that all potential corner and edge tiles are definitely corner and edge tiles!")
}

if cornerTiles.count == 4 {
    print("Found the four corner tiles! The product of their IDs is \(cornerTiles.reduce(1) { $0 * $1.id })")
} else {
    print("Naive solution didn't work, sadly")
}

let naiveGrid = buildNaiveTileGrid(tiles: tiles)
let sortedGrid = naiveGrid.sortMatching()

let monster = """
                  # 
#    ##    ##    ###
 #  #  #  #  #  #   
"""
let monsterRows = monster.components(separatedBy: "\n")
var monsterMask: [(Int, Int)] = []
var monsterDim = (0, 0)

for (i, monsterRow) in monsterRows.enumerated() {
    let rowArray = Array(monsterRow).map { String($0) }
    monsterDim.0 = max(i + 1, monsterDim.0)

    for (j, el) in rowArray.enumerated() {
        monsterDim.1 = max(j + 1, monsterDim.1)

        if el == "#" {
            monsterMask.append((i, j))
        }
    }
}

print(monsterMask)
print(monsterDim)

print(sortedGrid.toString())

var gridArray = sortedGrid.toArray()
var seaMonsterTiles = gridArray
for (i, row) in seaMonsterTiles.enumerated() {
    for (j, _) in row.enumerated() {
        seaMonsterTiles[i][j] = " "
    }
}

var flippedGrid = gridArray.map { Array($0.reversed()) }

let gridDim = gridArray.count
var nMatches = 0

for _ in 0 ... 1 {
    gridArray = flipArray(gridArray)
    seaMonsterTiles = flipArray(seaMonsterTiles)

    for _ in 0 ..< 4 {
        gridArray = rotateSqArray(gridArray, times: 1)
        seaMonsterTiles = rotateSqArray(seaMonsterTiles, times: 1)
        for i in 0 ..< (gridDim - monsterDim.1) {
            for j in 0 ..< (gridDim - monsterDim.0) {
                var matches = true
                for point in monsterMask {
                    if gridArray[i + point.1][j + point.0] != "#" {
                        matches = false
                        break
                    }
                }

                if matches {
                    nMatches += 1

                    for point in monsterMask {
                        seaMonsterTiles[i + point.1][j + point.0] = "#"
                    }
                }
            }
        }
    }
}

var numberChoppyTiles = 0
for row in gridArray {
    numberChoppyTiles += row.filter { $0 == "#" }.count
}

var numberSeaMonsterTiles = 0
for row in seaMonsterTiles {
    numberSeaMonsterTiles += row.filter { $0 == "#" }.count
}

print("The answer to part two is... drum roll please... \(numberChoppyTiles - numberSeaMonsterTiles)")

print(nMatches)

var monsterMaskString = ""

for row: [String] in seaMonsterTiles {
    monsterMaskString += row.joined()
    monsterMaskString += "\n"
}

print(monsterMaskString)

func buildNaiveTileGrid(tiles: [Tile]) -> Grid {
    // since we've determined we have the minimum number of matching edges, if a tile edge's key matches another,
    // then they necessarily must be adjacent. Therefore we should greedily place tiles in a square first, and
    // then determine the correct orientation afterwards
    var grid = Grid(deployed: [:], remaining: tiles)

    for i in 0 ... 11 {
        for j in 0 ... 11 {
            let position = Pos(i, j)
            grid = grid.hydrateTile(at: position)
        }
    }

    return grid
}

func numMatches(key: Int, tiles: [Tile]) -> Int {
    var matches = 0

    for tile in tiles {
        if tile.keys.contains(key) {
            matches += 1
        }
    }

    return matches
}

func buildTileFromString(_ s: String) -> Tile {
    let rows = s.components(separatedBy: "\n")
    let id = Int(rows[0].replacingOccurrences(of: "Tile ", with: "").replacingOccurrences(of: ":", with: ""))!

    var image: [[String]] = []
    for i in 1 ..< rows.count {
        let row = rows[i]
        if row.count == 0 {
            continue
        }

        image.append(Array(row).map {
            let char = String($0)

            if char == "." {
                return "0"
            } else {
                return "1"
            }
        })
    }

    return Tile(id: id, image: image)
}

func rotateSqArray<T>(_ arr: [[T]], times: Int) -> [[T]] {
    if times == 0 {
        return arr
    }

    var newArr = arr
    for (i, row) in arr.enumerated() {
        for (j, el) in row.enumerated() {
            // represents a 90° clockwise rotation
            let index = row.count - i - 1
            newArr[j][index] = el
        }
    }

    return rotateSqArray(newArr, times: times - 1)
}

func flipArray<T>(_ arr: [[T]]) -> [[T]] {
    return arr.reversed()
}

struct Grid {
    let deployed: [Pos: Tile]
    let remaining: [Tile]

    public func hydrateTile(at: Pos) -> Grid {
        let adjacencies = findAdjacentTiles(to: at)
        var possibleTiles = findPossibleTiles(adjacencies: adjacencies)

        // don't try and put non-outter tiles on outter positions!
        if isEdgePosition(at) {
            possibleTiles = possibleTiles.filter { isEdgeTile($0) }
        } else if isCornerPosition(at) {
            possibleTiles = possibleTiles.filter { isCornerTile($0) }
        }

        return with(tile: possibleTiles.first!, at: at)
    }

    public func sortMatching() -> Grid {
        var fixedTiles: Set<Pos> = []
        var newTiles = deployed

        // loop over the vertical edges and rotate/flip the tiles until they match
        for i in 0 ..< 12 {
            for j in 0 ..< 11 {
                print("-------BREAK-------")
                let leftPos = Pos(i, j)
                let rightPos = Pos(i, j + 1)

                print(leftPos)
                print(rightPos)

                guard var leftTile = newTiles[leftPos] else {
                    print("Damn left")
                    exit(1)
                }

                guard var rightTile = newTiles[rightPos] else {
                    print("Damn right")
                    exit(1)
                }

                print(leftTile.preciseKeys)
                print(rightTile.preciseKeys)
                print("here")

                // are they in the same plane? If they are, one needs to be flipped!
                if leftTile.isSamePlaneAs(tile: rightTile) {
                    print("Same plane!")

                    // flip one of them
                    if !fixedTiles.contains(leftPos) {
                        print("Flipping left")
                        leftTile = leftTile.flipped(around: .vertical)
                    } else if !fixedTiles.contains(rightPos) {
                        print("Flipping right")
                        rightTile = rightTile.flipped(around: .vertical)
                    } else {
                        print("Uh oh! We've fixed both but they're in a different plane!")
                        exit(1)
                    }
                }
                print("here2")

                var keyMatch: (Int, Int)?
                outerLoop: for x in 0 ..< 4 {
                    for y in 0 ..< 4 {
                        if leftTile.keys[x] == rightTile.keys[y] {
                            keyMatch = (x, y)
                            break outerLoop
                        }
                    }
                }

                if keyMatch == nil {
                    print(leftPos)
                    print("Couldn't match them? Wut?")
                    exit(1)
                }

                print(leftTile.preciseKeys)
                print(rightTile.preciseKeys)

                let x = keyMatch!.0
                let y = keyMatch!.1

                // we need to rotate the left one until the matched key is on the east
                // and rotate the right one until the matched key is on the west

                print(leftPos)
                if fixedTiles.contains(leftPos), x != 1 {
                    print(fixedTiles)
                    print("Uh oh! We've fixed left but it needs rotating!")
                    exit(1)
                } else if fixedTiles.contains(rightPos), y != 3 {
                    print("Uh oh! We've fixed right but it needs rotating!")
                    exit(1)
                }
                print("here4")

                if !fixedTiles.contains(leftPos), x != 1 {
                    let times = (5 - x) % 4
                    print("rotating left \(times) time(s) due to \(keyMatch!)")
                    leftTile = leftTile.rotated(times)
                }

                print("here5")

                if !fixedTiles.contains(rightPos), y != 3 {
                    let times = (7 - y) % 4
                    print("rotating right \(times) time(s) due to \(keyMatch!)")
                    rightTile = rightTile.rotated(times)
                }

                print("here6")
                // insert the tiles!
                newTiles[leftPos] = leftTile
                newTiles[rightPos] = rightTile

                // we're happy with these tiles, don't let them change position again
                fixedTiles.insert(leftPos)
                fixedTiles.insert(rightPos)
            }
        }

        // now, do we need to flip the rows?

        for i in 0 ..< 11 {
            let pos = Pos(i, 0)
            let posUnder = Pos(i + 1, 0)

            guard let tile = newTiles[pos] else {
                print("???")
                exit(1)
            }

            guard let underTile = newTiles[posUnder] else {
                print("???")
                exit(1)
            }

            if tile.isSamePlaneAs(tile: underTile) {
                // they need to be alternate planes to connect
                // flip the entire underneath row, the higher row is fixed

                for j in 0 ..< 12 {
                    let flipPos = Pos(i + 1, j)
                    if let tile = newTiles[flipPos] {
                        newTiles[flipPos] = tile.flipped(around: .horizontal)
                    }
                }
            }
        }

        return Grid(deployed: newTiles, remaining: [])
    }

    public func toArray() -> [[String]] {
        let string = toString()
        let rows = string.components(separatedBy: "\n")
        var newRows: [[String]] = []
        for row in rows {
            if row == "" {
                break
            }

            newRows.append(Array(row).map { String($0) })
        }

        return newRows
    }

    public func toString() -> String {
        var string = ""

        for i in 0 ... 11 {
            var tiles: [Tile] = []

            for j in 0 ... 11 {
                guard let tile = deployed[Pos(i, j)] else {
                    print("Crappity")
                    exit(1)
                }

                tiles.append(tile)
            }

            string += mergeTileImagesInARow(tiles: tiles)
        }

        return string.replacingOccurrences(of: "1", with: "#").replacingOccurrences(of: "0", with: ".")
    }

    private func with(tile: Tile, at: Pos) -> Grid {
        let matchingTiles = deployed.values.filter { $0.id == tile.id }
        guard matchingTiles.first == nil else {
            print("Oh no! Adding an existing tile!")
            exit(1)
        }

        var newDeployed = deployed
        let newRemaining = remaining.filter { $0.id != tile.id }
        newDeployed[at] = tile

        return Grid(deployed: newDeployed, remaining: newRemaining)
    }

    private func mergeTileImagesInARow(tiles: [Tile]) -> String {
        var ret = ""

        for i in 1 ..< 9 {
            var row: [String] = []
            for tile in tiles {
                var rowChars = tile.image[i]
                rowChars.removeFirst()
                rowChars.removeLast()
                row.append(contentsOf: rowChars)
                // row.append("  ")
            }
            ret += row.joined() + "\n"
        }

        return ret
    }

    private func isEdgePosition(_ pos: Pos) -> Bool {
        return calculateEdgeMetric(pos) == 1
    }

    private func isCornerPosition(_ pos: Pos) -> Bool {
        return calculateEdgeMetric(pos) == 2
    }

    private func calculateEdgeMetric(_ pos: Pos) -> Int {
        var matchedCriteria = 0

        if pos.i == 0 || pos.i == 11 {
            matchedCriteria += 1
        }

        if pos.j == 0 || pos.j == 11 {
            matchedCriteria += 1
        }

        return matchedCriteria
    }

    private func isEdgeTile(_ tile: Tile) -> Bool {
        return getNumMatches(tile) == 4
    }

    private func isCornerTile(_ tile: Tile) -> Bool {
        return getNumMatches(tile) == 3
    }

    private func getNumMatches(_ tile: Tile) -> Int {
        var matches = 0

        for t in allTiles() {
            for key in t.keys {
                if tile.keys.contains(key) {
                    matches += 1
                    break
                }
            }
        }

        return matches
    }

    private func allTiles() -> [Tile] {
        return remaining + deployed.values
    }

    private func findAdjacentTiles(to: Pos) -> [Tile] {
        var adjacencies: [Tile] = []
        let adjacentAddresses = [
            Pos(to.i - 1, to.j),
            Pos(to.i, to.j + 1),
            Pos(to.i + 1, to.j),
            Pos(to.i, to.j - 1),
        ]

        for address in adjacentAddresses {
            if let tile = deployed[address] {
                adjacencies.append(tile)
            }
        }

        return adjacencies
    }

    private func findPossibleTiles(adjacencies: [Tile]) -> [Tile] {
        var possibilities = remaining

        for adjacency in adjacencies {
            possibilities = possibilities.filter { $0.matchesAtLeastOne(of: adjacency.keys) }
        }

        return possibilities
    }
}

struct Tile {
    let id: Int
    let keys: [Int]
    let preciseKeys: [Int]
    let image: [[String]]

    init(id: Int, image: [[String]]) {
        self.id = id
        self.image = image
        let keys = Tile.buildKeys(image: image)
        self.keys = keys.map { $0.1 }
        preciseKeys = keys.map { $0.0 }
    }

    func toString() -> String {
        var string = ""

        for row in image {
            string += row.joined() + "\n"
        }

        return string.replacingOccurrences(of: "1", with: "#").replacingOccurrences(of: "0", with: ".")
    }

    func matchesAtLeastOne(of: [Int]) -> Bool {
        for key in of {
            if keys.contains(key) {
                return true
            }
        }

        return false
    }

    func flipped(around: Axis) -> Tile {
        if around == .vertical {
            // print(toString())
            let image: [[String]] = self.image.map { $0.reversed() }

            let tile = Tile(id: id, image: image)
            // print()
            // print(tile.toString())
            return tile
        } else {
            // print(toString())
            let image: [[String]] = self.image.reversed()

            let tile = Tile(id: id, image: image)
            // print()
            // print(tile.toString())
            return tile
        }
    }

    func rotated(_ times: Int) -> Tile {
        if times == 0 {
            return self
        }

        // just initialising it with the correct dimensions...
        var newImage = image
        for (i, row) in image.enumerated() {
            for (j, _) in row.enumerated() {
                // represents a 90° clockwise rotation
                let index = row.count - i - 1
                newImage[j][index] = image[i][j]
            }
        }

        let tile = Tile(id: id, image: newImage)

        return tile.rotated(times - 1)
    }

    func isSamePlaneAs(tile: Tile) -> Bool {
        for key in preciseKeys {
            if tile.preciseKeys.contains(key) {
                return true
            }
        }

        return false
    }

    static func buildKeys(image: [[String]]) -> [(Int, Int)] {
        var eastRow: [String] = []
        var westRow: [String] = []
        for row in image {
            westRow.append(row.first!)
            eastRow.append(row.last!)
        }

        let north = buildKey(row: image.first!)
        let east = buildKey(row: eastRow)
        let south = buildKey(row: image.last!.reversed())
        let west = buildKey(row: westRow.reversed())

        return [north, east, south, west]
    }

    static func buildKey(row: [String]) -> (Int, Int) {
        let rev = row.reversed()
        let rowValue = Int(row.joined(), radix: 2)!
        let revValue = Int(rev.joined(), radix: 2)!

        let concat: [String]
        if rowValue > revValue {
            concat = row + rev
        } else {
            concat = rev + row
        }

        let looseKey = Int(concat.joined(), radix: 2)!

        return (rowValue, looseKey)
    }
}

struct Pos: Hashable {
    let i, j: Int

    init(_ i: Int, _ j: Int) {
        self.i = i
        self.j = j
    }
}

enum Axis {
    case horizontal, vertical
}
