
import Foundation

let initialState = [2, 8, 4, 5, 7, 3, 9, 6, 1]
var state = State(sequence: initialState, index: 0)

for _ in 1 ... 100 {
    state = state.iterate()
}

let endSequence = state.sequence
let onePos = endSequence.firstIndex(of: 1)!
let results = Array(endSequence[(onePos + 1)...] + endSequence[0 ..< onePos])

print(results.map { String($0) }.joined())

struct State {
    let sequence: [Int]
    let index: Int

    public func iterate() -> State {
        var sequence = self.sequence
        let currentCup = sequence.first!

        // remove three cups clockwise of the current cup
        let removals = Array(sequence[1 ... 3])
        sequence = Array(sequence[0 ... 0] + sequence[4...])

        var destinationValue = getDestinationValue(cur: currentCup)
        while true {
            guard let destination = sequence.firstIndex(of: destinationValue) else {
                destinationValue = getDestinationValue(cur: destinationValue)
                continue
            }

            sequence.insert(contentsOf: removals, at: destination + 1)
            break
        }

        sequence.removeFirst()
        sequence.append(currentCup)

        return State(sequence: sequence, index: (index + 1) % 10)
    }

    private func getDestinationValue(cur: Int) -> Int {
        if cur == 1 {
            return 9
        } else {
            return cur - 1
        }
    }
}
