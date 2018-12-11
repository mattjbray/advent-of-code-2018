use combine::{count, easy::Stream, token, Parser};

pub fn run(path: &str) {
    let input = std::fs::read_to_string(path).expect("Couldn't read input file");

    let node = parse_node()
        .easy_parse(&input)
        .expect("Couldn't parse nodes")
        .0;

    let part_1_solution = node.sum_metadatas();
    println!("Day 8, part 1: {}", part_1_solution);
}

#[derive(Debug, PartialEq)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn sum_metadatas(&self) -> u32 {
        let child_sum: u32 = self.children.iter().map(|n| n.sum_metadatas()).sum();
        self.metadata.iter().sum::<u32>() + child_sum
    }
}

fn digits_u32<'a>() -> impl Parser<Input = Stream<&'a str>, Output = u32> {
    combine::from_str(combine::parser::range::take_while1(|c: char| {
        c.is_digit(10)
    }))
}

fn parse_node_<'a>() -> impl Parser<Input = Stream<&'a str>, Output = Node> {
    digits_u32()
        .skip(token(' '))
        .then(move |num_child_nodes: u32| {
            digits_u32().then(move |num_metadatas: u32| {
                (
                    count(num_child_nodes as usize, token(' ').with(parse_node())),
                    count(num_metadatas as usize, token(' ').with(digits_u32())),
                )
                    .map(|t| Node {
                        children: t.0,
                        metadata: t.1,
                    })
            })
        })
}

parser!{
    fn parse_node['a]()(Stream<&'a str>) -> Node
    {
        parse_node_()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn node_d() -> Node {
        Node {
            children: vec![],
            metadata: vec![99],
        }
    }

    fn node_c() -> Node {
        Node {
            children: vec![node_d()],
            metadata: vec![2],
        }
    }

    #[test]
    fn test_parse_node() {
        assert_eq!(parse_node().easy_parse("0 1 99"), Ok((node_d(), "")));

        assert_eq!(parse_node().easy_parse("1 1 0 1 99 2"), Ok((node_c(), "")));
    }

    #[test]
    fn test_sum_metadatas() {
        let node = parse_node()
            .easy_parse("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2")
            .expect("Failed to parse example")
            .0;

        assert_eq!(node.sum_metadatas(), 138);
    }
}
