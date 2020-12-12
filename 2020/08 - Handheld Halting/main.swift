
import Foundation

let data = try Data(contentsOf: URL(fileURLWithPath: "input"))
let string = String(decoding: data, as: UTF8.self)
let rows = string.components(separatedBy: "\n")

if let terminationValue = findTerminationValue(rows: rows) {
    print("Found a termination value \(terminationValue)")
}

func findTerminationValue(rows: [String]) -> Int? {
    for changeLine in 0..<rows.count {
        var newRows = rows
        var state = State(line: 0, value: 0)
        var visitedLines: [Int] = []
        
        if newRows[changeLine].contains("nop") {
            newRows[changeLine] = newRows[changeLine].replacingOccurrences(of: "nop", with: "jmp")
        } else if newRows[changeLine].contains("jmp") {
            newRows[changeLine] = newRows[changeLine].replacingOccurrences(of: "jmp", with: "nop")
        } else {
            continue
        }

        while !visitedLines.contains(state.line) {
            if let instruction = getInstruction(line: state.line, rows: newRows) {
                visitedLines.append(state.line)
                state = state.applyInstruction(instruction)
            } else {
                return state.value
            }
        }
    }

    return nil
}

func getInstruction(line: Int, rows: [String]) -> Instruction? {
    if line > rows.count - 1 {
        return nil
    }
    
    let row = rows[line]
    let pieces = row.components(separatedBy: " ")

    if pieces.count < 2 {
        return nil
    }

    let rule = matchRule(String(pieces[0]))
    let value = Int(String(pieces[1]))!

    return Instruction(rule: rule, value: value)
}

func matchRule(_ rule: String) -> Rule {
    switch rule {
    case "acc":
        return Rule.acc
    case "jmp":
        return Rule.jmp
    case "nop":
        return Rule.nop
    default:
        return Rule.nop
    }
}

struct State {
    let line: Int
    let value: Int

    init(line: Int, value: Int) {
        self.line = line
        self.value = value
    }

    func applyInstruction(_ instruction: Instruction) -> State {
        switch instruction.rule {
        case Rule.acc:
            return State(line: self.line + 1, value: self.value + instruction.value)
        case Rule.jmp:
            return State(line: self.line + instruction.value, value: self.value)
        case Rule.nop:
            return State(line: self.line + 1, value: self.value)
        }
    }
}

struct Instruction {
    let rule: Rule
    let value: Int

    init(rule: Rule, value: Int) {
        self.rule = rule
        self.value = value
    }
}

enum Rule {
    case acc, jmp, nop
}
