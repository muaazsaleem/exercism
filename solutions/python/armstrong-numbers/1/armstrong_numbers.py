# An [Armstrong number][armstrong-number] is a number that is the sum of its own digits each raised to the power of the number of digits.

def is_armstrong_number(number):
    if number <= 9:
        return True
    digits = [int(d) for d in list(str(number))]
    c = len(digits)
    s = sum(d ** c for d in digits)
    if number == s:
        return True
    return False


