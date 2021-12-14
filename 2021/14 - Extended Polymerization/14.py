import re

template = "HBCHSNFFVOBNOFHFOBNO"
# template = "NNCB"
_rules = {}


def template_to_rule_count(word: str, rules: dict[str, str]) -> dict[str, str]:
    matches = {}
    for i in range(len(word) - 1):
        rule = word[i] + word[i + 1]
        if rule not in matches:
            matches[rule] = 0

        matches[rule] += 1

    return matches


def iterate(rule_counts: dict[str, int], rules: dict[str, str]) -> dict[str, int]:
    ret_counts = {}
    for rule in rule_counts:
        for match in rules[rule]:
            if match not in ret_counts:
                ret_counts[match] = 0
            ret_counts[match] += rule_counts[rule]

    return ret_counts


def count_chars(rule_counts: dict[str, int], d: str) -> dict[str, int]:
    start = d[0]
    end = d[-1]

    ret = {
        start: 0.5,
        end: 0.5
    }

    for rule in rule_counts:
        char1 = rule[0]
        char2 = rule[1]

        if char1 not in ret:
            ret[char1] = 0
        ret[char1] += rule_counts[rule] / 2

        if char2 not in ret:
            ret[char2] = 0
        ret[char2] += rule_counts[rule] / 2

    return ret


with open('input') as f:
    for line in f.read().strip().split("\n"):
        pair, insertion = line.strip().split(" -> ")
        # _rules[pair] = pair[0] + insertion.lower() + pair[1]

        match1 = pair[0] + insertion
        match2 = insertion + pair[1]

        _rules[pair] = [match1, match2]

rule_count = template_to_rule_count(template, _rules)
iterated = rule_count
for j in range(40):
    iterated = iterate(iterated, _rules)

    if j == 9:
        counts = count_chars(iterated, template)
        print("part1", max(counts.values()) - min(counts.values()))


counts = count_chars(iterated, template)
print("part2", max(counts.values()) - min(counts.values()))