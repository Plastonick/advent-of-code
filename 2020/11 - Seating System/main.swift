
import Foundation

let data = try Data(contentsOf: URL(fileURLWithPath: "input"))
let string = String(decoding: data, as: UTF8.self)

var waitingArea = buildWaitingArea(string)

var lastString = ""
var count = 0

while true {
    let newString = waitingArea.toString()
    
    if lastString == newString {
        break
    }

    lastString = newString
    waitingArea = waitingArea.iterate()
}

print(waitingArea.countOccupiedSeats())


func buildWaitingArea(_ seatLayout: String) -> WaitingArea {
    var rows: [Row] = []
    for rowString in seatLayout.components(separatedBy: "\n") {
        let places = rowString.map { (seat: Character) -> Place in
            switch String(seat) {
            case "L":
                return Place.emptySeat
            case "#":
                return Place.occupiedSeat
            case ".":
                fallthrough
            default:
                return Place.floor
            }
        }

        rows.append(Row(places))
    }

    return WaitingArea(rows)
}

struct Row {
    let places: [Place]

    init(_ places: [Place]) {
        self.places = places
    }

    func getPlaceType(_ x: Int) -> Place? {
        if x < 0 || x > places.count - 1 {
            return nil
        }

        return places[x]
    }

    func toString() -> String {
        let placeStrings: [String] = self.places.map {
            switch $0 {
            case Place.floor:
                return "."
            case Place.emptySeat:
                return "L"
            case Place.occupiedSeat:
                return "#"
            }
        }
        
        return placeStrings.joined()
    }
}

enum Place {
    case floor, emptySeat, occupiedSeat
}

struct WaitingArea {
    let rows: [Row]

    init(_ rows: [Row]) {
        self.rows = rows
    }

    func iterate() -> WaitingArea {
        var newRows: [Row] = []

        for (y, row) in rows.enumerated() {
            var newPlaces: [Place] = []

            for (x, place) in row.places.enumerated() {
                newPlaces.append(calculateNewPlaceType(x: x, y: y, place: place))
            }

            newRows.append(Row(newPlaces))
        }

        return WaitingArea(newRows)
    }

    func calculateNewPlaceType(x: Int, y: Int, place: Place) -> Place {
        if place == Place.floor {
            return Place.floor
        }

        let numberOfAdjacentOccupents = getAdjacentPlaces(x: x, y: y).filter { $0 == Place.occupiedSeat }.count

        if numberOfAdjacentOccupents == 0 {
            return Place.occupiedSeat
        }

        if numberOfAdjacentOccupents >= 5 {
            return Place.emptySeat
        }

        return place
    }

    func getAdjacentPlaces(x: Int, y: Int) -> [Place] {
        var places: [Place] = []

        for i in -1 ... 1 {
            for j in -1 ... 1 {
                if i == 0, j == 0 {
                    continue
                }

                var distance = 1
                while true {
                    let place = getPlaceType(x: x + (i * distance), y: y + (j * distance))

                    if place == nil {
                        break
                    }

                    if place! == Place.floor {
                        distance += 1
                        continue
                    }

                    places.append(place!)
                    break
                }
            }
        }

        return places
    }

    func getPlaceType(x: Int, y: Int) -> Place? {
        if y < 0 || y > self.rows.count - 1 {
            return nil
        }

        return self.rows[y].getPlaceType(x)
    }

    func toString() -> String {
        let rowStrings: [String] = self.rows.map { $0.toString() }

        return rowStrings.joined(separator: "\n")
    }

    func countOccupiedSeats() -> Int {
        var count = 0

        for row in self.rows {
            let occupiedSeats = row.places.filter{ $0 == Place.occupiedSeat }
            count += occupiedSeats.count
        }

        return count
    }
}
