
import Foundation

let slopeData = try Data(contentsOf: URL(fileURLWithPath: "input"))
let slopeGrid = String(decoding: slopeData, as: UTF8.self)

let slope = Slope(gridString: slopeGrid)
var position = Vector(x: 0, y: 0)
let vectors = [
    Vector(x: 1, y: 1),
    Vector(x: 3, y: 1),
    Vector(x: 5, y: 1),
    Vector(x: 7, y: 1),
    Vector(x: 1, y: 2)
]

var product = 1
for vector in vectors {
    let numTrees = countArborealIncidents(position: position, vector: vector)

    print("We hit \(numTrees) trees with \(vector)")
    product *= numTrees
}

print("The product is \(product)")

func countArborealIncidents(position: Vector, vector: Vector) -> Int {
    var numTrees = 0

    var newPosition = position.add(vector)
    while let field = slope.getField(newPosition) {
        if field == Field.tree {
            numTrees += 1
        }
        
        newPosition = newPosition.add(vector)
    }
    
    return numTrees
}

struct Vector {
    let x, y: Int
    
    init(x: Int, y: Int) {
        self.x = x
        self.y = y
    }
    
    func add(_ c: Vector) -> Vector {
        let x = self.x + c.x
        let y = self.y + c.y
        
        return Vector(x: x, y: y)
    }
}

struct Slope {
    let grid: [Row]
    
    init(gridString: String) {
        let slopeRows = gridString.components(separatedBy: "\n")
        
        var grid: [Row] = []
        for row in slopeRows {
            if row.count > 0 {
                grid.append(Row(row: row))
            }
        }
        
        self.grid = grid
    }
    
    func getField(_ v: Vector) -> Field? {
        if let row = self.getRow(v.y) {
            return row.getField(v.x)
        }
        
        return nil
    }
    
    func getRow(_ y: Int) -> Row? {
        if y > self.grid.count - 1 {
            return nil
        }
        
        return self.grid[y]
    }
}

struct Row {
    let fields: [Field]
    
    init(row: String) {
        var fields: [Field] = []
        for char in Array(row) {
            if String(char) == "." {
                fields.append(Field.open)
            } else if String(char) == "#" {
                fields.append(Field.tree)
            }
        }
        self.fields = fields
    }
    
    func getField(_ x: Int) -> Field? {
        if x > self.fields.count - 1 {
            return self.getField(x - self.fields.count)
        }
        
        return self.fields[x]
    }
}

enum Field {
    case open, tree
}
