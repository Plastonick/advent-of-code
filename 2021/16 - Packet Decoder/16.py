import math
import sys

sys.setrecursionlimit(1000000)

hex = "620D7800996600E43184312CC01A88913E1E180310FA324649CD5B9DA6BFD107003A4FDE9C718593003A5978C00A7003C400A70025400D60259D400B3002880792201B89400E601694804F1201119400C600C144008100340013440021279A5801AE93CA84C10CF3D100875401374F67F6119CA46769D8664E76FC9E4C01597748704011E4D54D7C0179B0A96431003A48ECC015C0068670FA7EF1BC5166CE440239EFC226F228129E8C1D6633596716E7D4840129C4C8CA8017FCFB943699B794210CAC23A612012EB40151006E2D4678A4200EC548CF12E4FDE9BD4A5227C600F80021D08219C1A00043A27C558AA200F4788C91A1002C893AB24F722C129BDF5121FA8011335868F1802AE82537709999796A7176254A72F8E9B9005BD600A4FD372109FA6E42D1725EDDFB64FFBD5B8D1802323DC7E0D1600B4BCDF6649252B0974AE48D4C0159392DE0034B356D626A130E44015BD80213183A93F609A7628537EB87980292A0D800F94B66546896CCA8D440109F80233ABB3ABF3CB84026B5802C00084C168291080010C87B16227CB6E454401946802735CA144BA74CFF71ADDC080282C00546722A1391549318201233003361006A1E419866200DC758330525A0C86009CC6E7F2BA00A4E7EF7AD6E873F7BD6B741300578021B94309ABE374CF7AE7327220154C3C4BD395C7E3EB756A72AC10665C08C010D0046458E72C9B372EAB280372DFE1BCA3ECC1690046513E5D5E79C235498B9002BD132451A5C78401B99AFDFE7C9A770D8A0094EDAC65031C0178AB3D8EEF8E729F2C200D26579BEDF277400A9C8FE43D3030E010C6C9A078853A431C0C0169A5CB00400010F8C9052098002191022143D30047C011100763DC71824200D4368391CA651CC0219C51974892338D0"


# hex = "8A004A801A8002F478"  # 16
# hex = "620080001611562C8802118E34"  # 12
# hex = "C0015000016115A2E0802F182340"  # 23
# hex = "A0016C880162017C3686B18A3D4780"  # 31


class Packet:
    def __init__(self, version: int, type: int, value=None):
        self.children: list[Packet] = []
        self.version = version
        self.type = type
        self.value = value

    def sum_versions(self) -> int:
        total = self.version

        for child in self.children:
            total += child.sum_versions()

        return total

    def get_value(self):
        if self.type == 4:
            return self.value
        else:
            # add,
            if self.type == 0:
                return sum([c.get_value() for c in self.children])
            # multiply,
            elif self.type == 1:
                total = 1
                for child in self.children:
                    total *= child.get_value()
                return total
            # take minimum,
            elif self.type == 2:
                return min([c.get_value() for c in self.children])
            # take maximum,
            elif self.type == 3:
                return max([c.get_value() for c in self.children])
            # greater than,
            elif self.type == 5:
                return 1 if self.children[0].get_value() > self.children[1].get_value() else 0
            # less than,
            elif self.type == 6:
                return 1 if self.children[0].get_value() < self.children[1].get_value() else 0
            # equal to,
            else:
                return 1 if self.children[0].get_value() == self.children[1].get_value() else 0

    def add_child(self, child):
        self.children.append(child)


def parse_meta(packet_meta: str):
    if len(packet_meta) < 6:
        return 0

    packet_version_bin = packet_meta[:3]
    version = int(packet_version_bin, 2)

    return version, int(packet_meta[3:6], 2)


def bin_to_packet(binary: str) -> tuple[Packet, int]:
    v, t = parse_meta(binary[:6])
    i = 6

    if t == 4:
        value = ""
        # keep going until the last group
        while binary[i] == "1":
            # optionally read the packet
            value += binary[i + 1: i + 5]
            i += 5

        value += binary[i + 1: i + 5]
        i += 5

        return Packet(version=v, type=t, value=int(value, 2)), i
    else:
        packet = Packet(version=v, type=t)

        # certain number of children to consider
        length_type = int(binary[i], 2)
        i += 1
        if length_type == 0:
            # total number of bits that the children take
            total_length = int(binary[i:i + 15], 2)
            i += 15
            end_length = i + total_length

            while i < end_length:
                child, incr = bin_to_packet(binary[i:end_length])
                i += incr
                packet.add_child(child)
        else:
            num_packets = int(binary[i:i + 11], 2)
            i += 11

            for _ in range(num_packets):
                child, incr = bin_to_packet(binary[i:])
                i += incr
                packet.add_child(child)

    return packet, i


original = bin(int(hex, 16))[2:].zfill(8)

expected_size = math.ceil(len(original) / 4) * 4
binary_str = original.zfill(expected_size)

big_packet, _ = bin_to_packet(binary_str)

print("part1", big_packet.sum_versions())
print("part2", big_packet.get_value())
