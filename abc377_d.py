from atcoder.segtree import SegTree


MAXLEN = 2 * 10**5


def parse():
  l, r = list(map(int, input().rstrip().split()))
  return [l - 1, r - 1]


def main():
  n, m = map(int, input().rstrip().split())
  regions = [parse() for _ in range(n)]
  regions.sort()

  segment_tree = SegTree(min, m, MAXLEN)
  used = set()
  for l, r in regions:
    if l in used:
      continue
    # print(f"{l=}, {r=}")
    segment_tree.set(l, r)
    used.add(l)

  ans = 0
  for k in range(m):
    # print(f"{k=}, {segment_tree.prod(k, MAXLEN)}")
    ans += segment_tree.prod(k, MAXLEN) - k
  print(ans)

if __name__ == "__main__":
  main()
