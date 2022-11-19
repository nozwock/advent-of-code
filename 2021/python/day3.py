#!/usr/bin/env python
import sys

bit_count = [{}]

# https://adventofcode.com/2021/day/3

for line in sys.stdin:
    for i, c in enumerate(line.strip()):
        try:
            bit_count[i][c] = bit_count[i].get(c, 0) + 1
        except IndexError:
            bit_count.insert(i, {c: 1})

gamma_rate = ""
epsilon_rate = ""
for map in bit_count:
    count_max = max(map.values())
    for k, v in map.items():
        if v == count_max:
            gamma_rate += k
            continue
        epsilon_rate += k


print(f"gamma binary = {gamma_rate}\nepsilon binary = {epsilon_rate}")

gamma_rate = int(gamma_rate, 2)
epsilon_rate = int(epsilon_rate, 2)

print(f"gamma = {gamma_rate}\nepsilon = {epsilon_rate}")

print(f"power consumption = {gamma_rate *  epsilon_rate}")
