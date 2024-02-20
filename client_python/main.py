import requests

CLIENT_NAME = "client-pypy3-0.0.1"
URL = "http://127.0.0.1:5555/hello"


def get_task() -> tuple[int, int]:
    json = {"type": "Ready", "client_name": CLIENT_NAME}
    response = requests.post(URL, json=json)
    task = response.json()
    print("got task", task)
    from_ = task["Solve"]["from"]
    to = task["Solve"]["to"]
    return from_, to


def solve(n: int) -> int:
    biggest = n
    while n > 1:
        if n % 2 == 0:
            n //= 2
        else:
            n = 3 * n + 1
            biggest = max(biggest, n)
    return biggest


def solve_task(from_: int, to: int) -> int:
    biggest = 0
    for n in range(from_, to + 1):
        result = solve(n)
        biggest = max(result, biggest)
    return biggest


def report_solution(from_: str, to: str, solution: str):
    json = {
        "type": "Solved",
        "client_name": CLIENT_NAME,
        "from": from_,
        "to": to,
        "result": solution,
    }
    response = requests.post(URL, json=json)
    task = response.text
    print("got response", task)


def main():
    while True:
        from_, to = get_task()
        solution = solve_task(from_, to)
        report_solution(str(from_), str(to), str(solution))


if __name__ == "__main__":
    main()
