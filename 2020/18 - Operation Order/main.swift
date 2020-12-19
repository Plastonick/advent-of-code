
import Foundation

let data = try Data(contentsOf: URL(fileURLWithPath: "input"))
let string = String(decoding: data, as: UTF8.self)

let rows = string.components(separatedBy: "\n")
let calc = Claculator()

let sum = rows.reduce(0) { $0 + calc.calculate($1) }
print(sum)

// print(calc.calculate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"))

class Claculator {
    func calculate(_ s: String) -> Int {
        var str = s
        print(s)

        str = replaceSums(str)
        str = replaceBrackets(str)

        return flatCalculation(str)
    }

    private func replaceSums(_ s: String) -> String {
        var str = s

        while let innerOperation = findBasicSums(str) {
            let result = flatCalculation(innerOperation)
            str = str.replacingOccurrences(of: innerOperation, with: String(result))
            print("\(innerOperation) = \(result) --->>", str)
        }

        return str
    }

    private func replaceBrackets(_ s: String) -> String {
        var str = s

        while let innerOperation = findInnerCalculation(str) {
            let result = flatCalculation(innerOperation)
            str = str.replacingOccurrences(of: innerOperation, with: String(result))
            print("\(innerOperation) = \(result) --->>", str)

            str = replaceSums(str)
        }

        return str
    }

    private func findInnerCalculation(_ s: String) -> String? {
        let range = NSRange(location: 0, length: s.count)
        let innerRegex = try! NSRegularExpression(pattern: "\\([^\\(\\)]*\\)")

        if let match = innerRegex.firstMatch(in: s, options: [], range: range) {
            return String(s[Range(match.range, in: s)!])
        }

        return nil
    }

    private func findBasicSums(_ s: String) -> String? {
        let range = NSRange(location: 0, length: s.count)
        let innerRegex = try! NSRegularExpression(pattern: "\\d+\\s\\+\\s\\d+")

        if let match = innerRegex.firstMatch(in: s, options: [], range: range) {
            return String(s[Range(match.range, in: s)!])
        }

        return nil
    }

    private func flatCalculation(_ s: String) -> Int {
        let string = s.replacingOccurrences(of: "(", with: "").replacingOccurrences(of: ")", with: "")
        let chars = string.components(separatedBy: " ").map { String($0) }
        var val: Int?
        var lastOperation: Operation?

        for char in chars {
            if let integer = Int(char) {
                if val == nil {
                    val = integer
                } else {
                    if lastOperation == Operation.add {
                        val! += integer
                    } else {
                        val! *= integer
                    }
                }
            } else if let operation = Operation.from(char) {
                lastOperation = operation
            }
        }

        if val == nil {
            print(s)
            print(string)
            print("damn")
            exit(1)
        }

        return val!
    }
}

enum Operation {
    case add, multiply

    static func from(_ string: String) -> Operation? {
        if string == "+" {
            return Operation.add
        }

        if string == "*" {
            return Operation.multiply
        }

        return nil
    }
}
