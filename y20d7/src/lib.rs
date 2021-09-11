// use graphlib::Graph;
// use helper::{print_part_1, FromInput};
// use std::str::FromStr;
//
// const FILENAME: &str = env!("CARGO_PKG_NAME");
//
// struct BagContain {
//     colour: String,
//     contains: Vec<String>,
//     amounts: Vec<i32>,
// }
//
// impl FromStr for BagContain {
//     type Err = std::string::ParseError;
//
//     fn from_str(string: &str) -> Result<Self, Self::Err> {
//         let bag_string1 = string
//             .replace(" bags", "")
//             .replace(" bag", "")
//             .replace('.', "");
//
//         let bag_string: Vec<_> = bag_string1.split(" contain ").collect();
//         let colour = bag_string[0].to_owned();
//         if bag_string[1].contains("no other") {
//             let contains = vec![];
//             let amounts = vec![];
//             Ok(Self {
//                 colour,
//                 contains,
//                 amounts,
//             })
//         } else {
//             let raw_contains = bag_string[1].split(", ");
//             let (contains, amounts): (Vec<_>, Vec<i32>) = raw_contains
//                 .map(|s| (s[2..].to_owned(), s[0..1].parse::<i32>().unwrap()))
//                 .unzip();
//             Ok(Self {
//                 colour,
//                 contains,
//                 amounts,
//             })
//         }
//     }
// }
//
pub fn main() {
    //     let bags = BagContain::from_input(FILENAME);
    //     let mut vertices = vec![];
    //     let mut graph: Graph<usize> = Graph::new();
    //     let colours: Vec<_> = bags.iter().map(|b| b.colour.clone()).collect();
    //     for (i, c) in colours.iter().enumerate() {
    //         let v = graph.add_vertex(i);
    //         vertices.push(v)
    //     }
    //
    //     for (i, b) in bags.iter().enumerate() {
    //         for c in &b.contains {
    //             let u = colours.iter().position(|s| s == c).unwrap();
    //             graph.add_edge(&vertices[i], &vertices[u]);
    //         }
    //     }
    //
    //     let n_0 = colours
    //         .iter()
    //         .position(|s| s == &"shiny gold".to_owned())
    //         .unwrap();
    //     let v_0 = vertices[n_0];
    //     let mut count = 0;
    //     for v in vertices {
    //         let mut d = graph.dijkstra(&v, &v_0);
    //         if let Some(_n) = d.next() {
    //             count += 1
    //         }
    //     }
    //
    //     print_part_1(count -1)
    // }
    //
    // #[cfg(test)]
    // mod tests {
    //     use crate::BagContain;
    //     use std::str::FromStr;
    //
    //     #[test]
    //     fn test_from_str() {
    //         let s =
    //             "dim tan bags contain 3 shiny teal bags, 5 bright white bags, 4 striped bronze bags.";
    //         let bc = BagContain::from_str(s).unwrap();
    //         assert_eq!(bc.colour, "dim tan");
    //         assert_eq!(
    //             bc.contains,
    //             vec!["shiny teal", "bright white", "striped bronze"]
    //         );
    //         assert_eq!(bc.amounts, vec![3, 5, 4]);
    //     }
    //     #[test]
    //     fn test_from_str_no_ther() {
    //         let s = "dull aqua bags contain no other bags.";
    //         let bc = BagContain::from_str(s).unwrap();
    //         let c: Vec<String> = Vec::new();
    //         let d: Vec<i32> = Vec::new();
    //         assert_eq!(bc.colour, "dull aqua");
    //         assert_eq!(bc.contains, c);
    //         assert_eq!(bc.amounts, d);
    //     }
}
