from heapq import heappush, heappop
from typing import Union


def dijkstras_algorithm(graph: list[list[(int, int)]], src: int) -> list[Union[int, None]]:
    size = len(graph)
    dist = [None] * size
    dist[src] = 0
    seen = [False] * size
    count = 0

    hq = []
    heappush(hq, (0, src))

    while hq:
        _, now = heappop(hq)
        if seen[now]:
            continue
        seen[now] = True
        count += 1

        for (nxt, weight) in graph[now]:
            if seen[nxt]:
                continue

            d = dist[now] + weight

            if dist[nxt] is None or dist[nxt] > d:
                dist[nxt] = d
                heappush(hq, (dist[nxt], nxt))

        if count == size:
            break

    return dist
