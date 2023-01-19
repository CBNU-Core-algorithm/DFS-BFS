def dfs(graph, start, visited): 
    visited[start] = True # current 방문
    print(start, end=" ")
    graph[start].sort()
    for nxt in graph[start]: # current의 인접 노드를 확인한다. 이 노드를 nxt라고 할 것임.
      if not visited[nxt]: # 만일 nxt에 방문하지 않았다면
        dfs(graph, nxt, visited) #nxt 방문


def bfs(graph, start, visited): 
    queue = [start]
    visited[start] = True
    while len(queue) > 0: # 큐가 빌 때까지 반복
        current = queue.pop(0) #queue에서 노드를 하나 빼 온다. 이 노드를 current
        print(current, end=" ")
        graph[current].sort()
        for nxt in graph[current]: # current의 인접 노드들을 확인한다. 이 각각의 노드를 nxt
            if not visited[nxt]: # 만일 nxt에 방문하지 않았다면 nxt 방문
                queue.append(nxt)
                visited[nxt] = True



node, line, start=map(int,input().split())
graph=[[] for i in range(node+1)]
for i in range(line):
  a,b=map(int, input().split())
  graph[a].append(b)
  graph[b].append(a)
visited = {i:False for i in range(node+1)} # 방문 배열. visited[node] = True이면 node는 방문이 끝난 상태이다.
dfs(graph, start, visited)
print()
visited = {i:False for i in range(node+1)} # 방문 배열. visited[node] = True이면 node는 방문이 끝난 상태이다.
bfs(graph, start, visited)
