
import Foundation

let data = try! Data(contentsOf: URL(fileURLWithPath: "rules"))
let string = String(decoding: data, as: UTF8.self)
let rows = string.components(separatedBy: "\n").enumerated()
var rules: [String] = []
// longest line - shortest line
// let maxDiff = 96 - 24
let maxDiff = 4

for (i, row) in rows {
    if i == 127 {
        // silly blank line...
        break
    }

    if row == "\"a\"" || row == "\"b\"" {
        rules.append(row.replacingOccurrences(of: "\"", with: ""))
    } else {
        let dedupedRow: String
        if containsRule(row, i) {
            dedupedRow = removeLoop(rule: row, num: i, maxTimes: maxDiff)
        } else {
            dedupedRow = row
        }

        // print(dedupedRow)

        let pieces = dedupedRow.components(separatedBy: " | ")
        var escapedPieces: [String] = []
        for piece in pieces {
            let numbers = piece.components(separatedBy: " ")
            var escapedNumbers = ""
            for number in numbers {
                if let num = Int(number) {
                    escapedNumbers += "~\(number)~"
                } else {
                    escapedNumbers += number
                }
            }
            escapedPieces.append(escapedNumbers)
        }

        rules.append(escapedPieces.joined(separator: " | "))
    }
}

var patternString = rules.first!

while let escapedNumber = findEscapedNumber(patternString) {
    // print(patternString)

    // sleep(1)
    let number = Int(escapedNumber.replacingOccurrences(of: "~", with: ""))!
    let referencedRule = rules[number]

    patternString = patternString.replacingOccurrences(of: escapedNumber, with: "(\(referencedRule))")
}

// neaten this up... horribly
let finalRegex = "grep -E '^" + patternString
    .replacingOccurrences(of: " ", with: "")
    .replacingOccurrences(of: "(a)", with: "a")
    .replacingOccurrences(of: "(b)", with: "b") + "$' input | wc -l"

print(finalRegex)
private func findEscapedNumber(_ s: String) -> String? {
    let range = NSRange(location: 0, length: s.count)
    let innerRegex = try! NSRegularExpression(pattern: "~\\d+~")

    if let match = innerRegex.firstMatch(in: s, options: [], range: range) {
        return String(s[Range(match.range, in: s)!])
    }

    return nil
}

private func containsRule(_ s: String, _ rule: Int) -> Bool {
    let range = NSRange(location: 0, length: s.count)
    let innerRegex = try! NSRegularExpression(pattern: "(^\(rule)[^\\d])|([^\\d]\(rule)[^\\d])|([^\\d]\(rule)$)")

    return innerRegex.firstMatch(in: s, options: [], range: range) != nil
}

private func removeLoop(rule: String, num: Int, maxTimes: Int) -> String {
    var newRule = rule

    for _ in 1 ... maxTimes {
        // print(newRule)
        newRule = newRule.replacingOccurrences(of: String(num), with: "( \(rule) )")
    }

    return newRule.replacingOccurrences(of: " \(num)", with: "")
}
