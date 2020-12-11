
import Foundation

let data = try Data(contentsOf: URL(fileURLWithPath: "input"))
let string = String(decoding: data, as: UTF8.self)
let boardingPasses = string.components(separatedBy: "\n")

var maxId = 0
var allIds: [Int] = []
for boardingPass in boardingPasses {
    if boardingPass.count != 10 {
        continue
    }
    
    let pass = BoardingPass(boardingPass)
    if pass.getId() > maxId {
        maxId = pass.getId()
    }
    
    allIds.append(pass.getId())
}

allIds = allIds.sorted()
var lastId = -1
for id in allIds {
    if id == lastId + 2 {
        print("My ID is \(id - 1)")
        break
    }
    
    lastId = id
}

print(allIds)
print("Maximum ID is \(maxId)")

struct BoardingPass {
    let row: Int
    let column: Int
    
    init(_ pass: String) {
        let rowCode = String(Array(pass)[...6])
            .replacingOccurrences(of: "F", with: "0")
            .replacingOccurrences(of: "B", with: "1")
        let seatCode = String(Array(pass)[7...9])
            .replacingOccurrences(of: "L", with: "0")
            .replacingOccurrences(of: "R", with: "1")
        
        self.row = Int(rowCode, radix: 2)!
        self.column = Int(seatCode, radix: 2)!
    }
    
    func getId() -> Int {
        return (row * 8) + column
    }
    
}
