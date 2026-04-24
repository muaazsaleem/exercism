package armstrongnumbers

import (
    "strconv"
)

func IsNumber(n int) bool {
    s := strconv.Itoa(n)
    // s is ASCII only
    c := len(s)
    sum := 0
    for _, v := range s {
        // int(v - '0') works because a digit is always 0-9 and the ASCII values for that range are contigous
        d := int(v - '0')
        sum += intPow(d, c)
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
