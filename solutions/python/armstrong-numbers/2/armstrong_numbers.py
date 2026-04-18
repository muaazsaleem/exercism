# An [Armstrong number][armstrong-number] is a number that is the sum of its own digits each raised to the power of the number of digits.

def is_armstrong_number(number):
    if number <= 9:
        return True
    n = str(number)
    c = len(n)
    s = sum(int(d) ** c for d in list(n))
    return number == s

