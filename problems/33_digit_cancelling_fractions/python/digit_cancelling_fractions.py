# digit_cancelling_fractions.py
# Project Euler 33 - Digit Cancelling Fractions (Python)

from math import gcd

def is_curious(n: int, d: int) -> bool:
    nt, nu = divmod(n, 10)
    dt, du = divmod(d, 10)
    if 0 in (nt, nu, dt, du):
        return False
    def check(kn: int, kd: int) -> bool:
        return kd != 0 and n * kd == d * kn
    if nu == dt and check(nt, du): return True
    if nt == du and check(nu, dt): return True
    if nt == dt and check(nu, du): return True
    if nu == du and check(nt, dt): return True
    return False

def main() -> None:
    num, den = 1, 1
    for n in range(10, 100):
        for d in range(n + 1, 100):
            if is_curious(n, d):
                num *= n
                den *= d
    g = gcd(num, den)
    ans = den // g
    # print(ans)  # TODO: reveal locally
    print("denominator (lowest terms) = <redacted>")

if __name__ == "__main__":
    main()
