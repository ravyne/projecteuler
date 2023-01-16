# Problem #6: Sum square difference
# https://projecteuler.net/problem=6
#
# The sum of the squares of the first ten natural numbers is:
#
#   (1^2 + 2^2 + ... + 10^2) = 385
#
# The square of the sum of the first ten natural numbers is:
#
#   (1 + 2 + ... + 10)^2 = 55^2 = 3025
#
# Hence the difference between the sum of the squares of the first ten natural
# numbers and the square of the sum is: 3025 - 385 = 2640
#
# Find the difference between the sum of the squares of the first one hundred
# natural numbers and the square of the sum.


from functools import reduce

LIMIT = 100


def simple():
    numbers = [n + 1 for n in range(LIMIT)]
    sum_of_squares = reduce(lambda s, n: s+n, map(lambda n: n*n, numbers))
    square_of_sums = pow(reduce(lambda s, n: s+n, numbers), 2)

    return square_of_sums - sum_of_squares


def short():
    numbers = range(1, LIMIT + 1)
    sum_of_squares = sum(map(lambda n: n*n, numbers))
    square_of_sums = pow(sum(numbers), 2)

    return square_of_sums - sum_of_squares


def direct():
    n = LIMIT
    sum_n = n * (n+1) / 2
    sum_of_squares = n * (n+1) * (2 * n + 1) / 6
    square_of_sums = sum_n * sum_n

    return square_of_sums - sum_of_squares


# square of sums:                      | sum of squares:
# n^2 + n   n^2 + n                    |
# n*(n+1)   n*(n+1)   n^4 + 2n^3 + n^2 | (n^2 + n) * (2n + 1)   2n^3 + 3n^2 + n
# ------- x ------- = ---------------- | -------------------- = ---------------
#   2         2         4              |   6                      6
#
# square of sums - sum of squares:
# 3n^4 + 6n^3 + 3n^2   4n^3 + 6n^2 + 2n   3n^4 + 6n^3 - 4n^3 + 3n^2 - 6n^2 - 2n
# ------------------ - ---------------- = -------------------------------------
#   12                   12                 12
#
#     3n^4 + 2n^3 - 3n^2 - 2n   n(3n+2)(n+1)(n-1)
#   = ----------------------- = -----------------
#       12                        12
def optimal():
    n = LIMIT

    return n * (3*n+2) * (n+1) * (n-1) / 12


if __name__ == "__main__":
    print("simple: " + str(simple()))
    print("short: " + str(short()))
    print("direct: " + str(direct()))
    print("optimal: " + str(optimal()))
