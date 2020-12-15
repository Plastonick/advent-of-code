
import Foundation

let data = try Data(contentsOf: URL(fileURLWithPath: "input"))
let string = String(decoding: data, as: UTF8.self)
let instructions = string.components(separatedBy: "\n")
var state = State(coord: Coordinate(x: 0, y: 0), waypoint: Coordinate(x: 10, y: 1))

for instructionString in instructions {
    var pieces = Array(instructionString).map { String($0) }

    let action = Action.from(string: pieces.removeFirst())!
    let value = Int(pieces.joined())!

    let instruction = Instruction(action: action, value: value)

    state = state.with(instruction: instruction)
    let distance = abs(state.coord.x) + abs(state.coord.y)

    print(state.coord, state.waypoint, instruction, distance)
}

struct Coordinate {
    let x, y: Int
}

struct State {
    let coord, waypoint: Coordinate

    func with(instruction: Instruction) -> State {
        switch instruction.action {
        case Action.L, Action.R:
            return turn(instruction)
        case Action.F:
            return move(instruction)
        case Action.N, Action.E, Action.S, Action.W:
            return moveWaypoint(instruction)
        }
    }

    private func turn(_ instruction: Instruction) -> State {
        let turn: Int
        if instruction.action == Action.R {
            turn = 360 - instruction.value
        } else {
            turn = instruction.value
        }

        let waypoint: Coordinate

        switch turn {
        case 90:
            waypoint = Coordinate(x: -self.waypoint.y, y: self.waypoint.x)
        case 180:
            waypoint = Coordinate(x: -self.waypoint.x, y: -self.waypoint.y)
        case 270:
            waypoint = Coordinate(x: self.waypoint.y, y: -self.waypoint.x)

        case 0:
            fallthrough
        default:
            waypoint = self.waypoint
        }

        return State(coord: coord, waypoint: waypoint)
    }

    private func move(_ instruction: Instruction) -> State {
        let coord = Coordinate(
            x: self.coord.x + (instruction.value * waypoint.x),
            y: self.coord.y + (instruction.value * waypoint.y)
        )

        return State(coord: coord, waypoint: waypoint)
    }

    private func moveWaypoint(_ instruction: Instruction) -> State {
        let waypoint: Coordinate
        switch instruction.action {
        case Action.E:
            waypoint = Coordinate(x: self.waypoint.x + instruction.value, y: self.waypoint.y)
        case Action.N:
            waypoint = Coordinate(x: self.waypoint.x, y: self.waypoint.y + instruction.value)
        case Action.W:
            waypoint = Coordinate(x: self.waypoint.x - instruction.value, y: self.waypoint.y)
        case Action.S:
            fallthrough
        default:
            waypoint = Coordinate(x: self.waypoint.x, y: self.waypoint.y - instruction.value)
        }

        return State(coord: coord, waypoint: waypoint)
    }

    private func getDirection(degrees: Int) -> Action {
        switch degrees {
        case 0:
            return Action.E
        case 90:
            return Action.N
        case 180:
            return Action.W
        case 270:
            fallthrough
        default:
            return Action.S
        }
    }
}

struct Instruction {
    let action: Action
    let value: Int
}

extension CaseIterable {
    static func from(string: String) -> Self? {
        return Self.allCases.first { string == "\($0)" }
    }

    func toString() -> String { "\(self)" }
}

enum Action: CaseIterable {
    case N, E, S, W, L, R, F
}
