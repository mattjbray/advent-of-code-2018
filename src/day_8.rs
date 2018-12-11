use combine::{count, easy::Stream, token, Parser};

pub fn run(path: &str) {
    let input = std::fs::read_to_string(path).expect("Couldn't read input file");

    let node = parse_node()
        .easy_parse(&input)
        .expect("Couldn't parse nodes")
        .0;

    println!("Day 8, part 1: {}", node.sum_metadatas());

    println!("Day 8, part 2: {}", node.value());
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

    fn value(&self) -> u32 {
        if self.children.len() == 0 {
            self.sum_metadatas()
        } else {
            self.metadata
                .iter()
                .map(|&i| {
                    self.children
                        .get((i as usize) - 1)
                        .map(|n| n.value())
                        .unwrap_or(0)
                }).sum()
        }
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

    fn node_b() -> Node {
        Node {
            children: vec![],
            metadata: vec![10, 11, 12],
        }
    }

    fn node_a() -> Node {
        Node {
            children: vec![node_b(), node_c()],
            metadata: vec![1, 1, 2],
        }
    }

    #[test]
    fn test_parse_node() {
        assert_eq!(parse_node().easy_parse("0 1 99"), Ok((node_d(), "")));

        assert_eq!(parse_node().easy_parse("1 1 0 1 99 2"), Ok((node_c(), "")));

        assert_eq!(
            parse_node().easy_parse("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"),
            Ok((node_a(), ""))
        );
    }

    #[test]
    fn test_sum_metadatas() {
        assert_eq!(node_a().sum_metadatas(), 138);
    }

    #[test]
    fn test_value() {
        assert_eq!(node_c().value(), 0);
        assert_eq!(node_b().value(), 33);
        assert_eq!(node_a().value(), 66);
    }
}
