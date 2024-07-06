import sys
sys.setrecursionlimit(10 ** 9)


def strongly_connected_components(graph: list[list[int]]) -> list[int]:
    size = len(graph)
    seen = [False] * size
    stop = []

    def dfs(graph, stop, seen, u: int):
        seen[u] = True
        for v in graph[u]:
            if not seen[v]:
                dfs(graph, stop, seen, v)
        stop.append(u)

    for i in range(size):
        if not seen[i]:
            dfs(graph, stop, seen, i)

    graph_inv = [[] for _ in range(size)]
    for i in range(size):
        for u in graph[i]:
            graph_inv[u].append(i)
    stop_inv = []
    seen = [False] * size

    ret = [0] * size
    cnt = 0

    for u in stop[::-1]:
        if not seen[u]:
            dfs(graph_inv, stop_inv, seen, u)

            for v in stop_inv:
                ret[v] = cnt

            stop_inv = []
            cnt += 1

    return ret
