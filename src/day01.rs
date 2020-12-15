use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<f32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(masses: &[f32]) -> f32 {
    masses
        .iter()
        .fold(0_f32, |tot, m| tot + (m / 3.0).floor() - 2.0)
}

#[aoc(day1, part2)]
fn part2(masses: &[f32]) -> f32 {
    masses
        .iter()
        .fold(0_f32, |tot, &m| tot + calc_fuel_required_part2(m))
}

fn calc_fuel_required_part2(mass: f32) -> f32 {
    let fuel = (mass / 3.0).floor() - 2.0;

    if fuel <= 0.0 {
        0_f32
    } else {
        fuel + calc_fuel_required_part2(fuel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let input = "1969";
        assert_eq!(part1(&parse_input(input)), 654.0);
    }

    #[test]
    fn test_2_1() {
        let input = "1969";
        assert_eq!(part2(&parse_input(input)), 966.0);
    }
}
