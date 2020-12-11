
import Foundation

let passwordData = try Data(contentsOf: URL(fileURLWithPath: "input"))
let passwordString = String(decoding: passwordData, as: UTF8.self)
let passwords = passwordString.components(separatedBy: "\n")

var numValidByFirstRule = 0
var numValidBySecondRule = 0
for password in passwords {
    if password.count == 0 {
        continue
    }
    
    let passwordLine = parsePasswordLine(line: password)
    
    if isValidByFirstRule(passwordLine: passwordLine) {
        numValidByFirstRule += 1
    }
    
    if isValidBySecondRule(passwordLine: passwordLine) {
        numValidBySecondRule += 1
    }
}

print("There are \(numValidByFirstRule) valid passwords by the first rule")
print("There are \(numValidBySecondRule) valid passwords by the second rule")

struct PasswordLine {
    let min, max: Int
    let character, password: String
    
    init(min: Int, max: Int, character: String, password: String) {
        self.min = min
        self.max = max
        self.character = character
        self.password = password
    }
}

func isValidByFirstRule(passwordLine: PasswordLine) -> Bool {
    let numOccurrences = countOccurrencesOf(sub: passwordLine.character, inTarget: passwordLine.password)
    
    if numOccurrences < passwordLine.min {
        // this isn't valid, don't count it
        return false
    }
    
    if numOccurrences > passwordLine.max {
        // this isn't valid, don't count it
        return false
    }
    
    return true
}


func isValidBySecondRule(passwordLine: PasswordLine) -> Bool {
    let characters = Array(passwordLine.password)
    var numMatches = 0
    
    let minthChar = String(characters[passwordLine.min - 1])
    if minthChar == passwordLine.character {
        numMatches += 1
    }
    
    let maxthChar = String(characters[passwordLine.max - 1])
    if maxthChar == passwordLine.character {
        numMatches += 1
    }

    
    return numMatches == 1
}

func parsePasswordLine(line: String) -> PasswordLine {
    let parts = line.components(separatedBy: " ")
    
    let range = parts[0].components(separatedBy: "-")
    let char = parts[1].components(separatedBy: ":")[0]
    let password = parts[2]
    let min = Int(range[0])!
    let max = Int(range[1])!
    
    let passwordLine = PasswordLine(min: min, max: max, character: char, password: password)
    
    return passwordLine
}

func countOccurrencesOf(sub: String, inTarget: String) -> Int {
    return inTarget.components(separatedBy: sub).count - 1
}
