class NextGenerator:
    def __init__(self, n: int):
        self.n = n
        self.bag = {}
        for i in range(1, n + 1):
            self.bag[i] = i
        self.max = n

    def generate(self, ng: int):
        for k in reversed(range(1, self.max + 1)):
            if k not in self.bag:
                continue
            if k == ng:
                continue
            self.bag[k] -= 1
            if self.bag[k] == 0:
                self.bag.pop(k)
            while self.max not in self.bag and self.max > 1:
                self.max -= 1
            return k


def solve():
    n = int(input().rstrip())
    m = (n * (n + 1)) // 2
    a = ["" for _ in range(m)]

    b = m // 2
    i = b

    generator = NextGenerator(n)
    for t in range(1, m + 1):
        ng = a[i-1] if t % 2 == 1 else a[i+1]
        a[i] = generator.generate(ng)
        if t % 2 == 1:
            i = b - ((t + 1) // 2)
        else:
            i = b + (t // 2)

    print(" ".join(map(str, a)))


if __name__ == "__main__":
    solve()
