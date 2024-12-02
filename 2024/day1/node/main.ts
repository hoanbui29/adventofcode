import fs from 'node:fs';

function panic(err: Error) {
    console.error(err);
    process.exit(1);
}

async function getInput(): Promise<string> {
    try {
        const data = await fs.promises.readFile('input.txt', 'utf-8')
        return data
    } catch (error) {
        panic(error);
    }
    return ''; // Unreachable anyway, just to make the compiler happy
}

async function parseInput(input: string): Promise<[number[], number[]]> {
    const lines = input.split('\n')
    const left_ids: number[] = []
    const right_ids: number[] = []
    lines.forEach((line) => {
        if (line === '') {
            return
        }
        const parts = line.split('   ')
        left_ids.push(parseInt(parts[0]))
        right_ids.push(parseInt(parts[1]))
    })

    return [left_ids, right_ids]
}

async function part_one() {
    const input = await getInput();
    const [left_ids, right_ids] = await parseInput(input);
    left_ids.sort();
    right_ids.sort();

    let total = 0;

    for (let i = 0; i < left_ids.length; i++) {
        total += Math.abs(left_ids[i] - right_ids[i]);
    }

    console.log("Part one: ", total);
}

async function part_two() {
    const input = await getInput();
    const [left_ids, right_ids] = await parseInput(input);
    let right_map = new Map<number, number>();
    right_ids.forEach((id) => {
        let old = right_map.get(id) ?? 0;
        right_map.set(id, old + 1);
    })

    let total = 0;

    for (let i = 0; i < left_ids.length; i++) {
        let id = left_ids[i];
        let score = right_map.get(id) ?? 0;
        total += score * id
    }

    console.log("Part two: ", total);
}

async function main() {
    console.log('Node solution');
    await part_one();
    await part_two();
    console.log('------------------------------------------------------')
}

//Run
await main()
