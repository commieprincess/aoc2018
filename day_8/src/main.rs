fn main() {
    let input = include_str!("input.txt").trim();

    let tree = Node::from_iter(&mut input.split_whitespace().map(|v| str::parse(v).unwrap()));

    println!("part 1: {}", tree.metadata_total());
    println!("part 2: {}", tree.score());
}

struct Node {
    child_nodes: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn from_iter<T: Iterator<Item = u32>>(iter: &mut T) -> Node {
        let child_count = iter.next().unwrap();
        let metadata_count = iter.next().unwrap();

        let mut child_nodes = Vec::new();
        let mut metadata = Vec::new();

        for _ in 0..child_count {
            child_nodes.push(Node::from_iter(iter));
        }

        for _ in 0..metadata_count {
            metadata.push(iter.next().unwrap());
        }

        Node {
            child_nodes,
            metadata,
        }
    }

    fn metadata_total(&self) -> u32 {
        let mut total = self.metadata.iter().sum();

        for c in &self.child_nodes {
            total += c.metadata_total();
        }

        total
    }

    fn score(&self) -> u32 {
        if self.child_nodes.is_empty() {
            self.metadata.iter().sum()
        } else {
            let mut sum = 0;

            for v in &self.metadata {
                if *v != 0 && self.child_nodes.get(*v as usize - 1).is_some() {
                    sum += self.child_nodes[*v as usize - 1].score();
                }
            }

            sum
        }
    }
}
