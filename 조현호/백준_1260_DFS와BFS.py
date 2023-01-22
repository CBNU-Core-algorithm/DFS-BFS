import sys
from collections import deque

# sys.stdin = open('test.txt', 'rt') # 테스트용 코드
input = sys.stdin.readline

n, m, v = map(int, input().strip().split())

# 정점(노드)의 수만큼 빈 리스트 딕셔너리 생성
node_dict = {i: [] for i in range(1, n+1)}

# 간선을 통하는 하는 양방향 리스트 만들기
for i in range(m):
    a, b = map(int, input().strip().split())
    node_dict[a].append(b)
    node_dict[b].append(a)

# 정점 번호가 작은 것 부터 방문하기 위해 정렬을 해 주자.
for x in node_dict.values():
    x.sort()

# 함수 내에서 사용할 글로벌 리스트
DFS_list = []
BFS_list = []

# DFS,BFS를 위한 방문한 리스트 만들기
visited_1 = [False] * (n + 1)
visited_2 = [False] * (n + 1)

def DFS(start, visited):
    # 현재 방문한 노드 추가
    DFS_list.append(start)

    # 방문한 노드 체크
    visited[start] = True

    # 노드 순서대로 방문하기
    for x in node_dict[start]:
        if not visited[x]:
            DFS(x, visited)


def BFS(start, visited):
    queue = deque([start])
    # 방문한 노드 체크
    visited[start] = True

    while queue:
        # popleft를 통해 먼저 방문한 노드의 다음 수 중 작은 수 먼저 빼기
        current = queue.popleft()
        # BFS리스트에 방문한 노드 추가
        BFS_list.append(current)

        # 방문하지 않은 원소들의 인접원소들 삽입
        for x in node_dict[current]:
            if not(visited[x]):
                # print(x)
                queue.append(x)
                visited[x] = True

# print(node_dict)
DFS(v, visited_1)
BFS(v, visited_2)
print(' '.join(map(str, DFS_list)))
print(' '.join(map(str, BFS_list)))




