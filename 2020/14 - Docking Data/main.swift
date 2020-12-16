
import Foundation

let data = try Data(contentsOf: URL(fileURLWithPath: "input"))
let string = String(decoding: data, as: UTF8.self)

var currentMask: Mask?
var memory: [Int: Int] = [:]

for row in string.components(separatedBy: "\n") {
    if row.contains("mask") {
        var chars = Array(row).map { String($0) }
        chars.removeFirst(7)

        currentMask = Mask(mask: chars)
        continue
    }

    guard let mask = currentMask else {
        continue
    }

    let pieces = row.components(separatedBy: " ")
    let memoryAddress = getMemoryAddress(pieces[0])
    let value = Int(pieces[2])!
    let bin = String(memoryAddress, radix: 2)

    let maskedBinaries = mask.apply(binary: bin)

    for maskedBinary in maskedBinaries {
        memory[Int(maskedBinary.joined(), radix: 2)!] = value
    }
}

var sum = 0

for (_, value) in memory {
    sum += value
}

print(sum)

func getMemoryAddress(_ string: String) -> Int {
    var intChars: [String] = []
    for char in Array(string) {
        guard let _ = Int(String(char)) else {
            continue
        }

        intChars.append(String(char))
    }

    return Int(intChars.joined())!
}

struct Mask {
    let mask: [String]

    func apply(binary: String) -> [[String]] {
        var binaryChars = Array(binary).map { String($0) }

        for _ in 1 ... (36 - binaryChars.count) {
            binaryChars.insert("0", at: 0)
        }

        let maskChars = Array(mask).map { String($0) }
        var resultStringArrays = [binaryChars]

        for (i, char) in maskChars.enumerated() {
            if char == "0" {
                // do nothing, leave the result
                continue
            }

            if char == "1" {
                resultStringArrays = writeOne(at: i, stringArrays: resultStringArrays)
                continue
            }

            if char == "X" {
                resultStringArrays = duplicate(stringArrays: resultStringArrays, at: i)
                continue
            }
        }

        return resultStringArrays
    }

    func writeOne(at: Int, stringArrays: [[String]]) -> [[String]] {
        var retVal: [[String]] = []

        for stringArray in stringArrays {
            var newStringArray = stringArray
            newStringArray[at] = "1"

            retVal.append(newStringArray)
        }

        return retVal
    }

    func duplicate(stringArrays: [[String]], at: Int) -> [[String]] {
        var retVal: [[String]] = []

        for char in ["1", "0"] {
            for stringArray in stringArrays {
                var newStringArray = stringArray
                newStringArray[at] = char

                retVal.append(newStringArray)
            }
        }

        return retVal
    }
}
