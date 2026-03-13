use named_id::RenameAll;

#[derive(Debug)]
struct NotRenameable;

#[derive(Debug, RenameAll)]
struct MyStruct {
    good: u32,
    bad: NotRenameable,
}

fn main() {}
