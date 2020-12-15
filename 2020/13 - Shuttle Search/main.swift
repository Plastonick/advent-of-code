
import Foundation

let arrivalTime = 1008713
let busIds = ["13", "x", "x", "41", "x", "x", "x", "x", "x", "x", "x", "x", "x", "467", "x", "x", "x", "x", "x", "x", "x", "x", "x", "x", "x", "19", "x", "x", "x", "x", "17", "x", "x", "x", "x", "x", "x", "x", "x", "x", "x", "x", "29", "x", "353", "x", "x", "x", "x", "x", "37", "x", "x", "x", "x", "x", "x", "x", "x", "x", "x", "x", "x", "x", "x", "x", "x", "23"]

var sum = 1000



var bestSolution = (-1, -1)

var a: [Int] = []
var n: [Int] = []

for (i, busId) in busIds.enumerated() {
    guard let busFrequency = Int(busId) else {
        continue
    }

    let waitTime = busFrequency - (arrivalTime % busFrequency)

    let remainder = (((busFrequency - i) % busFrequency) + busFrequency) % busFrequency
    n.append(busFrequency)
    a.append(remainder)

    if bestSolution.0 == -1 || waitTime < bestSolution.1 {
        bestSolution = (busFrequency, waitTime)
    }
}

print(n.map { String($0)}.joined(separator: ","))
print(a.map { String($0)}.joined(separator: ","))

print(bestSolution.0 * bestSolution.1)
