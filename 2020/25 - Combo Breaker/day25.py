from math import sqrt

cardPubKey = 6930903
doorPubKey = 19716708

if 1 == 0:
    cardPubKey = 5764801
    doorPubKey = 17807724
cardLoopSize = -1
doorLoopSize = -1
loopSize = 1
pubKey = 1

def handshake(subject, loopSize):
    chunk = int(sqrt(loopSize))
    divisor = 20201227
    value = 1

    while loopSize > chunk:
        value *= (subject ** chunk) % divisor
        loopSize -= chunk

    return value * (subject ** loopSize) % divisor

while cardLoopSize < 0 or doorLoopSize < 0:
    pubKey = (pubKey * 7) % 20201227

    if pubKey == cardPubKey:
        cardLoopSize = loopSize
        print "Card loop size is ", loopSize

    if pubKey == doorPubKey:
        doorLoopSize = loopSize
        print "Door loop size is ", loopSize

    loopSize += 1

encryptionKey = handshake(doorPubKey, cardLoopSize)
encryptionKey2 = handshake(cardPubKey, doorLoopSize)

print 'Encryption key is ', encryptionKey
print 'Encryption key2 is ', encryptionKey