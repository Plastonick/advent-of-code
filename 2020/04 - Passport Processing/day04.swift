
import Foundation

let data = try Data(contentsOf: URL(fileURLWithPath: "input"))
let string = String(decoding: data, as: UTF8.self)

let passportStrings = string.components(separatedBy: "\n\n")

var triviallyValid = 0
var stronglyValid = 0
for passportString in passportStrings {
    if containsRequiredKeys(passport: passportString) {
        triviallyValid += 1
    } else {
        continue
    }
    
    if isStronglyValid(passport: passportString) {
        stronglyValid += 1
//        print(passportString)
//        print("-- BREAK --")
    }
}

print("There are \(triviallyValid) trivially valid passports")
print("There are \(stronglyValid) strongly valid passports")

func containsRequiredKeys(passport: String) -> Bool {
    let requiredKeys = [
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid"
    ]
    
    for key in requiredKeys {
        if !passport.contains(key + ":") {
            return false
        }
    }
    
    return true
}

func isStronglyValid(passport: String) -> Bool {
    if let val = getKeyValue(passport: passport, key: "byr") {
        let intVal = Int(val)!
        
        if intVal < 1920 || intVal > 2002 {
            return false
        }
    } else {
        return false
    }
    
    if let val = getKeyValue(passport: passport, key: "iyr") {
        let intVal = Int(val)!
        
        if intVal < 2010 || intVal > 2020 {
            return false
        }
    } else {
        return false
    }
    
    if let val = getKeyValue(passport: passport, key: "eyr") {
        let intVal = Int(val)!
        
        if intVal < 2020 || intVal > 2030 {
            return false
        }
    } else {
        return false
    }
    
    if let val = getKeyValue(passport: passport, key: "hgt") {
        let range = NSRange(location: 0, length: val.count)
        let cmRegex = try! NSRegularExpression(pattern: "[0-9]+cm")
        let inchRegex = try! NSRegularExpression(pattern: "[0-9]+in")
        let lengthRegex = try! NSRegularExpression(pattern: "[0-9]+(in|cm)")
        
        if let _ = cmRegex.firstMatch(in: val, options: [], range: range) {
            let intVal = Int(val.replacingOccurrences(of: "cm", with: "", options: [], range: nil))!
            
            if intVal < 150 || intVal > 193 {
                return false
            }
        } else if let _ = inchRegex.firstMatch(in: val, options: [], range: range) {
            let intVal = Int(val.replacingOccurrences(of: "in", with: "", options: [], range: nil))!
            
            if intVal < 59 || intVal > 76 {
                return false
            }
        } else if lengthRegex.firstMatch(in: val, options: [], range: range) == nil {
            return false
        }
    } else {
        return false
    }
    
    if let val = getKeyValue(passport: passport, key: "hcl") {
        let range = NSRange(location: 0, length: val.count)
        let hexColourRegex = try! NSRegularExpression(pattern: "^#[0-9a-f]{6}$")
        
        if hexColourRegex.firstMatch(in: val, options: [], range: range) == nil {
            return false
        }
    } else {
        return false
    }
    
    if let val = getKeyValue(passport: passport, key: "ecl") {
        let validColours = [
            "amb",
            "blu",
            "brn",
            "gry",
            "grn",
            "hzl",
            "oth",
        ]
        
        if !validColours.contains(val) {
            return false
        }
    } else {
        return false
    }
    
    if let val = getKeyValue(passport: passport, key: "pid") {
        let range = NSRange(location: 0, length: val.count)
        let nineDigitRegex = try! NSRegularExpression(pattern: "^[0-9]{9}$")
        
        if nineDigitRegex.firstMatch(in: val, options: [], range: range) == nil {
            return false
        }
    } else {
        return false
    }
    
    return true
}

func getKeyValue(passport: String, key: String) -> String? {
    let parsedString = passport.replacingOccurrences(of: "\n", with: " ", options: [], range: nil)
    let allValues = parsedString.components(separatedBy: " ")
    
    for value in allValues {
        if value.contains(key + ":") {
            return value.replacingOccurrences(of: key + ":", with: "", options: [], range: nil)
        }
    }
    
    return nil
}
