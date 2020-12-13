
import Foundation

let data = try Data(contentsOf: URL(fileURLWithPath: "input"))
let string = String(decoding: data, as: UTF8.self)
let numbers = string.components(separatedBy: "\n").map { Int($0)! }

var index = 25

while true {
    if index > numbers.count - 1 {
        break
    }

    guard let _ = numbersThatSumTo(target: numbers[index], fromList: Array(numbers[index-25...index-1])) else {
        break
    }

    index += 1
}

let invalidValue = numbers[index]

print("Program terminated at index \(index) with value \(numbers[index])")

for (index, _) in numbers.enumerated() {
    if let weirdValue = weirdMethod(target: invalidValue, list: Array(numbers[index...numbers.count - 1])) {
        print("Found the weakness: \(weirdValue)")
    }
}

func weirdMethod(target: Int, list: [Int]) -> Int? {
    var remaining = target
    var smallest: Int? = nil
    var largest: Int? = nil

    for value in list {
        remaining = remaining - value

        if smallest == nil || smallest! > value {
            smallest = value
        }

        if largest == nil || largest! < value {
            largest = value
        }

        if remaining == 0 {
            return smallest! + largest!
        }

        if remaining < 0 {
            return nil
        }
    }

    return nil
}

func numbersThatSumTo(target: Int, fromList: [Int]) -> (Int, Int)? {
    for value in fromList {
        let complement = target - value
        
        if fromList.contains(complement) {
            return (value, complement)
        }
    }
    
    return nil
}
