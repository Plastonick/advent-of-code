
import Foundation

let data = try! Data(contentsOf: URL(fileURLWithPath: "input"))
let string = String(decoding: data, as: UTF8.self)

var unconfirmedAllergens: [String: Set<String>] = [:]
var allAlergens: Set<String> = []
var everyIngredientInstance: [String] = []

for row in string.components(separatedBy: "\n") {
    if row == "" {
        break
    }

    let pieces = row.components(separatedBy: " (contains ")
    let ingredients = pieces[0].components(separatedBy: " ")

    everyIngredientInstance.append(contentsOf: ingredients)
    let allergens = pieces[1].replacingOccurrences(of: ")", with: "").components(separatedBy: ", ")

    allAlergens = allAlergens.union(allergens)

    for allergen in allergens {
        if unconfirmedAllergens[allergen] == nil {
            unconfirmedAllergens[allergen] = Set(ingredients)
        } else {
            unconfirmedAllergens[allergen] = unconfirmedAllergens[allergen]!.intersection(Set(ingredients))
        }
    }
}

var confirmedAllergens: [String: String] = [:]

var modification = true
while modification {
    modification = false
    for (allergen, ingredients) in unconfirmedAllergens {
        if ingredients.count == 1 {
            modification = true
            let allergenicIngredient = ingredients.first!

            confirmedAllergens[allergen] = allergenicIngredient
            unconfirmedAllergens = remove(ingr: allergenicIngredient, from: unconfirmedAllergens)
        }
    }
}

print(confirmedAllergens)
print(everyIngredientInstance.filter { !confirmedAllergens.values.contains($0) }.count)

var canonicalDangerousIngredients: [String] = []
for allergen in Array(confirmedAllergens.keys).sorted() {
    canonicalDangerousIngredients.append(confirmedAllergens[allergen]!)
}

print(canonicalDangerousIngredients.joined(separator: ","))

func remove(ingr: String, from: [String: Set<String>]) -> [String: Set<String>] {
    var newDict: [String: Set<String>] = [:]
    for (allergen, ingredients) in from {
        let newIngredients = ingredients.filter { $0 != ingr }

        if newIngredients.count >= 1 {
            newDict[allergen] = newIngredients
        }
    }
    return newDict
}
