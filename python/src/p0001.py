# Problem #1: Multiples of 3 or 5
# https://projecteuler.net/problem=1
#
# If we list all the natural numbers below 10 that are multiples of 3 or 5, we
# get 3, 5, 6 and 9. The sum of these multiples is 23.
#
# Find the sum of all the multiples of 3 or 5 below 1000.

LIMIT = 1000

def simple():
    sum = 0

    for n in range(LIMIT):
        if (n % 5 == 0 or n % 3 == 0):
            sum += n

    return sum


def short():
    return sum(n for n in range(LIMIT) if (n % 5 == 0 or n % 3 == 0))


if __name__ == "__main__":
    print("simple: " + str(simple()))
    print("short: " + str(short()))

