use std::collections::HashMap;
use std::collections::HashSet;

pub fn run(path: &str) {
    let input = std::fs::read_to_string(path).expect("Couldn't read data file");

    let (steps, _) = parser::parse_steps(&input).expect("Couldn't parse input steps");

    let mut step_deps = build_step_deps(&steps);

    let step_order: String = step_order(&mut step_deps).into_iter().collect();

    println!("Day 7, part 1: {}", step_order);

    let mut step_deps = build_step_deps(&steps);

    let part_2_solution = duration(&mut step_deps, 5, 60);
    println!("Day 7, part 2: {}", part_2_solution);
}

type Step = char;

type StepDeps = HashMap<Step, HashSet<Step>>;

fn build_step_deps(step_lines: &[(Step, Step)]) -> StepDeps {
    step_lines
        .iter()
        .fold(HashMap::new(), |mut step_deps, (dep, step)| {
            step_deps
                .entry(*step)
                .and_modify(|deps| {
                    deps.insert(*dep);
                }).or_insert_with(|| {
                    let mut deps = HashSet::new();
                    deps.insert(*dep);
                    deps
                });
            step_deps.entry(*dep).or_insert_with(|| HashSet::new());
            step_deps
        })
}

fn get_available_steps(step_deps: &StepDeps) -> Vec<Step> {
    step_deps
        .iter()
        .filter_map(
            |(&step, deps)| {
                if deps.len() == 0 {
                    Some(step)
                } else {
                    None
                }
            },
        ).collect()
}

fn do_step(step: Step, step_deps: &mut StepDeps) {
    step_deps.remove(&step);
    step_deps.iter_mut().for_each(|(_, deps)| {
        deps.remove(&step);
    });
}

fn step_order(step_deps: &mut StepDeps) -> Vec<Step> {
    let mut done_steps = Vec::new();

    while step_deps.len() > 0 {
        let mut available_steps = get_available_steps(step_deps);
        available_steps.sort();
        let step = available_steps.remove(0);
        done_steps.push(step);
        do_step(step, step_deps);
    }

    done_steps
}

fn step_cost(step: Step, base: u32) -> u32 {
    step as u32 - 64 + base
}

#[derive(Debug)]
struct Task {
    step: Step,
    started_at: u32,
}

fn duration(step_deps: &mut StepDeps, workers: usize, base_cost: u32) -> u32 {
    let mut tasks: Vec<Task> = vec![];

    for now in 0.. {
        let (done_tasks, mut tasks_still_running): (Vec<Task>, Vec<Task>) = tasks
            .into_iter()
            .partition(|task| now - task.started_at >= step_cost(task.step, base_cost));

        for t in done_tasks.iter() {
            do_step(t.step, step_deps);
        }

        let available_workers = workers - tasks_still_running.len();
        let new_tasks: Vec<Task> = get_available_steps(step_deps)
            .iter()
            .filter(|&step| !tasks_still_running.iter().any(|task| task.step == *step))
            .take(available_workers)
            .map(|&step| Task {
                step,
                started_at: now,
            }).collect();

        tasks_still_running.extend(new_tasks);
        tasks = tasks_still_running;
        if tasks.len() == 0 {
            return now;
        }
    }

    0
}

mod parser {
    use combine::parser::char::{newline, string, upper};
    use combine::sep_by;
    use combine::Parser;

    // Step P must be finished before step O can begin.
    pub fn parse_steps(
        input: &str,
    ) -> Result<
        (Vec<(char, char)>, &str),
        combine::easy::Errors<char, &str, combine::stream::PointerOffset>,
    > {
        let step_line = || {
            (
                string("Step "),
                upper(),
                string(" must be finished before step "),
                upper(),
                string(" can begin."),
            )
                .map(|t| (t.1, t.3))
        };

        sep_by(step_line(), newline()).easy_parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_example_step_deps() -> StepDeps {
        build_step_deps(&[
            ('C', 'A'),
            ('C', 'F'),
            ('A', 'B'),
            ('A', 'D'),
            ('B', 'E'),
            ('D', 'E'),
            ('F', 'E'),
        ])
    }

    #[test]
    fn test_get_available_steps() {
        let step_deps: StepDeps = get_example_step_deps();

        assert_eq!(get_available_steps(&step_deps), vec!['C']);
    }

    #[test]
    fn test_do_step() {
        let mut step_deps: StepDeps = get_example_step_deps();

        do_step('C', &mut step_deps);

        assert!(!step_deps.contains_key(&'C'));
        assert_eq!(step_deps.get(&'A'), Some(&HashSet::new()));
    }

    #[test]
    fn test_step_order() {
        let mut step_deps: StepDeps = get_example_step_deps();

        assert_eq!(
            step_order(&mut step_deps),
            vec!['C', 'A', 'B', 'D', 'F', 'E']
        );
    }

    #[test]
    fn test_step_cost() {
        assert_eq!(step_cost('A', 0), 1);
        assert_eq!(step_cost('A', 60), 61);
        assert_eq!(step_cost('Z', 60), 86);
    }

    #[test]
    fn test_duration() {
        let mut step_deps: StepDeps = get_example_step_deps();

        assert_eq!(duration(&mut step_deps, 2, 0), 15);
    }
}
