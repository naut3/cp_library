class UnionFind:
    def __init__(self, size: int):
        self.data = [-1] * size

    def find(self, v: int) -> int:
        if self.data[v] < 0:
            return v

        self.data[v] = self.find(self.data[v])
        return self.data[v]

    def issame(self, u: int, v: int) -> bool:
        return self.find(u) == self.find(v)

    def unite(self, u: int, v: int):
        u = self.find(u)
        v = self.find(v)

        if u == v:
            return

        if self.data[u] > self.data[v]:
            (u, v) = (v, u)

        self.data[u] += self.data[v]
        self.data[v] = u

    def size(self, v: int) -> int:
        return -self.find(self.data[v])
