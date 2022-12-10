import sys

try:
    with open("test.txt") as fp:
        buf = fp.read()
        print("read size: {}".format(len(buf)))
except Exception:
    print("Failed to open or read the file", file=sys.stderr)
    sys.exit(1)