def is_armstrong_number(number):
    digits = to_digits(number)
    count = len(digits)
    digits_sum = sum(d ** count for d in digits)
    return number == digits_sum

def to_digits(number):
    digits = []
    while number > 0:
        digits.append(number % 10)
        number //= 10
    return digits