
import Foundation

let data = try Data(contentsOf: URL(fileURLWithPath: "input"))
let string = String(decoding: data, as: UTF8.self)
let numbers = string.components(separatedBy: "\n").map { Int($0)! }
let orderedJoltages = numbers.sorted()
var jumps: [Int: Int] = [:]
var previousJoltage = 0

print(orderedJoltages.count)

for joltage in orderedJoltages {
    let jump = joltage - previousJoltage

    if jump > 3 {
        print("damn")
    }

    if jumps[jump] != nil {
        jumps[jump]! += 1
    } else {
        jumps[jump] = 1
    }

    previousJoltage = joltage
}

jumps[3]! += 1 // becuase why not

print(jumps)
print("Result for part 1 is \(jumps[1]! * jumps[3]!)")

let joltages = Joltages(orderedJoltages)

print("There are \(joltages.countAllSteps(from: 0)) iterations")

class Joltages {
    let availableAdapters: Set<Int>
    var memo: [Int: Int] = [:]

    init (_ joltages: [Int]) {
        self.availableAdapters = Set(joltages)
    }

    func countAllSteps(from: Int) -> Int {
        let nextSteps = self.getAllPossibleSteps(from: from)

        if let steps = self.memo[from] {
            return steps
        }

        if nextSteps.count == 0 {
            self.memo[from] = 1
            return 1
        }

        let numSteps = nextSteps.reduce(0) { $0 + countAllSteps(from: $1) }
        self.memo[from] = numSteps

        return numSteps
    }

    func getAllPossibleSteps(from: Int) -> [Int] {
        var possibleSteps: [Int] = []

        for i in 1...3 {
            if self.availableAdapters.contains(from + i) {
                possibleSteps.append(from + i)
            }
        }

        return possibleSteps
    }
}

