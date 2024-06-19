class BinaryIndexedTree:
    def __init__(self, size: int):
        self.tree = [0] * (size + 1)
        self.size = size + 1

    def add(self, i: int, v: int):
        self._add(i + 1, v)

    def prefix_sum(self, i: int) -> int:
        return self._sum(i + 1)

    def sum(self, left: int, right: int) -> int:
        if left == 0:
            return self.prefix_sum(right)
        else:
            return self.prefix_sum(right) - self.prefix_sum(left - 1)

    def _add(self, i: int, v: int):
        while i < self.size:
            self.tree[i] += v
            i += i & -i

    def _sum(self, i: int) -> int:
        ret = 0
        while i > 0:
            ret += self.tree[i]
            i -= i & -i
        return ret
