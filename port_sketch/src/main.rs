use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
struct Shape(Vec<u32>);

trait Primitive {}
impl Primitive for u32 {}
impl Primitive for f32 {}
impl Primitive for Box<dyn Primitive> {}
impl<T: Primitive> Primitive for Box<T> {}
impl<T: Primitive> Primitive for Port<T> {}

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
    let primitive_u32 = Port::Primitive(5);
    let primitive_f32 = Port::Primitive(5.1);

    let primitive_boxed = Port::Primitive(Box::new(5.1));
    //let non_primitive_boxed = Port::Primitive(Box::new("hi"));

    // should this be possible? can it be prevented?
    let nested_port_primitive = Port::Primitive(Port::Primitive(5));

    let array_u32 = Port::Array(Shape(vec![2, 2]), vec![1, 2, 3, 4]);
    let array_f32 = Port::Array(Shape(vec![2, 2]), vec![1.1, 2.0, 3.1, 4.1]);
    //let array_u32_f32 = Port::Array(Shape(vec![2, 2]), vec![1, 2, 3.1, 4.1]);

    //let struct_no_work: Port<Box<dyn Primitive>> =
    //    Port::Struct([("bar".to_string(), Box::new(2u32))].into());

    // must specify type before??
    let box_u32: Box<dyn Primitive> = Box::new(2u32);
    let box_f32: Box<dyn Primitive> = Box::new(2.0f32);
    let my_struct: Port<Box<dyn Primitive>> = Port::Struct([("foo".to_string(), box_u32)].into());
    let my_struct2: Port<Box<dyn Primitive>> = Port::Struct([("bar".to_string(), box_f32)].into());

    let box_u32: Box<dyn Primitive> = Box::new(2u32);
    let box_f32: Box<dyn Primitive> = Box::new(2.0f32);
    let my_struct3: Port<Box<dyn Primitive>> =
        Port::Struct([("foo".to_string(), box_u32), ("bar".to_string(), box_f32)].into());

    // no work let my_struct = Port::Struct([("foo".to_string(), Box::new(primitive_u32))].into());
    let box_primitive_u32: Box<dyn Primitive> = Box::new(primitive_u32);
    let box_primitive_f32: Box<dyn Primitive> = Box::new(primitive_f32);
    let my_struct4: Port<Box<dyn Primitive>> = Port::Struct(
        [
            ("foo".to_string(), box_primitive_u32),
            ("bar".to_string(), box_primitive_f32),
        ]
        .into(),
    );

    let box_array_u32: Box<dyn Primitive> = Box::new(array_u32);
    let box_array_f32: Box<dyn Primitive> = Box::new(array_f32);
    let box_struct: Box<dyn Primitive> = Box::new(my_struct4);
    let d: Port<Box<dyn Primitive>> = Port::Struct(
        [
            ("baz1".to_string(), box_array_u32),
            ("baz2".to_string(), box_array_f32),
            ("baz3".to_string(), box_struct),
        ]
        .into(),
    );
}
//struct Node {
//    ports: HashMap<String, Port<Box<dyn Primitive>>>,
//}
