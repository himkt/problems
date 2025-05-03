def parse():
    x, y, z = input().rstrip().split()
    return [int(x), int(y), z]

def main():
    _, m = map(int, input().rstrip().split())
    xyz = [parse() for _ in range(m)]
    xyz.sort()

    min_y = 1_000_000_000
    for _, y, z in xyz:
        match z:
            case 'W':
                min_y = min(min_y, y)
            case 'B':
                if min_y <= y:
                    print("No")
                    return
    print("Yes")

if __name__ == "__main__":
    main()

