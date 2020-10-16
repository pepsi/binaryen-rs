#[macro_use]
extern crate lazy_static;
use binaryen_rs::*;
lazy_static! {
    static ref v128_byes: Vec<i128> =
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
}
fn make_unary(module: &mut Module, op: Op, input_type: Type) -> ExpressionRef {
    let c = if input_type == Type::int_32() {
        module.make_const(Literal::int_32(-10))
    } else if input_type == Type::int_64() {
        module.make_const(Literal::int_64(-22))
    } else if input_type == Type::float_32() {
        module.make_const(Literal::float_32(-33.612f32))
    } else if input_type == Type::float_64() {
        module.make_const(Literal::float_64(-9005.841f64))
    } else {
        //TODO: Add vec128
        // TODO: allow matching expressions so that I dont need this trailing else statement
        module.make_const(Literal::int_32(-0))
    };
    module.unary(op, c)
}

fn make_binary(module: &mut Module, op: Op, input_type: Type) -> binaryen_rs::ExpressionRef {
    if input_type == Type::int_32() {
        let temp = module.make_const(Literal::int_32(-11));
        // Rust limitation, Cannot borrow `module` as mutable more than once in the same line.
        let l = module.make_const(Literal::int_32(-10));
        return module.binary(op, l, temp);
    } else if input_type == Type::int_64() {
        let temp = module.make_const(Literal::int_64(-23));
        let l = module.make_const(Literal::int_32(-22));
        return module.binary(op, l, temp);
    } else if input_type == Type::float_32() {
        let temp = module.make_const(Literal::float_32(-65.5f32));
        let l = module.make_const(Literal::float_32(-33.612f32));
        return module.binary(op, l, temp);
    } else if input_type == Type::float_64() {
        let temp = module.make_const(Literal::float_64(-9007.333f64));
        let l = module.make_const(Literal::float_64(-9005.841f64));
        return module.binary(op, l, temp);
    } else {
        // TODO: allow matching expressions so that I dont need this trailing else statement
        let temp = module.make_const(Literal::int_32(-0));
        let l = module.make_const(Literal::int_32(-0));
        return module.binary(op, l, temp);
    }
}

fn make_int_32(module: &mut Module, x: i32) -> ExpressionRef {
    module.make_const(Literal::int_32(x))
}
fn make_int_64(module: &mut Module, x: i64) -> ExpressionRef {
    module.make_const(Literal::int_64(x))
}
fn make_float_32(module: &mut Module, x: f32) -> ExpressionRef {
    module.make_const(Literal::float_32(x))
}
fn make_float_64(module: &mut Module, x: f64) -> ExpressionRef {
    module.make_const(Literal::float_64(x))
}
fn make_something(module: &mut Module) -> ExpressionRef {
    make_int_32(module, 1337)
}
fn make_dropped_int_32(module: &mut Module, x: i32) -> ExpressionRef {
    let c = module.make_const(Literal::int_32(x));
    module.drop_var(c)
}
//TODO: Simd
fn make_memory_init(module: &mut Module) -> ExpressionRef {
    let dest = make_int_32(module, 1024);
    let offset = make_int_32(module, 0);
    let size = make_int_32(module, 12);
    module.memory_init(0, dest, offset, size)
}
fn make_data_drop(module: &mut Module) -> ExpressionRef {
    module.data_drop(0)
}

