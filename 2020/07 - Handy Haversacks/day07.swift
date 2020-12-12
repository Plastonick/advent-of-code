
import Foundation

let data = try Data(contentsOf: URL(fileURLWithPath: "input"))
let string = String(decoding: data, as: UTF8.self)
let bagRules = string.components(separatedBy: "\n")
var chainBroken = false
var suitableColours = ["shiny gold"]

while !chainBroken {
    chainBroken = true
    
    for bagRule in bagRules {
        let rulePieces = bagRule.components(separatedBy: " contain ")
        
        if rulePieces.count != 2 {
            continue
        }
        
        let masterBag = rulePieces[0].replacingOccurrences(of: "bags", with: "bag")
        if suitableColours.contains(masterBag) {
            continue
        }
        
        let interiorBags = rulePieces[1]
        
        for suitableColour in suitableColours {
            if interiorBags.contains(suitableColour) {
                chainBroken = false
                suitableColours.append(masterBag)
                
                break
            }
        }
    }
}

print("There are \(suitableColours.count - 1) bags which could eventually contain shiny gold, not including shiny gold")

let engine = RussianDollEngine(rules: bagRules)

print("I need at least \(engine.sumInteriorBags(master: "shiny gold bag") - 1) bags for my shiny gold one")

struct RussianDollEngine {
    let rules: [String]
    
    init(rules: [String]) {
        self.rules = rules
    }
    
    func sumInteriorBags(master: String) -> Int {
        var sum = 1
        
        if let rule = getBagRule(bag: master) {
            let interiorRules = parseRule(rule: rule)
            
            for interiorRule in interiorRules {
                print(interiorRule)
                sum += interiorRule.0 * self.sumInteriorBags(master: interiorRule.1)
            }
        }
        
        return sum
    }
    
    func parseRule(rule: String) -> [(Int, String)] {
        let pieces = rule.components(separatedBy: ", ").map {
            $0.replacingOccurrences(of: "bags", with: "bag")
                .replacingOccurrences(of: "bag.", with: "bag")
        }
        
        var parses: [(Int, String)] = []
        for piece in pieces {
            let chars = Array(piece)
            
            
            if let num = Int(String(chars[0])) {
                let bag = String(chars[2..<chars.count])
                
                parses.append((num, bag))
            } else {
                parses.append((0, "not a bag"))
            }
        }
        
        return parses
    }

    func getBagRule(bag: String) -> String? {
        for rule in self.rules {
            let rulePieces = rule.components(separatedBy: " contain ")
            
            if rulePieces.count != 2 {
                continue
            }
            
            let bagRule = rulePieces[0].replacingOccurrences(of: "bags", with: "bag")
            
            if bagRule == bag {
                return rulePieces[1]
            }
        }
        
        return nil
    }
}
