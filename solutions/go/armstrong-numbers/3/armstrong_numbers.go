package armstrongnumbers

import (
    "math"
    "strconv"
)

func IsNumber(n int) bool {
    num := strconv.Itoa(n)
    // num is ASCII only
    digitCount := len(num)
    sum := 0
    for _, char := range num {
        // int(char - '0') works because a digit is always 0-9 and the ASCII values for that range are contigous
        digit := int(char - '0')
        sum += int(math.Round(math.Pow(float64(digit), float64(digitCount))))
    }
    
    return sum == n
}

func intPow(base, exp int) int {
    result := 1
    for range exp {
        result *= base
    }
    return result
}
