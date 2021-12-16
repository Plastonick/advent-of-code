import math
from collections import deque

hex = "620D7800996600E43184312CC01A88913E1E180310FA324649CD5B9DA6BFD107003A4FDE9C718593003A5978C00A7003C400A70025400D60259D400B3002880792201B89400E601694804F1201119400C600C144008100340013440021279A5801AE93CA84C10CF3D100875401374F67F6119CA46769D8664E76FC9E4C01597748704011E4D54D7C0179B0A96431003A48ECC015C0068670FA7EF1BC5166CE440239EFC226F228129E8C1D6633596716E7D4840129C4C8CA8017FCFB943699B794210CAC23A612012EB40151006E2D4678A4200EC548CF12E4FDE9BD4A5227C600F80021D08219C1A00043A27C558AA200F4788C91A1002C893AB24F722C129BDF5121FA8011335868F1802AE82537709999796A7176254A72F8E9B9005BD600A4FD372109FA6E42D1725EDDFB64FFBD5B8D1802323DC7E0D1600B4BCDF6649252B0974AE48D4C0159392DE0034B356D626A130E44015BD80213183A93F609A7628537EB87980292A0D800F94B66546896CCA8D440109F80233ABB3ABF3CB84026B5802C00084C168291080010C87B16227CB6E454401946802735CA144BA74CFF71ADDC080282C00546722A1391549318201233003361006A1E419866200DC758330525A0C86009CC6E7F2BA00A4E7EF7AD6E873F7BD6B741300578021B94309ABE374CF7AE7327220154C3C4BD395C7E3EB756A72AC10665C08C010D0046458E72C9B372EAB280372DFE1BCA3ECC1690046513E5D5E79C235498B9002BD132451A5C78401B99AFDFE7C9A770D8A0094EDAC65031C0178AB3D8EEF8E729F2C200D26579BEDF277400A9C8FE43D3030E010C6C9A078853A431C0C0169A5CB00400010F8C9052098002191022143D30047C011100763DC71824200D4368391CA651CC0219C51974892338D0"
# hex = "8A004A801A8002F478"  # 16
# hex = "620080001611562C8802118E34"  # 12
# hex = "C0015000016115A2E0802F182340"  # 23
# hex = "A0016C880162017C3686B18A3D4780"  # 31


def parse_meta(packet_meta: str):
    # if len(packet_meta) < 6:
    #     return 0

    packet_version_bin = packet_meta[:3]
    version = int(packet_version_bin, 2)

    type = int(packet_meta[3:6], 2)

    return version, type


original = bin(int(hex, 16))[2:].zfill(8)
binary_str = original.zfill((math.floor(len(original) / 4) + 1) * 4)
# binary_str = '00111000000000000110111101000101001010010001001000000000'

version_total = 0
i = 0
operations = {
    0: "+",
    1: "*",
    2: "min",
    3: "max",
    5: ">",
    6: "<",
    7: "==",
}

depth = 0
packets_stack = deque()
num_left = 0
num_right = 0

while i < len(binary_str) - 6:
    version, type = parse_meta(binary_str[i:i + 6])
    version_total += version

    if len(packets_stack) > 0:
        packet_data = packets_stack.pop()

        if packet_data[0] is None:
            if i >= packet_data[1]:
                # print(")")
                num_right += 1
            else:
                packets_stack.append(packet_data)
        else:
            if packet_data[0] - 1 == 0:
                # print(")")
                num_right += 1
            else:
                packets_stack.append((packet_data[0] - 1, None))

    # move past the meta
    i = i + 6
    if type == 4:
        value = ""
        # keep going until the last group
        while binary_str[i] == "1":
            # optionally read the packet
            value += binary_str[i + 1: i + 5]
            i = i + 5

        value += binary_str[i + 1: i + 5]

        # print(int(value, 2))
        # finished reading the packet, increment i
        i = i + 5
    else:
        # print("(")
        num_left += 1
        depth += 1

        # print(operations[type])

        length_type = int(binary_str[i], 2)
        i = i + 1
        if length_type == 0:
            total_length = int(binary_str[i:i + 15], 2)
            # don't care enough
            packets_stack.append((None, i + total_length))
            i = i + 15
        else:
            num_packets = int(binary_str[i:i + 11], 2)
            packets_stack.append((num_packets, None))
            i = i + 11

    # a = 1

# for _ in range(num_left - num_right):
#     # print(")")

print(version_total)
