use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}

#[aoc(day2, part1)]
fn part1(in_vec: &[usize]) -> usize {
    let mut prog: Vec<usize> = in_vec.to_vec();
    prog[1] = 12;
    prog[2] = 2;

    exec_prog(&mut prog);

    prog[0]
}

#[aoc(day2, part2)]
fn part2(in_vec: &[usize]) -> usize {
    let k = 19690720;

    for i in 0..99 {
        for j in 0..99 {
            let mut prog = in_vec.to_vec();
            prog[1] = i;
            prog[2] = j;

            exec_prog(&mut prog);

            if prog[0] == k {
                return i * 100 + j;
            };
        }
    }
    unreachable!()
}

fn exec_prog(prog: &mut [usize]) {
    let mut index = 0;
    loop {
        let opcode = prog[index];
        if opcode == 99 {
            break;
        }
        let input0 = prog[index + 1];
        let input1 = prog[index + 2];
        let output = prog[index + 3];

        match opcode {
            1 => prog[output] = prog[input0] + prog[input1],
            2 => prog[output] = prog[input0] * prog[input1],
            _ => (),
        }

        index += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let input = "1,1,1,4,99,5,6,0,99";
        assert_eq!(part1(&parse_input(input)), 30);
    }
}
