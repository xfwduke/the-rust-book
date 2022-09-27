use std::option::IterMut;

struct Try {
    values: [String; 5],
    idx: usize,
}

impl Iterator for Try {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        if self.idx < 6 {
            Some(self.values[self.idx - 1].to_string())
        } else {
            None
        }
    }
}

impl Try {
    fn iter_mut(&mut self) -> IterMut<'_, Try> {
        return IterMut::new(self)
    }
}

fn main() {
    let mut obj = Try {
        values: [
            "aa".to_string(),
            "bb".to_string(),
            "cc".to_string(),
            "dd".to_string(),
            "ee".to_string(),
        ],
        idx: 0,
    };
    obj.iter
    // println!("{}", obj.next().unwrap());
    // println!("{}", obj.values[0])

    let mut v = vec![1, 2, 3];
    v.into_iter()
    let iv = v.iter_mut();
    for i in iv {
        // println!("{}", i);
        *i = 2;
    }
    for i in v.iter() {
        println!("{}", i);
    }
    // for i in iv {
    //     println!("{}", i);
    // }
    // obj.display();
    // obj.display();
}
