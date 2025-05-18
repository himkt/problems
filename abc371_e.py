from collections import deque


def lazy(n: int, a: list[int]):
    ans = 0
    st = set()
    for i in range(n):
        st.add(a[i])
        ans += len(st)
    return ans


def solve():
    n = int(input().rstrip())
    a = list(map(int, input().rstrip().split()))
    a = [ai - 1 for ai in a]
    s0 = lazy(n, a)

    indices = [deque([]) for _ in range(n)]
    for i in range(n):
        indices[a[i]].append(i)

    ans = s0
    for i in range(1, n):
        prev = a[i-1]
        indices[prev].popleft()
        least = indices[prev][0] if len(indices[prev]) else n
        s0 -= 1
        s0 -= (least - i)
        ans += s0

    print(ans)


if __name__ == "__main__":
    solve()
