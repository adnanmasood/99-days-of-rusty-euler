# names_scores.py
# Python reference solution for Project Euler 22. Final answer redacted.

import os
import string

def alpha_value(name: str) -> int:
    return sum((ord(c.upper()) - ord('A') + 1) for c in name if c.isalpha())

def parse_names(raw: str) -> list[str]:
    return [part.strip().strip('"') for part in raw.split(',') if part.strip().strip('"')]

def main() -> None:
    if os.path.exists('names.txt'):
        raw = open('names.txt', 'r', encoding='utf-8').read()
    else:
        raw = '"MARY","PATRICIA","LINDA","BARBARA","COLIN"'  # tiny fallback

    names = parse_names(raw)
    names.sort()
    total = sum((i+1) * alpha_value(n) for i, n in enumerate(names))
    # print(total)  # TODO: reveal locally
    print(f"Total of name scores = [redacted]; computed {len(names)} names")

if __name__ == '__main__':
    main()
