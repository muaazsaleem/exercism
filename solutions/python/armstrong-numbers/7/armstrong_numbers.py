"""Utilities for identifying Armstrong numbers."""


def is_armstrong_number(number):
    """Return True if number is an Armstrong number, False otherwise."""
    digits = to_digits(number)
    count = len(digits)
    digits_sum = sum(digit ** count for digit in digits)
    return number == digits_sum


def to_digits(number):
    """Return the decimal digits of number as a list of ints."""
    digits = []
    while number > 0:
        digits.append(number % 10)
        number //= 10
    return digits