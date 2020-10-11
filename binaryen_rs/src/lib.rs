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
// #[derive(Debug)]

pub struct Literal{
    inner: BinaryenLiteral
}
#[derive(Debug)]
pub struct Op {
    inner: BinaryenOp,
}
impl Op {
    pub fn new(op: BinaryenOp) -> Self {
        return Self { inner: op };
    }
    // Potentially use macros for this?

    pub fn add_int_32() -> Self {
        return unsafe { Self::new(BinaryenAddInt32()) };
    }
    pub fn add_int_64() -> Self {
        return unsafe { Self::new(BinaryenAddInt64()) };
    }
    pub fn add_float_32() -> Self {
        return unsafe { Self::new(BinaryenAddFloat32()) };
    }
    pub fn add_float_64() -> Self {
        return unsafe { Self::new(BinaryenAddFloat64()) };
    }

    pub fn sub_int_32() -> Self {
        return unsafe { Self::new(BinaryenSubInt32()) };
    }
    pub fn sub_int_64() -> Self {
        return unsafe { Self::new(BinaryenSubInt64()) };
    }
    pub fn sub_float_32() -> Self {
        return unsafe { Self::new(BinaryenSubFloat32()) };
    }
    pub fn sub_float_64() -> Self {
        return unsafe { Self::new(BinaryenSubFloat64()) };
    }

    pub fn mul_int_32() -> Self {
        return unsafe { Self::new(BinaryenMulInt32()) };
    }
    pub fn mul_int_64() -> Self {
        return unsafe { Self::new(BinaryenMulInt64()) };
    }
    pub fn mul_float_32() -> Self {
        return unsafe { Self::new(BinaryenMulFloat32()) };
    }
    pub fn mul_float_64() -> Self {
        return unsafe { Self::new(BinaryenMulFloat64()) };
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
    pub fn print_wat(&mut self) {
        unsafe { BinaryenModulePrint(self.inner) }
    }
    pub fn print_asmjs(&mut self) {
        unsafe { BinaryenModulePrintAsmjs(self.inner) }
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
    /*
    Create new binary operation
    */
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
    // Bool whether the validation was successful
    pub fn validate(&mut self) -> bool {
        return unsafe { BinaryenModuleValidate(self.inner) == 1 };
    }
    // Incase you want to have the raw validation number.
    pub fn validate_i(&mut self) -> i32 {
        return unsafe { BinaryenModuleValidate(self.inner) };
    }
    // Optimis
    pub fn optimize(&mut self) {
        unsafe { BinaryenModuleOptimize(self.inner) }
    }
}
impl Drop for Module{
    fn drop(&mut self) {
        unsafe {
            BinaryenModuleDispose(self.inner)
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