fn make_memory_copy(module: &mut Module) -> ExpressionRef {
    let dest = make_int_32(module, 2048);
    let source = make_int_32(module, 1024);
    let size = make_int_32(module, 12);
    module.memory_copy(dest, source, size)
}
fn make_memory_fill(module: &mut Module) -> ExpressionRef {
    let dest = make_int_32(module, 2048);
    let source = make_int_32(module, 1024);
    let size = make_int_32(module, 12);
    module.memory_fill(dest, source, size)
}
#[test]
fn test_types() {
    //TODO like 152 - 158 example
    //TODO: add expanding example
    {
        let mut unreachable = Type::unreachable();
        println!("  // BinaryenTypeUnreachable: {:?}\n", unreachable);
        assert!(unreachable.arity() == 1);
    }
    {
        let mut i32_ = Type::int_32();
        println!("  // BinaryenTypeInt32: {:?}\n", i32_);
        assert!(i32_.arity() == 1);
    }

    {
        let mut i64_ = Type::int_64();
        println!("  // BinaryenTypeInt64: {:?}\n", i64_);
        assert!(i64_.arity() == 1);
    }
    {
        let mut f32_ = Type::float_32();
        println!("  // BinaryenTypeFloat32: {:?}", f32_);
        assert!(f32_.arity() == 1);
    }
    {
        let mut f64_ = Type::float_32();
        println!("  // BinaryenTypeFloat64: {:?}", f64_);
        assert!(f64_.arity() == 1);
    }
    //TODO: Add v128 test
    {
        let mut funcref = Type::funcref();
        println!("  // BinaryenTypeFuncref: {:?}", funcref);
        assert!(funcref.arity() == 1);
    }
    {
        let mut externref = Type::externref();
        println!("  // BinaryenTypeExternref: {:?}", externref);
        assert!(externref.arity() == 1);
    }
    {
        let mut exnref = Type::exnref();
        println!("  // BinaryenTypeExnreff: {:?}", exnref);
        assert!(exnref.arity() == 1);
    }
    //TODO Eqref, and i31ref has bindings at line 105 binaryen-c.h, but its not in bindings.rs
    {
        let mut eqref = Type::eqref();
        println!("  // BinaryenTypeEqref: {:?}\n", eqref);
        assert!(eqref.arity() == 1);
    }
    {
        let mut i31ref = Type::i31ref();
        println!("  // BinaryenTypeI31Ref: {:?}\n", i31ref);
        assert!(i31ref.arity() == 1);
    }
    {
        println!("  // BinaryenTypeAuto: {:?}", Type::auto())
    }
    {
        let i32_0 = Type::int_32();
        let i32_1 = Type::int_32();

        let pair = vec![i32_0, i32_1];
        let mut i32_pair = Type::create(pair.clone());
        assert!(i32_pair.arity() == 2);
        //TODO: expand ln 239

        let duplicate_pair = Type::create(pair);
        assert!(duplicate_pair == i32_pair);
        let pair = vec![Type::float_32(), Type::float_32()];
        let float_pair = Type::create(pair);
        assert!(float_pair != i32_pair);
    }
}
#[test]
fn test_features() {
    assert_eq!(Features::mvp(), 0);
    assert_eq!(Features::atomics(), 1);
    assert_eq!(Features::bulk_memory(), 16);
    assert_eq!(Features::mutable_globals(), 2);
    assert_eq!(Features::non_trapping_fp_to_int(), 4);
    assert_eq!(Features::sign_ext(), 32);
    assert_eq!(Features::simd_128(), 8);
    assert_eq!(Features::exception_handling(), 64);
    assert_eq!(Features::tail_call(), 128);
    assert_eq!(Features::reference_types(), 256);
    assert_eq!(Features::multi_value(), 512);
    assert_eq!(Features::gc(), 1024);
    assert_eq!(Features::feature_all(), 4095);

    //     println!("BinaryenFeatureMVP: {:?}", Features::mvp());
    //     println!("BinaryenFeatureAtomics: {:?}", Features::atomics());
    //     println!("BinaryenFeatureBulkMemory: {:?}", Features::bulk_memory());
}
#[test]
fn test_core() {
    let mut module = Module::new();

    let (ConstI32, ConstI64, ConstF32, ConstF64, ConstF32Bits, ConstF64Bits) = (
        module.make_const(Literal::int_32(1)),
        module.make_const(Literal::int_64(2)),
        module.make_const(Literal::float_32(3.14f32)),
        module.make_const(Literal::float_64(2.1824f64)),
        module.make_const(Literal::float_32_bits(0xffff123)), //TODO: Figure out why this cant be 0xffff1234 like in the example (ln 279)
        module.make_const(Literal::float_64_bits(0xffff1234456abcdi64)),
    );
    let switch_value_names = vec!["the-value"];
    let switch_body_names = vec!["the-nothing"];

    let call_operands2 = vec![
        make_int_32(&mut module, 13),
        make_float_64(&mut module, 3.7),
    ];
    let call_operands4 = vec![
        make_int_32(&mut module, 13),
        make_int_64(&mut module, 37),
        make_float_32(&mut module, 1.3f32),
        make_float_64(&mut module, 3.7),
    ];
    let call_operands4b = vec![
        make_int_32(&mut module, 13),
        make_int_64(&mut module, 37),
        make_float_32(&mut module, 1.3f32),
        make_float_64(&mut module, 3.7),
    ];
    let tuple_elements4a = vec![
        make_int_32(&mut module, 13),
        make_int_64(&mut module, 37),
        make_float_32(&mut module, 1.3f32),
        make_float_64(&mut module, 3.7f64),
    ];

    let iIfF = vec![
        Type::int_32(),
        Type::int_64(),
        Type::float_32(),
        Type::float_64(),
    ];

    let iIfF = Type::create(iIfF);

    let (
        temp1,
        temp2,
        temp3,
        temp4,
        temp5,
        temp6,
        temp7,
        temp8,
        temp9,
        temp10,
        temp11,
        temp12,
        temp13,
        temp14,
        temp15,
        temp16,
    ) = (
        make_int_32(&mut module, 1),
        make_int_32(&mut module, 2),
        make_int_32(&mut module, 3),
        make_int_32(&mut module, 4),
        make_int_32(&mut module, 5),
        make_int_32(&mut module, 0),
        make_int_32(&mut module, 1),
        make_int_32(&mut module, 0),
        make_int_32(&mut module, 1),
        make_int_32(&mut module, 1),
        make_int_32(&mut module, 3),
        make_int_32(&mut module, 5),
        make_int_32(&mut module, 10),
        make_int_32(&mut module, 11),
        make_int_32(&mut module, 110),
        make_int_32(&mut module, 111),
    );
    let externrefExpr = module.ref_null(Type::externref());
    let mut funcrefExpr = module.ref_null(Type::funcref());
    funcrefExpr = module.ref_func("kitchen()sinker");
    let exnref = module.ref_null(Type::exnref());
    let i31ref = {
        let temp = make_int_32(&mut module, 1);
        module.make_i31(temp)
    };

    //Events
    module.add_event("a-event", 0, Type::int_32(), Type::none());

    let try_body = {
        let temp = vec![make_int_32(&mut module, 0)];
        module.throw("a-event", temp)
    };

    //TODO: Clean this up and put comments
    let catch_body = {
        let pop = module.pop(Type::exnref());
        let mut children = vec![module.set_local(5, pop)];
        let loca = module.get_local(5, Type::exnref());
        let try_block_children = vec![module.br_on_exn("try-block", "a-event", loca)];
        let blk = module.new_block("try-block", try_block_children, Type::int_32());
        children.push(module.drop_var(blk));
        module.new_nameless_block(children, Type::none())
    };

    let i32_ = Type::int_32();
    let i64_ = Type::int_64();
    let f32_ = Type::float_32();
    let f64_ = Type::float_64();
    macro_rules! binop {
        ($name: ident, $type: ident) => {
            make_binary(&mut module, Op::$name(), $type)
        };
    }
    macro_rules! unop {
        ($name: ident, $type: ident) => {
            make_unary(&mut module, Op::$name(), $type)
        };
    }
    unop!(clz_int32, i32_);
    unop!(ctz_int32, i32_);

    binop!(add_int32, i32_);
    binop![add_int64, i64_];

    //TODO: Fill the rest of the operators in

    make_memory_init(&mut module);
    make_data_drop(&mut module);
    make_memory_copy(&mut module);
    make_memory_fill(&mut module);
    module.r#if(temp1, temp2, Some(temp3));
    module.r#if(temp4, temp5, None); //TODO: Get actuall null pointer owrking (ln 680)
    let mut value_list = vec![];
    let l1 = {
        let temp = make_int_32(&mut module, 0);
        module.r#loop("in", temp)
    };
    let l2 = {
        let temp = make_int_32(&mut module, 0);
        module.r#loop("z", temp)
    };
    let b0 = module.r#break("the-value", Some(temp6), Some(temp7));
    let b1 = {
        let temp = make_int_32(&mut module, 2);
        module.r#break("the-nothing", Some(temp), None)
    };
    let b2 = {
        let temp = make_int_32(&mut module, 3);
        module.r#break("the-value", None, Some(temp))
    };
    let b3 = {
        // let temp = make_int_32(&mut module, 3);
        module.r#break("the-nothing", None, None)
    };
    let s0 = module.switch(switch_value_names, "the-value", temp8, temp9);
    let s1 = {
        let temp = make_int_32(&mut module, 2);
        module.switch(
            switch_body_names,
            "the-nothing",
            temp,
            ExpressionRef::null_expr(),
        )
    };
    let called = {
        let val = module.r#call("kitchen()sinker", call_operands4, Type::int_32());
        module.unary(Op::eq_z_int32(), val)
    };
    let u = {
        let v = module.r#call("an-imported", call_operands2, Type::float_32());
        let val = module.unary(Op::trunc_s_float32_to_int32(), v);
        module.unary(Op::eq_z_int32(), val)
    };
    let u2 = {
        let m = make_int_32(&mut module, 2449);
        let indirectly_called = module.call_indirect(m, call_operands4b, iIfF, Type::int_32());
        module.unary(Op::eq_z_int32(), indirectly_called)
    };
    let d = {
        let l0 = module.get_local(0, Type::int_32());
        module.drop_var(l0)
    };

    let ls = {
        let i = make_int_32(&mut module, 101);
        module.set_local(0, i)
    };

    let lt = {
        let i = make_int_32(&mut module, 102);
        module.tee_local(0, i, Type::int_32())
    };

    // value_list.push(l1);
    // value_list.push(l2);
    // value_list.push(b0);
    // value_list.push(b1);
    // value_list.push(b2);
    // value_list.push(b3);
    // value_list.push(s0);
    // value_list.push(s1);
    // value_list.push(called);
    // value_list.push(u);
    // value_list.push(u2);
    // value_list.push(d);
    // value_list.push(ls);
    // value_list.push(lt);
    value_list.append(&mut vec![
        {
            let i = make_int_32(&mut module, 2);
            module.load(4, 0, 0, 0, Type::int_32(), i)
        },
        {
            let i = make_int_32(&mut module, 9);
            module.load(8, 0, 2, 8, Type::int_32(), i)
        },
        {
            module.store(4,0,0,temp13,temp14, Type::int_32())
        },
        {
            module.store(8,2,4,temp15,temp16, Type::int_64())
        },
        {
            module.select(temp10, temp11, temp12, Type::auto())
        },
        {
            let i = make_int_32(&mut module, 1337);
            module.r#return(i)
        },
        {
            //TODO, find a way around redefining call_operands4
            let call_operands4 = vec![
                make_int_32(&mut module, 13),
                make_int_64(&mut module, 37),
                make_float_32(&mut module, 1.3f32),
                make_float_64(&mut module, 3.7),
            ];
            module.return_call("kitchen()sinker", call_operands4, Type::int_32())
        },
        {
            let call_operands4b = vec![
                make_int_32(&mut module, 13),
                make_int_64(&mut module, 37),
                make_float_32(&mut module, 1.3f32),
                make_float_64(&mut module, 3.7),
            ];
            let target = make_int_32(&mut module, 2499);
            module.return_call_indirect(target, call_operands4b, iIfF, Type::int_32())
            // call_operands4b,

        }

    ]);


    let blk = module.new_nameless_block(value_list, Type::none());
    blk.print();
}

fn main() {
    println!("You should run with `cargo test` from command line, not `cargo run` :)");
}
