use binaryen_rs;
fn general_test() {
    let mut module = binaryen_rs::Module::new();

    {
        let params = binaryen_rs::Type::create(vec![
            binaryen_rs::Type::int_32(),
            binaryen_rs::Type::int_32(),
        ]);
        let result = binaryen_rs::Type::int_32();
        let lhs = module.get_local(0, binaryen_rs::Type::int_32());
        let rhs = module.get_local(1, binaryen_rs::Type::int_32());
        let add = module.binary(binaryen_rs::Op::add_int32(), lhs, rhs);
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
        let add = module.binary(binaryen_rs::Op::add_float64(), lhs, rhs);
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
        let add = module.binary(binaryen_rs::Op::mul_float32(), lhs, rhs);
        module.add_function("mul_f32", params, result, vec![], add);
    }
    {
        let params = binaryen_rs::Type::create(vec![binaryen_rs::Type::int_32()]);
        let result = binaryen_rs::Type::int_32();
        let lhs = module.get_local(0, binaryen_rs::Type::int_32());
        let rhs = module.make_const(binaryen_rs::Literal::int_32(5));
        let add = module.binary(binaryen_rs::Op::add_int32(), lhs, rhs);
        module.add_function("add_5", params, result, vec![], add);
    }

    {
        let params = binaryen_rs::Type::create(vec![
            binaryen_rs::Type::int_32(),
            binaryen_rs::Type::int_32(),
        ]);
        let result = binaryen_rs::Type::int_32();
        let mut bodies = vec![];
        for _i in 0..1 {
            let lhs = module.get_local(0, binaryen_rs::Type::int_32());
            let rhs = module.get_local(1, binaryen_rs::Type::int_32());

            // let rhs = module.make_const(binaryen_rs::literal_int_32(5));
            let add = module.binary(binaryen_rs::Op::atomic_r_m_w_xor(), lhs, rhs);
            let body: binaryen_rs::ExpressionRef = module.set_local(1, add);
            let blk = module.new_nameless_block(vec![body], binaryen_rs::Type::none());
            bodies.push(blk);
        }

        let final_block = module.new_block("test", bodies, binaryen_rs::Type::none());
        module.add_function(
            "test",
            params,
            binaryen_rs::Type::none(),
            vec![binaryen_rs::Type::int_32()],
            final_block,
        );
        let exp = module.add_export("test", "te2st");
        // println!("{:?}", exp);
    }

    module.print();
}
use std::thread::*;
use std::sync::*;
fn thread_test(num_threads: u32) {
    let module = binaryen_rs::Module::new();
    let mod_mutex = std::sync::Arc::new(std::sync::Mutex::new(module));
    let mut handles = vec![];
    for i in 0..num_threads {
        let v = mod_mutex.clone();
        handles.push(spawn(move || {
            let mut module = v.lock().unwrap();
            let name = format!("add{}", i);
            let adder = {
                let op = binaryen_rs::Op::add_int32();
                let lhs = module.make_const(binaryen_rs::Literal::int_32(i as i32));
                let rhs = module.make_const(binaryen_rs::Literal::int_32((i as i32) * 25000));
                module.binary(op, lhs, rhs)
            };
            module.add_function(
                &name,
                binaryen_rs::Type::none(),
                binaryen_rs::Type::int_32(),
                vec![],
                adder,
            );
            module.add_export(&name, &name);
            println!(" Thread {} export function {}", i, &name);
        }))
    }
    for handle in handles {
        handle.join().unwrap();
    }
    mod_mutex.clone().lock().unwrap().print()
}
use binaryen_rs::*;
use std::sync::{Arc, Mutex};
fn thread_2_test(num_threads: u32) {
    let mut module = Module::new();
    let safe_module = Arc::new(Mutex::new(module));
    let handles = (0..num_threads)
        .into_iter()
        .map(|i| {
            let module = std::sync::Arc::clone(&safe_module);
            spawn(move || {
                {
                    for j in 0..25{
                        let mut data = module.lock().unwrap();
                        let name = format!("{}add{}", j, i);
                        let adder = {
                            let op = binaryen_rs::Op::add_int32();
                            let lhs = data.make_const(binaryen_rs::Literal::int_32(i as i32));
                            let rhs =
                            data.make_const(binaryen_rs::Literal::int_32((i as i32) * 25000));
                            data.binary(op, lhs, rhs)
                        };
                        data.add_function(
                            &name,
                            binaryen_rs::Type::none(),
                            binaryen_rs::Type::int_32(),
                            vec![],
                            adder,
                        );
                        data.add_export(&name, &name);
                    }
                    }
                    

                // std::thread::sleep(std::time::Duration::from_secs(1))
            })
        })
        .collect::<Vec<std::thread::JoinHandle<_>>>();
    for handle in handles {
        handle.join().unwrap();
    }
    {
        safe_module.lock().unwrap().print();
    }
    // handles.into_iter().map(|h|h)
}
fn main() {
    thread_2_test(2);
}
