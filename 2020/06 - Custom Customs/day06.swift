
import Foundation

let data = try Data(contentsOf: URL(fileURLWithPath: "input"))
let string = String(decoding: data, as: UTF8.self)

let groupedAnswers = string.components(separatedBy: "\n\n")

var sumUniqueAnswers = 0
var sumCommonAnswers = 0
for groupedAnswer in groupedAnswers {
    let answers = Answers(groupedAnswer)
    
    sumUniqueAnswers += answers.getNumberUniqueAnswers()
    sumCommonAnswers += answers.getNumberOfCommonAnswers()
}

print("Total of unique answers in groups is \(sumUniqueAnswers)")
print("Total of common answers in groups is \(sumCommonAnswers)")

struct Answers {
    let answerString: String
    
    init(_ a: String) {
        self.answerString = a
    }
    
    func getNumberUniqueAnswers() -> Int {
        let allAnswers = Set(answerString.replacingOccurrences(of: "\n", with: ""))
        
        return allAnswers.count
    }
    
    func getNumberOfCommonAnswers() -> Int {
        let eachAnswers = self.answerString.components(separatedBy: "\n")
        var commonAnswers = Set(eachAnswers[0])
        
        for answers in eachAnswers {
            if answers.count < 1 {
                continue
            }
            
            commonAnswers = commonAnswers.intersection(Set(answers))
        }
        
        print(self.answerString)
        print(commonAnswers)
        
        return commonAnswers.count
    }
}


