from pathlib import Path
from collections import deque
from os import listdir, path
import sys


# 各モジュールが依存しているモジュール名を列挙した隣接リスト表現のグラフ
dependencies_graph: dict[str, list[str]] = {
    "bellman_ford_algorithm": [],
    "binary_indexed_tree": [],
    "binary_indexed_tree_2d": [],
    "combination": ["modint"],
    "coordinate_compression": [],
    "cumulative_sum": [],
    "dijkstras_algorithm": ["graph"],
    "dual_segment_tree": ["monoid"],
    "dynamic_binary_indexed_tree": [],
    "dynamic_binary_indexed_tree_2d": [],
    "floyd_warshall_algorithm": [],
    "graph": [],
    "heavy_light_decomposition": ["graph"],
    "lazy_segment_tree": ["monoid"],
    "lowest_common_ancestor": [],
    "low_link": [],
    "manachers_algorithm": [],
    "miller_rabin_primality_test": [],
    "modint": [],
    "monoid": [],
    "offline_lca": [],
    "potentialized_union_find": [],
    "restore_shortest_path": ["graph"],
    "rolling_hash": [],
    "scc": ["graph"],
    "segment_tree": ["monoid"],
    "sieve_of_eratosthenes": [],
    "sparse_table": [],
    "static_rectangle_sum": [],
    "topological_sort": ["graph"],
    "union_find": [],
    "z_algorithm": [],
    "wavelet_matrix": [],
    "gcd": [],
}

# モジュール名の集合
module_name_set = set(dependencies_graph.keys())


def enumerate_all_modules() -> list[str]:
    """
    main, lib以外の存在するモジュールを列挙する
    """
    src_folder = Path('./src')
    modules = listdir(src_folder)
    modules.remove('lib.rs')
    modules.remove('main.rs')
    modules = [m[:-3] for m in modules]
    return modules


def parse_args() -> list[str]:
    args = sys.argv[1:]

    # 実際に存在するモジュール名かどうかを調べておく
    for module_name in args:
        assert module_name in module_name_set

    return args


def enumerate_dependencies(append_modules: list[str]) -> list[str]:
    # dependencies_graph 上で探索して、必須なモジュールを列挙する
    seen = set()
    q = deque()

    need_modules = []

    for m in append_modules:
        q.append(m)
        seen.add(m)
        need_modules.append(m)

    while q:
        current_module = q.pop()

        for nxt_module in dependencies_graph[current_module]:
            if nxt_module in seen:
                continue

            q.append(nxt_module)
            seen.add(nxt_module)
            need_modules.append(nxt_module)

    return need_modules


def expand(modules: list[str]):
    """
    展開したいモジュール名の一覧を受け取って、main.rsにそれらを展開する。
    """

    modules = set(modules)
    expanded_modules = set()

    # すでに展開されているモジュールは追加してはいけないので、先に検出しておく
    with open(Path('src\main.rs'), mode='r', encoding='utf-8') as f:
        content = f.read().splitlines()

        for row in content:
            for m in modules:
                if row.startswith(f'pub mod {m}'):
                    expanded_modules.add(m)

    for m in expanded_modules:
        modules.remove(m)

    modules = list(modules)

    with open(Path('src\main.rs'), mode='a', encoding='utf-8') as f:
        for module in modules:
            module_path = Path(f'src\{module}.rs')
            assert path.isfile(module_path)

            with open(module_path, mode='r', encoding='utf-8') as fm:
                content = fm.read().splitlines()

            f.write('\n')  # 一行空けたほうが見やすい気がする
            f.write(f'pub mod {module} {{\n')

            for row in content:
                f.write(row)
                f.write('\n')

            f.write('}\n')

    return


def main():
    all_modules = sorted(enumerate_all_modules())
    assert all_modules == sorted(dependencies_graph.keys())

    append_modules = parse_args()
    need_modules = enumerate_dependencies(append_modules)

    expand(need_modules)


if __name__ == '__main__':
    main()
