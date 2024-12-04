import fs from 'node:fs'

async function getInput(): Promise<string> {
    return await fs.promises.readFile('input.txt', 'utf-8')
}

async function process_line(data: number[]): Promise<number> {
    if (data[0] == data[1]) {
        return 0;
    }
    const is_increasing = data[0] < data[1];

    for (let i = 1; i < data.length; i++) {
        if ((is_increasing && data[i - 1] >= data[i]) || (!is_increasing && data[i - 1] <= data[i])) {
            return 0;
        }

        if (Math.abs(data[i] - data[i - 1]) > 3) {
            return 0;
        }
    }
    return 1;
}

async function process_line_fault_tolerance(data: number[]): Promise<number> {
    const base_result = await process_line(data);
    if (base_result > 0) {
        return base_result;
    }

    for (let i = 0; i < data.length; i++) {
        let new_data: number[] = [];
        for (let j = 0; j < data.length; j++) {
            if (i != j) {
                new_data.push(data[j]);
            }
        }

        let new_result = await process_line(new_data);
        if (new_result > 0) {
            return new_result;
        }
    }

    return 0;
}

async function part_one(data: number[][]) {
    let total = 0;
    for (let d of data) {
        total += await process_line(d);
    }
    console.log("Part one: ", total);
}

async function part_two(data: number[][]) {
    let total = 0;
    for (let d of data) {
        total += await process_line_fault_tolerance(d);
    }
    console.log("Part two: ", total);
}

async function main() {
    const input = await getInput()
    let lines = input.split('\n')
    lines.pop(); // remove last empty line
    const data = lines.map((line) => line.split(' ').map((x) => parseInt(x)));
    console.log("Javascript solution:")
    await part_one(data);
    await part_two(data);
    console.log("---------------------------------------")
}

await main();
