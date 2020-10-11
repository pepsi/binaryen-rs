use binaryen_rs;
fn main() {
    let mut module = binaryen_rs::Module::new();
    let mut ii = binaryen_rs::Type::create(vec![
        binaryen_rs::Type::int_32(),
        binaryen_rs::Type::int_32(),
    ]);
    let result = binaryen_rs::Type::int_32();
    let lhs = module.get_local(0, binaryen_rs::Type::int_32());
    let rhs = module.get_local(1, binaryen_rs::Type::int_32());
    let add = module.binary(binaryen_rs::Op::add_int_32(), lhs, rhs);
    module.add_function("add", ii, result, vec![], add);
    module.print();
    module.print_wat();
    module.print_asmjs();

    println!("Hello, world!");
}
