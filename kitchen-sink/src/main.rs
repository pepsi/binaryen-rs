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
    // {
    //     let mut eqref = Type::eqref();
    //     println!("  // BinaryenTypeEqref: {:?}\n", eqref);
    //     assert!(eqref.arity() == 1);
    // }
    // {
    //     let mut i31ref = Type::i31_ref();
    //     println!("  // BinaryenTypeExnreff: {:?}\n", exnref);
    //     assert!(exnref.arity() == 1);
    // }
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
}
fn main() {
    println!("Hello, world!");
}
