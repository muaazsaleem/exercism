# An [Armstrong number][armstrong-number] is a number that is the sum of its own digits each raised to the power of the number of digits.
from math import log10, floor

def is_armstrong_number(number):
    ds = digits(number)
    c = len(ds)
    s = sum(d ** c for d in ds)
    return number == s

def digits(number):
    ds = []
    while number > 0:
        ds.append(number % 10)
        number //= 10
    return ds