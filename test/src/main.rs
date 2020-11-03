use binaryen_rs::*;
fn main() {
    let mut module = Module::new();

    {
        let test = Type::i31ref();

        let left = module.get_local(0, Type::i31ref());
        let right = module.get_local(0, Type::i31ref());
        let adder = module.binary(Op::add_int32(), left, right);
        let children = vec![adder];
        let blk = module.new_block("FuncBody", children, Type::i31ref());
        module.add_function("add", Type::create(vec![Type::i31ref(), Type::i31ref()]), Type::i31ref(), vec![], blk);
    }


    module.validate();
    module.print_wat();
}
