use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
struct Shape(Vec<u32>);

trait Primitive {}
impl Primitive for u32 {}
impl Primitive for f32 {}
impl Primitive for Box<dyn Primitive> {}
impl Primitive for Box<u32> {}
impl Primitive for Box<f32> {}
//impl Primitive for Port<Box<dyn Primitive>> {}
impl Primitive for Port<u32> {}
impl Primitive for Port<f32> {}
impl Primitive for Port<Box<u32>> {}
impl Primitive for Port<Box<f32>> {}

enum Port<T>
where
    T: Primitive,
{
    Primitive(T),
    // basic implementation:
    //  stored as flat vec, use Shape to index into flat array
    // Likely better to just use ndarray
    Array(Shape, Vec<T>),
    Struct(HashMap<String, Box<dyn Primitive>>),
}

fn main() {
    let a = Port::Primitive(5);
    let b = Port::Primitive(5.1);
    let b1 = Port::Primitive(Box::new(5.1));
    let b1 = Port::Primitive(Port::Primitive(5));
    let c = Port::Array(Shape(vec![2, 2]), vec![1, 2, 3, 4]);
    let c1 = Port::Array(Shape(vec![2, 2]), vec![1.1, 2.0, 3.1, 4.1]);

    let ex: Box<dyn Primitive> = Box::new(2u32);
    let exf: Box<dyn Primitive> = Box::new(2.0f32);

    let d: Port<Box<dyn Primitive>> = Port::Struct(
        [
            ("foo".to_string(), ex),
            ("bar".to_string(), exf),
            ("baz".to_string(), Box::new(a)),
        ]
        .into(),
    );

    //let e = Port::Struct(
    //    [
    //        ("foo".to_string(), Port::Primitive(1)),
    //        ("bar".to_string(), Port::Primitive(2.1)),
    //    ]
    //    .into(),
    //);
    //
    //dbg!(a, b, c);
}
//struct Node {
//    ports: HashMap<String, Port<Box<dyn Primitive>>>,
//}
