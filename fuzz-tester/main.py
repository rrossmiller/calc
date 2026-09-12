import argparse
import random
import subprocess
from enum import Enum, auto

from tqdm import trange


class Tokens(Enum):
    PLUS = auto()
    MINUS = auto()
    DIVIDE = auto()
    MULTIPLY = auto()
    EOF = auto()


tokens = list(Tokens)


def make_number() -> str:
    n = random.random() * 100
    if random.randint(0, 9) > 0 and n >= 1:
        n = int(n)
    else:
        n = round(n, 4)

    return str(n)


def generate_test() -> tuple[str, float]:
    is_op = False

    stmt = make_number()
    t = None
    while t != Tokens.EOF:
        # if the last token was an operator, the next must be a number)
        if is_op:
            if random.randint(0, 9) == 0:
                stmt += " ("
                stmt += generate_test()[0]
                stmt += ") "
            else:
                stmt += make_number()
            is_op = False
            continue

        t = random.choice(tokens)
        match t:
            case Tokens.PLUS:
                stmt += " + "
                is_op = True
            case Tokens.MINUS:
                stmt += " - "
                is_op = True
            case Tokens.DIVIDE:
                stmt += " / "
                is_op = True
            case Tokens.MULTIPLY:
                stmt += " * "
                is_op = True

    ans = eval(stmt)
    return (stmt, ans)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--num", "-n", help="number of test cases to generate", default=1000, type=int
    )
    args = parser.parse_args()

    tr = trange(args.num, desc="Running tests")
    for t in tr:
        try:
            t = generate_test()
        except:
            continue
        result = subprocess.run(
            [
                "calc",
                t[0],
            ],
            check=True,
            capture_output=True,
            text=True,
        )
        # check if the result is correct
        ans = float(result.stdout.strip())
        if round(ans, 5) != round(t[1], 5):
            print(f"Test failed: {t[0]} = {t[1]}, got {result.stdout.strip()}")
            break

        # print the test case and the result
        # print(f"Test case: {t[0]} = {t[1]}, got {result.stdout.strip()}")
        # print(f"Test case:  {t[1]} =? {result.stdout.strip()}")
        tr.write(f"Test case: {t[0]} = {t[1]} =? {result.stdout.strip()}")
