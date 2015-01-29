pub struct Foo { x: i32 }

pub fn go() {
    println!("{:?}", std::any::TypeId::of::<Foo>());
}
