#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::rc::Rc;
use std::{convert::TryInto, ffi::CString};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
// #[derive(Debug)]
// pub struct Local {
//     module: Rc<Module>,
// }

// impl Local {
//     pub fn get(&self, index: i32, dype: Type) -> BinaryenExpressionRef {
//         BinaryenLocalGet(self.module.inner, index.try_into().unwrap(), dype.inner)
//     }
// }

#[derive(Debug)]
pub struct Op {
    inner: BinaryenOp,
}
impl Op {
    pub fn new(op: BinaryenOp) -> Self {
        return Self { inner: op };
    }
    pub fn add_int_32() -> Self {
        return unsafe { Self::new(BinaryenAddInt32()) };
    }
}
#[derive(Debug)]
pub struct ExpressionRef {
    inner: BinaryenExpressionRef,
}
impl ExpressionRef {
    pub fn new(expr: BinaryenExpressionRef) -> Self {
        return ExpressionRef { inner: expr };
    }
}
#[derive(Debug)]
pub struct Module {
    inner: *mut BinaryenModule,
}
impl Module {
    pub fn new() -> Self {
        return unsafe {
            Module {
                inner: BinaryenModuleCreate(),
            }
        };
    }
    pub fn print(&mut self) {
        unsafe { BinaryenModulePrint(self.inner) }
    }
    pub fn get_local(&mut self, index: i32, dype: Type) -> ExpressionRef {
        unsafe {
            ExpressionRef::new(BinaryenLocalGet(
                self.inner,
                index.try_into().unwrap(),
                dype.inner,
            ))
        }
    }
    pub fn binary(&mut self, op: Op, left: ExpressionRef, right: ExpressionRef) -> ExpressionRef {
        return unsafe {
            ExpressionRef::new(BinaryenBinary(
                self.inner,
                op.inner,
                left.inner,
                right.inner,
            ))
        };
    }

    pub fn add_function(
        &mut self,
        name: &str,
        params: Type,
        results: Type,
        var_types: Vec<Type>,
        body: ExpressionRef,
    ) {
        let mut inners = var_types
        .iter()
        .map(|t| t.inner)
        .collect::<Vec<BinaryenType>>();
        unsafe {
            BinaryenAddFunction(
                self.inner,
                CString::new(name).unwrap().as_ptr(),
                params.inner,
                results.inner,
                inners.as_mut_ptr(),
                var_types.len().try_into().unwrap(),
                body.inner,
            );
        }
    }
}
#[derive(Debug)]
pub struct Type {
    inner: BinaryenType,
}
impl Type {
    pub fn int_32() -> Self {
        return Self {
            inner: { unsafe { BinaryenTypeInt32() } },
        };
    }
    pub fn int_64() -> Self {
        return Self {
            inner: { unsafe { BinaryenInt64() } },
        };
    }
    pub fn float_32() -> Self {
        return Self {
            inner: { unsafe { BinaryenFloat32() } },
        };
    }
    pub fn float_64() -> Self {
        return Self {
            inner: { unsafe { BinaryenFloat64() } },
        };
    }
    pub fn create(value_types: Vec<Type>) -> Self {
        return unsafe {
            let mut inners = value_types
                .iter()
                .map(|t| t.inner)
                .collect::<Vec<BinaryenType>>();
            Self {
                inner: BinaryenTypeCreate(
                    inners.as_mut_ptr(),
                    value_types.len().try_into().unwrap(),
                ),
            }
        };
    }
}

// fn main() {
// unsafe {
//     let module = BinaryenModuleCreate();

//     let mut ii = vec![BinaryenTypeInt32(), BinaryenTypeInt32()];
//     let params = BinaryenTypeCreate(ii.as_mut_ptr(), 2);
//     let result = BinaryenTypeInt32();
//     let i = BinaryenConst(module, BinaryenLiteralInt32(0));
//     let ptr = BinaryenLoad(module, 4, 0, 0, 0, BinaryenTypeInt32(), i);
//     let mut list = vec![ptr];
//     let blk = BinaryenBlock(module, 0 as *const i8, list.as_mut_ptr(), 1, result);
//     let func = BinaryenAddFunction(
//         module,
//         CString::new("main").unwrap().as_ptr(),
//         params,
//         result,
//         std::ptr::null_mut(),
//         0,
//         blk,
//     );
//     BinaryenModulePrint(module);
//     println!("module = {:?}", module);
// }
// println!("Hello, world!");
// }
