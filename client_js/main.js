const CLIENT_NAME = "client-js-0.0.1";

const URL = "http://127.0.0.1:5555/hello";

async function get_task() {
    const payload = { client_name: CLIENT_NAME, type: "Ready" };
    const response = await fetch(URL, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify(payload, null, 0),
    });
    const body = await response.json();
    console.log(`Got body = ${JSON.stringify(body, null, 2)}`);
    return body["Solve"];
}

function solve(n) {
    let result = n;
    while (n > 1) {
        if (n % 2 == 0n) {
            n /= 2;
        } else {
            n = 3 * n + 1;
            if (n > result) result = n;
        }
    }
    return result;
}

function solve_task({ from, to }) {
    let biggest = 0n;

    for (let n = from; n <= to; ++n) {
        const result = solve(n);
        if (result > biggest) biggest = result;
    }
    return { from, to, result: biggest.toString() };
}

async function report_task({ from, to, result }) {
    const payload = { client_name: CLIENT_NAME, type: "Solved", from: from.toString(), to: to.toString(), result };
    const response = await fetch(URL, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify(payload, null, 0),
    });
    const body = await response.json();
    console.log(`Got body = ${JSON.stringify(body, null, 2)}`);
    return body;
}


async function main() {
    while (true) {
        const task = await get_task();
        const solved_task = solve_task(task);
        console.log(`result from=${task.from} to=${task.to} is ${solved_task.result}`);
        await report_task(solved_task);
    }
}

main();