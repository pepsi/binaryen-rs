use binaryen_rs::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
enum Xvar{
    Local(String)
}
#[derive(Debug, Deserialize, Serialize)]
enum Xinstruction {
    Add(Xvar, Xvar),
    Sub(Xvar, Xvar)
}
#[derive(Debug, Deserialize, Serialize)]
enum Xtype{
    I32,I64,F32,F64
}
#[derive(Debug, Deserialize, Serialize)]
struct Xparam {
    name: String,
    r#type: Xtype
}
#[derive(Debug, Deserialize, Serialize)]
struct Xfunction {
    name: String,
    params: Vec<Xparam>,
    instructions: Vec<Xinstruction>
}
#[derive(Debug, Deserialize, Serialize)]
struct Xmodule {
    functions: Vec<Xfunction>,
}
fn main() {
    let input = String::from_utf8(std::fs::read("input.json").unwrap()).unwrap();
    let mut module = Module::new();
    let example_func = Xfunction{
        name: "example".to_string(),
        params: vec![Xparam{
            name: "lhs".to_string(),
            r#type: Xtype::I32
        },
        Xparam{
            name: "rhs".to_string(),
            r#type: Xtype::I32
        }],
        instructions: vec![Xinstruction::Add(Xvar::Local("lhs".into()), Xvar::Local("rhs".into())),Xinstruction::Add(Xvar::Local("lhs".into()), Xvar::Local("rhs".into()))],
    };

    let my_mod = Xmodule{
        functions: vec![example_func]
    };


    println!("my mod = {}", serde_lexpr::to_string(&my_mod).unwrap());
    let xmod = serde_json::from_str::<Xmodule>(&input).unwrap();
    for func in xmod.functions{
        let mut body = vec![];
        for instruction in func.instructions{
            match instruction{
                Xinstruction::Add(l, r) => {
                    let left = match l {
                        Xvar::Local(ll) => {
                            ll.parse().unwrap()
                        }
                    };
                    let right = match r {
                        Xvar::Local(rr) => {
                            rr.parse().unwrap()
                        }
                    };
                    let lhs = module.get_local(left, Type::int_32());
                    let rhs = module.get_local(right, Type::int_32());
                    
                    let added = module.binary(Op::add_int32(), lhs, rhs);


                    body.push(added)
                }
                Xinstruction::Sub(l, r) => {
                    let left = match l {
                        Xvar::Local(ll) => {
                            ll.parse().unwrap()
                        }
                    };
                    let right = match r {
                        Xvar::Local(rr) => {
                            rr.parse().unwrap()
                        }
                    };
                    let lhs = module.get_local(left, Type::int_32());
                    let rhs = module.get_local(right, Type::int_32());
                    
                    let subtracted = module.binary(Op::sub_int32(), lhs, rhs);


                    body.push(subtracted)
                }
            }
        }
        let blk = module.new_nameless_block(body, Type::int_32());
        module.add_function(&func.name, Type::none(), Type::int_32(), vec![], blk);
        
    }
    module.print();



    // let blk = module.new_nameless_block(body, Type::int_64());
    // module.add_function("main", Type::none(), Type::int_64(), vec![], blk);
    // assert!(module.validate());
    // module.print();
}
