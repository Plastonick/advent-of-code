
import Foundation

let debug = false
var player1Cards = [31, 33, 27, 43, 29, 25, 36, 11, 15, 5, 14, 34, 7, 18, 26, 41, 19, 45, 12, 1, 8, 35, 44, 30, 50]
var player2Cards = [42, 40, 6, 17, 3, 16, 22, 23, 32, 21, 24, 46, 49, 48, 38, 47, 13, 9, 39, 20, 10, 2, 37, 28, 4]

let game = RecursiveCombatGame(player1Cards, player2Cards)

let result = game.play()
print(result)

print(calculateScore(cards: result.deck))

func calculateScore(cards: [Int]) -> Int {
    var score = 0

    for (i, card) in cards.reversed().enumerated() {
        score += (i + 1) * card
    }

    return score
}

class RecursiveCombatGame {
    var player1Cards: [Int]
    var player2Cards: [Int]
    var states: Set<[Int]> = []

    init(_ a: [Int], _ b: [Int]) {
        player1Cards = a
        player2Cards = b
    }

    public func play() -> Result {
        while player1Cards.count > 0, player2Cards.count > 0 {
            let state = player1Cards + [0] + player2Cards
            if states.contains(state) {
                return Result(winner: 1, deck: player1Cards)
            }

            states.insert(state)

            let player1 = player1Cards.removeFirst()
            let player2 = player2Cards.removeFirst()

            if player1 <= player1Cards.count, player2 <= player2Cards.count {
                let player1SubDeck = Array(player1Cards[0 ..< player1])
                let player2SubDeck = Array(player2Cards[0 ..< player2])
                let subGame = RecursiveCombatGame(player1SubDeck, player2SubDeck)

                let subResult = subGame.play()

                if subResult.winner == 1 {
                    player1Cards.append(player1)
                    player1Cards.append(player2)
                } else {
                    player2Cards.append(player2)
                    player2Cards.append(player1)
                }
            } else {
                // we can't do a sub game... normal rules!
                if player1 > player2 {
                    player1Cards.append(player1)
                    player1Cards.append(player2)
                } else {
                    player2Cards.append(player2)
                    player2Cards.append(player1)
                }
            }
        }

        // player 2 has exhausted, player 1 wins!
        if player2Cards.count == 0 {
            return Result(winner: 1, deck: player1Cards)
        } else {
            return Result(winner: 2, deck: player2Cards)
        }
    }
}

struct Result {
    let winner: Int
    let deck: [Int]
}
