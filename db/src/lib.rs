use std::marker::Sized;

#[derive(Debug, Clone, Copy, PartialEq)]
enum LayoutType<'a> {
    Str(&'a str),
    Ref(&'a str),
    Num(i64),
}

struct Bag<'a> {
    values: Vec<(&'a str, LayoutType<'a>)>
}

impl<'a> Bag<'a> {

    fn new() -> Self {
        Bag { values: Vec::new() }
    }

    fn get(&self, key: &str) -> Result<LayoutType, &'static str> {
        for x in self.values.iter() {
            if x.0 == key { return Ok(x.1); }
        }
        return Err("not found")
    }

    fn add(&mut self, key: &'a str, value: &'a str) {
        self.values.push((key, LayoutType::Str(value)));
    }
}

// struct Ref {
//     id: &str
// }

// struct Obj {
//     class: Ref
// }

// struct Doc {
//     obj: Obj,
//     id: Ref
// }

// struct Class {
//     doc: Doc
// }

#[cfg(test)]
mod tests {
    use crate::*;
    
    #[test]
    fn amount_add_assign() {
        let mut b = Bag::new();
        b.add("hey", "there");
        let result = b.get("hey");
        match result {
            Ok(x) => assert_eq!(x, LayoutType::Str("there")),
            Err(e) => panic!(e)
        }
        
    }
}