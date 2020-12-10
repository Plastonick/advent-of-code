
import Foundation

let passwordData = try Data(contentsOf: URL(fileURLWithPath: "input"))
let passwordString = String(decoding: passwordData, as: UTF8.self)
let passwords = passwordString.components(separatedBy: "\n")

var numValid = 0
for password in passwords {
    if password.count == 0 {
        continue
    }
    
    let parsed = parsePasswordLine(line: password)
    
    let numOccurrences = countOccurrencesOf(sub: parsed.0, inTarget: parsed.3)
    
    if numOccurrences < parsed.1 {
        // this isn't valid, don't count it
        continue
    }
    
    if numOccurrences > parsed.2 {
        // this isn't valid, don't count it
        continue
    }
    
    numValid += 1
}

print("There are \(numValid) valid passwords")

func parsePasswordLine(line: String) -> (String, Int, Int, String) {
    let parts = line.components(separatedBy: " ")
    
    let range = parts[0].components(separatedBy: "-")
    let char = parts[1].components(separatedBy: ":")[0]
    let password = parts[2]
    let min = Int(range[0])
    let max = Int(range[1])
    
    let ret: (String, Int, Int, String) = (char, min!, max!, password)
    
    return ret
}

func countOccurrencesOf(sub: String, inTarget: String) -> Int {
    return inTarget.components(separatedBy: sub).count - 1
}
