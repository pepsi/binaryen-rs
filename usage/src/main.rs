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
        let add = module.binary(binaryen_rs::Op::mul_float_32(), lhs, rhs);
        module.add_function("mul_f32", params, result, vec![], add);
    }
    {
        let params = binaryen_rs::Type::create(vec![
            binaryen_rs::Type::int_32(),
        ]);
        let result = binaryen_rs::Type::int_32();
        let lhs = module.get_local(0, binaryen_rs::Type::int_32());
        let rhs = module.make_const(binaryen_rs::literal_int_32(5));
        let add = module.binary(binaryen_rs::Op::add_int_32(), lhs, rhs);
        module.add_function("add_5", params, result, vec![], add);
    }
    
    {
        let params = binaryen_rs::Type::create(vec![
            binaryen_rs::Type::int_32(),
            binaryen_rs::Type::int_32(),

        ]);
        let result = binaryen_rs::Type::int_32();
        let mut bodies = vec![];
        for _i in 0..20{
            let lhs = module.get_local(0, binaryen_rs::Type::int_32());
            let rhs = module.get_local(1, binaryen_rs::Type::int_32());
    
            // let rhs = module.make_const(binaryen_rs::literal_int_32(5));
            let add = module.binary(binaryen_rs::Op::add_int_32(), lhs, rhs);
            let body: binaryen_rs::ExpressionRef = module.set_local(1, add);
            // let blk = module.new_nameless_block(vec![body], binaryen_rs::Type::none());
            bodies.push(body);
        }
        
        let final_block = module.new_block("test", bodies, binaryen_rs::Type::none());
        module.add_function("test", params, binaryen_rs::Type::none(), vec![binaryen_rs::Type::int_32()], final_block);
    }
    assert!(module.validate(), "Module failed to validate!");
    // module.optimize_with_level(0);s
    module.print();
    println!("Hello, world!");
}
