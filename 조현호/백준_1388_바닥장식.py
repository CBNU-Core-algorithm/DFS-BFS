import sys

# sys.stdin = open('a.txt', 'rt') # 테스트용 코드
input = sys.stdin.readline

# 값 입력받기
n, m = map(int, input().strip().split())
arr = [[x for x in input().strip()] for _ in range(n)]

# print(arr)

# 방문처리 함수
def visite(x, i, j):
    if x == '-':
        while arr[i][j] == '-':
            visited[i][j] = True
            j += 1
            if j == m:
                break
    if x == '|':
        while arr[i][j] == '|':
            visited[i][j] = True
            i += 1
            if i == n:
                break

# 방문리스트 만들기
# 옅은복사 오류 조심하자. 밑에처럼 리스트 컴프리헨션 이용해야 오류가 안 난다.
visited = [[False] * m for _ in range(n)]
count = 0

for i in range(n):
    for j in range(m):
        if visited[i][j] == False:
            count += 1
            visite(arr[i][j], i, j)


print(count)
