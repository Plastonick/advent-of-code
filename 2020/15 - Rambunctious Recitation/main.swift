
import Foundation

let numbers = [2, 1, 10, 11, 0, 6]
// let numbers = [0,3,6]
var memory: [Int: (Int?, Int)] = [:]
var lastNumber = -1
var speak = -1

for (i, number) in numbers.enumerated() {
    memory[number] = (nil, i)
    lastNumber = number
    print(i + 1, number)
}

print(memory)

for i in (numbers.count)..<1000 {
    if let mem = memory[lastNumber] {
        // we've spoken this before at least once
        if let penultimateTime = mem.0 {
            speak = mem.1 - penultimateTime
        } else {
            speak = 0
        }
    } else {
        speak = 0
    }

    // update memory for the number I've just spoken
    let penultimateTime: Int?
    if let mem = memory[speak] {
        penultimateTime = mem.1
    } else {
        penultimateTime = nil
    }

    memory[speak] = (penultimateTime, i)

    // set the last number for the next iteration
    lastNumber = speak
}

print(lastNumber)