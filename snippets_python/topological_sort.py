def topological_sort(graph: list[list[int]]) -> list[int]:
    from collections import deque
    size = len(graph)
    out_degs = [0] * size
    order = []

    graph_inv = [[] for _ in range(size)]
    q = deque()

    for i in range(size):
        out_degs[i] = len(graph[i])

        if out_degs[i] == 0:
            q.append(i)

        for nxt in graph[i]:
            graph_inv[nxt].append(i)

    while q:
        u = q.popleft()
        order.append(u)

        for nxt in graph_inv[u]:
            out_degs[nxt] -= 1
            if out_degs[nxt] == 0:
                q.append(nxt)

    order.reverse()
    return order
