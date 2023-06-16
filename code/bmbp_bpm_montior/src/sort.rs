use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Node {
    id: String,
    sort_num: i64,
    name: String,
    field1: String,
    field12: String,
    field13: String,
    field14: String,
    field15: String,
    field16: String,
    field17: String,
    field18: String,
    field19: String,
    field10: String,
    field21: String,
    field31: String,
    field41: String,
}

#[cfg(test)]
mod tests {

    use std::{fs::File, io::Write};

    use super::Node;
    use rand::Rng;

    #[test]
    fn it_works() {
        let row_count = 1000000000;
        let mut v = vec![];
        let mut rng = rand::thread_rng();
        for i in 0..row_count {
            let mut node = Node::default();
            node.id = i.to_string();
            let num = rng.gen_range(0i64..10000000000i64);
            node.sort_num = num;
            v.push(node);
            if i % 10000 == 0 {
                let json = serde_json::to_string(&v).unwrap();
                let p = format!("./{}.json", i);
                let mut f = File::create(p).unwrap();
                f.write_all(&json.as_bytes()).unwrap();
                println!("V===Size:{}", v.len());
                v = vec![];
            }
        }
        println!("V===Size:{}", v.len());
    }
}
