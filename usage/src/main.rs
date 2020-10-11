use binaryen_rs;
fn main() {
    let mut module = binaryen_rs::Module::new();
    

    {
        let params = binaryen_rs::Type::create(vec![
            binaryen_rs::Type::int_32(),
            binaryen_rs::Type::int_32(),
        ]);
        let result = binaryen_rs::Type::int_32();
        let lhs = module.get_local(0, binaryen_rs::Type::int_32());
        let rhs = module.get_local(1, binaryen_rs::Type::int_32());
        let add = module.binary(binaryen_rs::Op::add_int_32(), lhs, rhs);
        module.add_function("add_i32", params, result, vec![], add);
    }

    {
        let params = binaryen_rs::Type::create(vec![
            binaryen_rs::Type::float_64(),
            binaryen_rs::Type::float_64(),
        ]);
        let result = binaryen_rs::Type::float_64();
        let lhs = module.get_local(0, binaryen_rs::Type::float_64());
        let rhs = module.get_local(1, binaryen_rs::Type::float_64());
        let add = module.binary(binaryen_rs::Op::add_float_64(), lhs, rhs);
        module.add_function("add_f64", params, result, vec![], add);
    }
{
        let params = binaryen_rs::Type::create(vec![
            binaryen_rs::Type::float_32(),
            binaryen_rs::Type::float_32(),
        ]);
        let result = binaryen_rs::Type::float_32();
        let lhs = module.get_local(0, binaryen_rs::Type::float_32());
        let rhs = module.get_local(1, binaryen_rs::Type::float_32());
        let add = module.binary(binaryen_rs::Op::mul_float_64(), lhs, rhs);
        module.add_function("mul_f32", params, result, vec![], add);
    }
    module.print();
    // module.print_wat();
    // module.print_asmjs();

    println!("Hello, world!");
}
