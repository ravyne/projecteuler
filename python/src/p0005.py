# Problem #5: Smallest multiple
# https://projecteuler.net/problem=5
#
# 2520 is the smallest number that can be divided by each of the numbers from
# 1 to 10 without any remainder.
#
# What is the smallest positive number that is evenly divisible by all of the
# numbers from 1 to 20?
#
# 10 9 8 7 6 5 4 3 2 1
# 10 9 8 7 6 - - - - -
# 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1
# 20 19 18 17 16 15 14 13 12 11 -- - - - - - - - - -


from math import lcm
from functools import reduce

import numpy as np

LIMIT = 20


def simple():
    numbers = [n + 1 for n in range(LIMIT)]
    return reduce(lambda a, n: lcm(a, n), numbers)


def short():
    return reduce(lcm, range(1, LIMIT + 1))


def using_numpy():
    return np.lcm.reduce(range(1, LIMIT + 1))


if __name__ == "__main__":
    print("simple: " + str(simple()))
    print("short: " + str(short()))
    print("using_numpy: " + str(using_numpy()))
