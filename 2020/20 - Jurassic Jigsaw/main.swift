
import Foundation

let data = try! Data(contentsOf: URL(fileURLWithPath: "input"))
let string = String(decoding: data, as: UTF8.self)
let tileSize = 10
var tiles: [Tile] = []

for tileString in string.components(separatedBy: "\n\n") {
    let tile = buildTileFromString(tileString)

    tiles.append(tile)
}

var potentialCornerTiles: [Tile] = []

for tile in tiles {
    var outsideEdges = 0
    for key in tile.keys {
        if numMatches(key: key, tiles: tiles) == 1 {
            // it only matches itself, it must be on the edge!
            outsideEdges += 1
        }
    }

    if outsideEdges == 2 {
        potentialCornerTiles.append(tile)
    }
}

if potentialCornerTiles.count == 4 {
    print("Found the four corner tiles! The product of their IDs is \(potentialCornerTiles.reduce(1) { $0 * $1.id })")
} else {
    print("Naive solution didn't work, sadly")
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
    var eastRow: [String] = []
    var westRow: [String] = []
    for row in image {
        westRow.append(row.first!)
        eastRow.append(row.last!)
    }

    let north = buildLooseKey(row: image.first!)
    let east = buildLooseKey(row: eastRow)
    let south = buildLooseKey(row: image.last!)
    let west = buildLooseKey(row: westRow)

    return Tile(id: id, keys: [north, east, south, west])
}

func buildLooseKey(row: [String]) -> Int {
    let rev = row.reversed()
    let rowValue = Int(row.joined(), radix: 2)!
    let revValue = Int(rev.joined(), radix: 2)!

    let concat: [String]
    if rowValue > revValue {
        concat = row + rev
    } else {
        concat = rev + row
    }

    return Int(concat.joined(), radix: 2)!
}

struct Tile {
    let id: Int
    let keys: [Int]
}
