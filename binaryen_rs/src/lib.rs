#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::rc::Rc;
use std::{convert::TryInto, ffi::CString};

use bindings::*;
mod bindings;
// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
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

pub struct Literal
{
    inner: BinaryenLiteral,
}
impl Literal
{
    pub fn new(l: BinaryenLiteral) -> Self
    {
        Self { inner: l }
    }

    pub fn int_32(x: i32) -> Literal
    {
        unsafe { Self::new(BinaryenLiteralInt32(x)) }
    }
    pub fn int_64(x: i64) -> Literal
    {
        unsafe { Self::new(BinaryenLiteralInt64(x)) }
    }
    pub fn float_32(x: f32) -> Literal
    {
        unsafe { Self::new(BinaryenLiteralFloat32(x)) }
    }
    pub fn float_64(x: f64) -> Literal
    {
        unsafe { Self::new(BinaryenLiteralFloat64(x)) }
    }
}
#[derive(Debug)]
pub struct Op
{
    inner: BinaryenOp,
}
macro_rules! binop {
    ($name: ident, $full: ident) => {
        pub fn $name() -> Op
        {
            unsafe { Op::new($full()) }
        }
    };
}
impl Op
{
    pub fn new(op: BinaryenOp) -> Self
    {
        return Self { inner: op };
    }
    // Potentially use macros for this?


    // binop!(sqrt_float_32, BinaryenSqrtFloat32);
    // binop!(clz_int_32, BinaryenClzInt32);
    // binop!(ctz_int_32, BinaryenCtzInt32);
    // binop!(pop_cnt_int_i32, BinaryenPopcntInt32);
    // binop!(neg_float_int_32, BinaryenNegFloat32);

    binop!(clz_int32, BinaryenClzInt32);
    binop!(ctz_int32, BinaryenCtzInt32);
    binop!(popcnt_int32, BinaryenPopcntInt32);
    binop!(neg_float32, BinaryenNegFloat32);
    binop!(abs_float32, BinaryenAbsFloat32);
    binop!(ceil_float32, BinaryenCeilFloat32);
    binop!(floor_float32, BinaryenFloorFloat32);
    binop!(trunc_float32, BinaryenTruncFloat32);
    binop!(nearest_float32, BinaryenNearestFloat32);
    binop!(sqrt_float32, BinaryenSqrtFloat32);
    binop!(eq_z_int32, BinaryenEqZInt32);
    binop!(clz_int64, BinaryenClzInt64);
    binop!(ctz_int64, BinaryenCtzInt64);
    binop!(popcnt_int64, BinaryenPopcntInt64);
    binop!(neg_float64, BinaryenNegFloat64);
    binop!(abs_float64, BinaryenAbsFloat64);
    binop!(ceil_float64, BinaryenCeilFloat64);
    binop!(floor_float64, BinaryenFloorFloat64);
    binop!(trunc_float64, BinaryenTruncFloat64);
    binop!(nearest_float64, BinaryenNearestFloat64);
    binop!(sqrt_float64, BinaryenSqrtFloat64);
    binop!(eq_z_int64, BinaryenEqZInt64);
    binop!(extend_s_int32, BinaryenExtendSInt32);
    binop!(extend_u_int32, BinaryenExtendUInt32);
    binop!(wrap_int64, BinaryenWrapInt64);
    binop!(trunc_s_float32_to_int32, BinaryenTruncSFloat32ToInt32);
    binop!(trunc_s_float32_to_int64, BinaryenTruncSFloat32ToInt64);
    binop!(trunc_u_float32_to_int32, BinaryenTruncUFloat32ToInt32);
    binop!(trunc_u_float32_to_int64, BinaryenTruncUFloat32ToInt64);
    binop!(trunc_s_float64_to_int32, BinaryenTruncSFloat64ToInt32);
    binop!(trunc_s_float64_to_int64, BinaryenTruncSFloat64ToInt64);
    binop!(trunc_u_float64_to_int32, BinaryenTruncUFloat64ToInt32);
    binop!(trunc_u_float64_to_int64, BinaryenTruncUFloat64ToInt64);
    binop!(reinterpret_float32, BinaryenReinterpretFloat32);
    binop!(reinterpret_float64, BinaryenReinterpretFloat64);
    binop!(convert_s_int32_to_float32, BinaryenConvertSInt32ToFloat32);
    binop!(convert_s_int32_to_float64, BinaryenConvertSInt32ToFloat64);
    binop!(convert_u_int32_to_float32, BinaryenConvertUInt32ToFloat32);
    binop!(convert_u_int32_to_float64, BinaryenConvertUInt32ToFloat64);
    binop!(convert_s_int64_to_float32, BinaryenConvertSInt64ToFloat32);
    binop!(convert_s_int64_to_float64, BinaryenConvertSInt64ToFloat64);
    binop!(convert_u_int64_to_float32, BinaryenConvertUInt64ToFloat32);
    binop!(convert_u_int64_to_float64, BinaryenConvertUInt64ToFloat64);
    binop!(promote_float32, BinaryenPromoteFloat32);
    binop!(demote_float64, BinaryenDemoteFloat64);
    binop!(reinterpret_int32, BinaryenReinterpretInt32);
    binop!(reinterpret_int64, BinaryenReinterpretInt64);
    binop!(extend_s8_int32, BinaryenExtendS8Int32);
    binop!(extend_s16_int32, BinaryenExtendS16Int32);
    binop!(extend_s8_int64, BinaryenExtendS8Int64);
    binop!(extend_s16_int64, BinaryenExtendS16Int64);
    binop!(extend_s32_int64, BinaryenExtendS32Int64);
    binop!(add_int32, BinaryenAddInt32);
    binop!(sub_int32, BinaryenSubInt32);
    binop!(mul_int32, BinaryenMulInt32);
    binop!(div_s_int32, BinaryenDivSInt32);
    binop!(div_u_int32, BinaryenDivUInt32);
    binop!(rem_s_int32, BinaryenRemSInt32);
    binop!(rem_u_int32, BinaryenRemUInt32);
    binop!(and_int32, BinaryenAndInt32);
    binop!(or_int32, BinaryenOrInt32);
    binop!(xor_int32, BinaryenXorInt32);
    binop!(shl_int32, BinaryenShlInt32);
    binop!(shr_u_int32, BinaryenShrUInt32);
    binop!(shr_s_int32, BinaryenShrSInt32);
    binop!(rot_l_int32, BinaryenRotLInt32);
    binop!(rot_r_int32, BinaryenRotRInt32);
    binop!(eq_int32, BinaryenEqInt32);
    binop!(ne_int32, BinaryenNeInt32);
    binop!(lt_s_int32, BinaryenLtSInt32);
    binop!(lt_u_int32, BinaryenLtUInt32);
    binop!(le_s_int32, BinaryenLeSInt32);
    binop!(le_u_int32, BinaryenLeUInt32);
    binop!(gt_s_int32, BinaryenGtSInt32);
    binop!(gt_u_int32, BinaryenGtUInt32);
    binop!(ge_s_int32, BinaryenGeSInt32);
    binop!(ge_u_int32, BinaryenGeUInt32);
    binop!(add_int64, BinaryenAddInt64);
    binop!(sub_int64, BinaryenSubInt64);
    binop!(mul_int64, BinaryenMulInt64);
    binop!(div_s_int64, BinaryenDivSInt64);
    binop!(div_u_int64, BinaryenDivUInt64);
    binop!(rem_s_int64, BinaryenRemSInt64);
    binop!(rem_u_int64, BinaryenRemUInt64);
    binop!(and_int64, BinaryenAndInt64);
    binop!(or_int64, BinaryenOrInt64);
    binop!(xor_int64, BinaryenXorInt64);
    binop!(shl_int64, BinaryenShlInt64);
    binop!(shr_u_int64, BinaryenShrUInt64);
    binop!(shr_s_int64, BinaryenShrSInt64);
    binop!(rot_l_int64, BinaryenRotLInt64);
    binop!(rot_r_int64, BinaryenRotRInt64);
    binop!(eq_int64, BinaryenEqInt64);
    binop!(ne_int64, BinaryenNeInt64);
    binop!(lt_s_int64, BinaryenLtSInt64);
    binop!(lt_u_int64, BinaryenLtUInt64);
    binop!(le_s_int64, BinaryenLeSInt64);
    binop!(le_u_int64, BinaryenLeUInt64);
    binop!(gt_s_int64, BinaryenGtSInt64);
    binop!(gt_u_int64, BinaryenGtUInt64);
    binop!(ge_s_int64, BinaryenGeSInt64);
    binop!(ge_u_int64, BinaryenGeUInt64);
    binop!(add_float32, BinaryenAddFloat32);
    binop!(sub_float32, BinaryenSubFloat32);
    binop!(mul_float32, BinaryenMulFloat32);
    binop!(div_float32, BinaryenDivFloat32);
    binop!(copy_sign_float32, BinaryenCopySignFloat32);
    binop!(min_float32, BinaryenMinFloat32);
    binop!(max_float32, BinaryenMaxFloat32);
    binop!(eq_float32, BinaryenEqFloat32);
    binop!(ne_float32, BinaryenNeFloat32);
    binop!(lt_float32, BinaryenLtFloat32);
    binop!(le_float32, BinaryenLeFloat32);
    binop!(gt_float32, BinaryenGtFloat32);
    binop!(ge_float32, BinaryenGeFloat32);
    binop!(add_float64, BinaryenAddFloat64);
    binop!(sub_float64, BinaryenSubFloat64);
    binop!(mul_float64, BinaryenMulFloat64);
    binop!(div_float64, BinaryenDivFloat64);
    binop!(copy_sign_float64, BinaryenCopySignFloat64);
    binop!(min_float64, BinaryenMinFloat64);
    binop!(max_float64, BinaryenMaxFloat64);
    binop!(eq_float64, BinaryenEqFloat64);
    binop!(ne_float64, BinaryenNeFloat64);
    binop!(lt_float64, BinaryenLtFloat64);
    binop!(le_float64, BinaryenLeFloat64);
    binop!(gt_float64, BinaryenGtFloat64);
    binop!(ge_float64, BinaryenGeFloat64);
    binop!(atomic_r_m_w_add, BinaryenAtomicRMWAdd);
    binop!(atomic_r_m_w_sub, BinaryenAtomicRMWSub);
    binop!(atomic_r_m_w_and, BinaryenAtomicRMWAnd);
    binop!(atomic_r_m_w_or, BinaryenAtomicRMWOr);
    binop!(atomic_r_m_w_xor, BinaryenAtomicRMWXor);
    binop!(atomic_r_m_w_xchg, BinaryenAtomicRMWXchg);
    binop!(
        trunc_sat_s_float32_to_int32,
        BinaryenTruncSatSFloat32ToInt32
    );
    binop!(
        trunc_sat_s_float32_to_int64,
        BinaryenTruncSatSFloat32ToInt64
    );
    binop!(
        trunc_sat_u_float32_to_int32,
        BinaryenTruncSatUFloat32ToInt32
    );
    binop!(
        trunc_sat_u_float32_to_int64,
        BinaryenTruncSatUFloat32ToInt64
    );
    binop!(
        trunc_sat_s_float64_to_int32,
        BinaryenTruncSatSFloat64ToInt32
    );
    binop!(
        trunc_sat_s_float64_to_int64,
        BinaryenTruncSatSFloat64ToInt64
    );
    binop!(
        trunc_sat_u_float64_to_int32,
        BinaryenTruncSatUFloat64ToInt32
    );
    binop!(
        trunc_sat_u_float64_to_int64,
        BinaryenTruncSatUFloat64ToInt64
    );
    binop!(splat_vec_i8x16, BinaryenSplatVecI8x16);
    binop!(extract_lane_s_vec_i8x16, BinaryenExtractLaneSVecI8x16);
    binop!(extract_lane_u_vec_i8x16, BinaryenExtractLaneUVecI8x16);
    binop!(replace_lane_vec_i8x16, BinaryenReplaceLaneVecI8x16);
    binop!(splat_vec_i16x8, BinaryenSplatVecI16x8);
    binop!(extract_lane_s_vec_i16x8, BinaryenExtractLaneSVecI16x8);
    binop!(extract_lane_u_vec_i16x8, BinaryenExtractLaneUVecI16x8);
    binop!(replace_lane_vec_i16x8, BinaryenReplaceLaneVecI16x8);
    binop!(splat_vec_i32x4, BinaryenSplatVecI32x4);
    binop!(extract_lane_vec_i32x4, BinaryenExtractLaneVecI32x4);
    binop!(replace_lane_vec_i32x4, BinaryenReplaceLaneVecI32x4);
    binop!(splat_vec_i64x2, BinaryenSplatVecI64x2);
    binop!(extract_lane_vec_i64x2, BinaryenExtractLaneVecI64x2);
    binop!(replace_lane_vec_i64x2, BinaryenReplaceLaneVecI64x2);
    binop!(splat_vec_f32x4, BinaryenSplatVecF32x4);
    binop!(extract_lane_vec_f32x4, BinaryenExtractLaneVecF32x4);
    binop!(replace_lane_vec_f32x4, BinaryenReplaceLaneVecF32x4);
    binop!(splat_vec_f64x2, BinaryenSplatVecF64x2);
    binop!(extract_lane_vec_f64x2, BinaryenExtractLaneVecF64x2);
    binop!(replace_lane_vec_f64x2, BinaryenReplaceLaneVecF64x2);
    binop!(eq_vec_i8x16, BinaryenEqVecI8x16);
    binop!(ne_vec_i8x16, BinaryenNeVecI8x16);
    binop!(lt_s_vec_i8x16, BinaryenLtSVecI8x16);
    binop!(lt_u_vec_i8x16, BinaryenLtUVecI8x16);
    binop!(gt_s_vec_i8x16, BinaryenGtSVecI8x16);
    binop!(gt_u_vec_i8x16, BinaryenGtUVecI8x16);
    binop!(le_s_vec_i8x16, BinaryenLeSVecI8x16);
    binop!(le_u_vec_i8x16, BinaryenLeUVecI8x16);
    binop!(ge_s_vec_i8x16, BinaryenGeSVecI8x16);
    binop!(ge_u_vec_i8x16, BinaryenGeUVecI8x16);
    binop!(eq_vec_i16x8, BinaryenEqVecI16x8);
    binop!(ne_vec_i16x8, BinaryenNeVecI16x8);
    binop!(lt_s_vec_i16x8, BinaryenLtSVecI16x8);
    binop!(lt_u_vec_i16x8, BinaryenLtUVecI16x8);
    binop!(gt_s_vec_i16x8, BinaryenGtSVecI16x8);
    binop!(gt_u_vec_i16x8, BinaryenGtUVecI16x8);
    binop!(le_s_vec_i16x8, BinaryenLeSVecI16x8);
    binop!(le_u_vec_i16x8, BinaryenLeUVecI16x8);
    binop!(ge_s_vec_i16x8, BinaryenGeSVecI16x8);
    binop!(ge_u_vec_i16x8, BinaryenGeUVecI16x8);
    binop!(eq_vec_i32x4, BinaryenEqVecI32x4);
    binop!(ne_vec_i32x4, BinaryenNeVecI32x4);
    binop!(lt_s_vec_i32x4, BinaryenLtSVecI32x4);
    binop!(lt_u_vec_i32x4, BinaryenLtUVecI32x4);
    binop!(gt_s_vec_i32x4, BinaryenGtSVecI32x4);
    binop!(gt_u_vec_i32x4, BinaryenGtUVecI32x4);
    binop!(le_s_vec_i32x4, BinaryenLeSVecI32x4);
    binop!(le_u_vec_i32x4, BinaryenLeUVecI32x4);
    binop!(ge_s_vec_i32x4, BinaryenGeSVecI32x4);
    binop!(ge_u_vec_i32x4, BinaryenGeUVecI32x4);
    binop!(eq_vec_f32x4, BinaryenEqVecF32x4);
    binop!(ne_vec_f32x4, BinaryenNeVecF32x4);
    binop!(lt_vec_f32x4, BinaryenLtVecF32x4);
    binop!(gt_vec_f32x4, BinaryenGtVecF32x4);
    binop!(le_vec_f32x4, BinaryenLeVecF32x4);
    binop!(ge_vec_f32x4, BinaryenGeVecF32x4);
    binop!(eq_vec_f64x2, BinaryenEqVecF64x2);
    binop!(ne_vec_f64x2, BinaryenNeVecF64x2);
    binop!(lt_vec_f64x2, BinaryenLtVecF64x2);
    binop!(gt_vec_f64x2, BinaryenGtVecF64x2);
    binop!(le_vec_f64x2, BinaryenLeVecF64x2);
    binop!(ge_vec_f64x2, BinaryenGeVecF64x2);
    binop!(not_vec128, BinaryenNotVec128);
    binop!(and_vec128, BinaryenAndVec128);
    binop!(or_vec128, BinaryenOrVec128);
    binop!(xor_vec128, BinaryenXorVec128);
    binop!(and_not_vec128, BinaryenAndNotVec128);
    binop!(bitselect_vec128, BinaryenBitselectVec128);
    binop!(abs_vec_i8x16, BinaryenAbsVecI8x16);
    binop!(neg_vec_i8x16, BinaryenNegVecI8x16);
    binop!(any_true_vec_i8x16, BinaryenAnyTrueVecI8x16);
    binop!(all_true_vec_i8x16, BinaryenAllTrueVecI8x16);
    binop!(bitmask_vec_i8x16, BinaryenBitmaskVecI8x16);
    binop!(shl_vec_i8x16, BinaryenShlVecI8x16);
    binop!(shr_s_vec_i8x16, BinaryenShrSVecI8x16);
    binop!(shr_u_vec_i8x16, BinaryenShrUVecI8x16);
    binop!(add_vec_i8x16, BinaryenAddVecI8x16);
    binop!(add_sat_s_vec_i8x16, BinaryenAddSatSVecI8x16);
    binop!(add_sat_u_vec_i8x16, BinaryenAddSatUVecI8x16);
    binop!(sub_vec_i8x16, BinaryenSubVecI8x16);
    binop!(sub_sat_s_vec_i8x16, BinaryenSubSatSVecI8x16);
    binop!(sub_sat_u_vec_i8x16, BinaryenSubSatUVecI8x16);
    binop!(mul_vec_i8x16, BinaryenMulVecI8x16);
    binop!(min_s_vec_i8x16, BinaryenMinSVecI8x16);
    binop!(min_u_vec_i8x16, BinaryenMinUVecI8x16);
    binop!(max_s_vec_i8x16, BinaryenMaxSVecI8x16);
    binop!(max_u_vec_i8x16, BinaryenMaxUVecI8x16);
    binop!(avgr_u_vec_i8x16, BinaryenAvgrUVecI8x16);
    binop!(abs_vec_i16x8, BinaryenAbsVecI16x8);
    binop!(neg_vec_i16x8, BinaryenNegVecI16x8);
    binop!(any_true_vec_i16x8, BinaryenAnyTrueVecI16x8);
    binop!(all_true_vec_i16x8, BinaryenAllTrueVecI16x8);
    binop!(bitmask_vec_i16x8, BinaryenBitmaskVecI16x8);
    binop!(shl_vec_i16x8, BinaryenShlVecI16x8);
    binop!(shr_s_vec_i16x8, BinaryenShrSVecI16x8);
    binop!(shr_u_vec_i16x8, BinaryenShrUVecI16x8);
    binop!(add_vec_i16x8, BinaryenAddVecI16x8);
    binop!(add_sat_s_vec_i16x8, BinaryenAddSatSVecI16x8);
    binop!(add_sat_u_vec_i16x8, BinaryenAddSatUVecI16x8);
    binop!(sub_vec_i16x8, BinaryenSubVecI16x8);
    binop!(sub_sat_s_vec_i16x8, BinaryenSubSatSVecI16x8);
    binop!(sub_sat_u_vec_i16x8, BinaryenSubSatUVecI16x8);
    binop!(mul_vec_i16x8, BinaryenMulVecI16x8);
    binop!(min_s_vec_i16x8, BinaryenMinSVecI16x8);
    binop!(min_u_vec_i16x8, BinaryenMinUVecI16x8);
    binop!(max_s_vec_i16x8, BinaryenMaxSVecI16x8);
    binop!(max_u_vec_i16x8, BinaryenMaxUVecI16x8);
    binop!(avgr_u_vec_i16x8, BinaryenAvgrUVecI16x8);
    binop!(abs_vec_i32x4, BinaryenAbsVecI32x4);
    binop!(neg_vec_i32x4, BinaryenNegVecI32x4);
    binop!(any_true_vec_i32x4, BinaryenAnyTrueVecI32x4);
    binop!(all_true_vec_i32x4, BinaryenAllTrueVecI32x4);
    binop!(bitmask_vec_i32x4, BinaryenBitmaskVecI32x4);
    binop!(shl_vec_i32x4, BinaryenShlVecI32x4);
    binop!(shr_s_vec_i32x4, BinaryenShrSVecI32x4);
    binop!(shr_u_vec_i32x4, BinaryenShrUVecI32x4);
    binop!(add_vec_i32x4, BinaryenAddVecI32x4);
    binop!(sub_vec_i32x4, BinaryenSubVecI32x4);
    binop!(mul_vec_i32x4, BinaryenMulVecI32x4);
    binop!(min_s_vec_i32x4, BinaryenMinSVecI32x4);
    binop!(min_u_vec_i32x4, BinaryenMinUVecI32x4);
    binop!(max_s_vec_i32x4, BinaryenMaxSVecI32x4);
    binop!(max_u_vec_i32x4, BinaryenMaxUVecI32x4);
    binop!(dot_s_vec_i16x8_to_vec_i32x4, BinaryenDotSVecI16x8ToVecI32x4);
    binop!(neg_vec_i64x2, BinaryenNegVecI64x2);
    binop!(any_true_vec_i64x2, BinaryenAnyTrueVecI64x2);
    binop!(all_true_vec_i64x2, BinaryenAllTrueVecI64x2);
    binop!(shl_vec_i64x2, BinaryenShlVecI64x2);
    binop!(shr_s_vec_i64x2, BinaryenShrSVecI64x2);
    binop!(shr_u_vec_i64x2, BinaryenShrUVecI64x2);
    binop!(add_vec_i64x2, BinaryenAddVecI64x2);
    binop!(sub_vec_i64x2, BinaryenSubVecI64x2);
    binop!(mul_vec_i64x2, BinaryenMulVecI64x2);
    binop!(abs_vec_f32x4, BinaryenAbsVecF32x4);
    binop!(neg_vec_f32x4, BinaryenNegVecF32x4);
    binop!(sqrt_vec_f32x4, BinaryenSqrtVecF32x4);
    binop!(q_f_m_a_vec_f32x4, BinaryenQFMAVecF32x4);
    binop!(q_f_m_s_vec_f32x4, BinaryenQFMSVecF32x4);
    binop!(add_vec_f32x4, BinaryenAddVecF32x4);
    binop!(sub_vec_f32x4, BinaryenSubVecF32x4);
    binop!(mul_vec_f32x4, BinaryenMulVecF32x4);
    binop!(div_vec_f32x4, BinaryenDivVecF32x4);
    binop!(min_vec_f32x4, BinaryenMinVecF32x4);
    binop!(max_vec_f32x4, BinaryenMaxVecF32x4);
    binop!(p_min_vec_f32x4, BinaryenPMinVecF32x4);
    binop!(p_max_vec_f32x4, BinaryenPMaxVecF32x4);
    binop!(ceil_vec_f32x4, BinaryenCeilVecF32x4);
    binop!(floor_vec_f32x4, BinaryenFloorVecF32x4);
    binop!(trunc_vec_f32x4, BinaryenTruncVecF32x4);
    binop!(nearest_vec_f32x4, BinaryenNearestVecF32x4);
    binop!(abs_vec_f64x2, BinaryenAbsVecF64x2);
    binop!(neg_vec_f64x2, BinaryenNegVecF64x2);
    binop!(sqrt_vec_f64x2, BinaryenSqrtVecF64x2);
    binop!(q_f_m_a_vec_f64x2, BinaryenQFMAVecF64x2);
    binop!(q_f_m_s_vec_f64x2, BinaryenQFMSVecF64x2);
    binop!(add_vec_f64x2, BinaryenAddVecF64x2);
    binop!(sub_vec_f64x2, BinaryenSubVecF64x2);
    binop!(mul_vec_f64x2, BinaryenMulVecF64x2);
    binop!(div_vec_f64x2, BinaryenDivVecF64x2);
    binop!(min_vec_f64x2, BinaryenMinVecF64x2);
    binop!(max_vec_f64x2, BinaryenMaxVecF64x2);
    binop!(p_min_vec_f64x2, BinaryenPMinVecF64x2);
    binop!(p_max_vec_f64x2, BinaryenPMaxVecF64x2);
    binop!(ceil_vec_f64x2, BinaryenCeilVecF64x2);
    binop!(floor_vec_f64x2, BinaryenFloorVecF64x2);
    binop!(trunc_vec_f64x2, BinaryenTruncVecF64x2);
    binop!(nearest_vec_f64x2, BinaryenNearestVecF64x2);
    binop!(
        trunc_sat_s_vec_f32x4_to_vec_i32x4,
        BinaryenTruncSatSVecF32x4ToVecI32x4
    );
    binop!(
        trunc_sat_u_vec_f32x4_to_vec_i32x4,
        BinaryenTruncSatUVecF32x4ToVecI32x4
    );
    binop!(
        trunc_sat_s_vec_f64x2_to_vec_i64x2,
        BinaryenTruncSatSVecF64x2ToVecI64x2
    );
    binop!(
        trunc_sat_u_vec_f64x2_to_vec_i64x2,
        BinaryenTruncSatUVecF64x2ToVecI64x2
    );
    binop!(
        convert_s_vec_i32x4_to_vec_f32x4,
        BinaryenConvertSVecI32x4ToVecF32x4
    );
    binop!(
        convert_u_vec_i32x4_to_vec_f32x4,
        BinaryenConvertUVecI32x4ToVecF32x4
    );
    binop!(
        convert_s_vec_i64x2_to_vec_f64x2,
        BinaryenConvertSVecI64x2ToVecF64x2
    );
    binop!(
        convert_u_vec_i64x2_to_vec_f64x2,
        BinaryenConvertUVecI64x2ToVecF64x2
    );
    binop!(load_splat_vec8x16, BinaryenLoadSplatVec8x16);
    binop!(load_splat_vec16x8, BinaryenLoadSplatVec16x8);
    binop!(load_splat_vec32x4, BinaryenLoadSplatVec32x4);
    binop!(load_splat_vec64x2, BinaryenLoadSplatVec64x2);
    binop!(
        load_ext_s_vec8x8_to_vec_i16x8,
        BinaryenLoadExtSVec8x8ToVecI16x8
    );
    binop!(
        load_ext_u_vec8x8_to_vec_i16x8,
        BinaryenLoadExtUVec8x8ToVecI16x8
    );
    binop!(
        load_ext_s_vec16x4_to_vec_i32x4,
        BinaryenLoadExtSVec16x4ToVecI32x4
    );
    binop!(
        load_ext_u_vec16x4_to_vec_i32x4,
        BinaryenLoadExtUVec16x4ToVecI32x4
    );
    binop!(
        load_ext_s_vec32x2_to_vec_i64x2,
        BinaryenLoadExtSVec32x2ToVecI64x2
    );
    binop!(
        load_ext_u_vec32x2_to_vec_i64x2,
        BinaryenLoadExtUVec32x2ToVecI64x2
    );
    binop!(
        narrow_s_vec_i16x8_to_vec_i8x16,
        BinaryenNarrowSVecI16x8ToVecI8x16
    );
    binop!(
        narrow_u_vec_i16x8_to_vec_i8x16,
        BinaryenNarrowUVecI16x8ToVecI8x16
    );
    binop!(
        narrow_s_vec_i32x4_to_vec_i16x8,
        BinaryenNarrowSVecI32x4ToVecI16x8
    );
    binop!(
        narrow_u_vec_i32x4_to_vec_i16x8,
        BinaryenNarrowUVecI32x4ToVecI16x8
    );
    binop!(
        widen_low_s_vec_i8x16_to_vec_i16x8,
        BinaryenWidenLowSVecI8x16ToVecI16x8
    );
    binop!(
        widen_high_s_vec_i8x16_to_vec_i16x8,
        BinaryenWidenHighSVecI8x16ToVecI16x8
    );
    binop!(
        widen_low_u_vec_i8x16_to_vec_i16x8,
        BinaryenWidenLowUVecI8x16ToVecI16x8
    );
    binop!(
        widen_high_u_vec_i8x16_to_vec_i16x8,
        BinaryenWidenHighUVecI8x16ToVecI16x8
    );
    binop!(
        widen_low_s_vec_i16x8_to_vec_i32x4,
        BinaryenWidenLowSVecI16x8ToVecI32x4
    );
    binop!(
        widen_high_s_vec_i16x8_to_vec_i32x4,
        BinaryenWidenHighSVecI16x8ToVecI32x4
    );
    binop!(
        widen_low_u_vec_i16x8_to_vec_i32x4,
        BinaryenWidenLowUVecI16x8ToVecI32x4
    );
    binop!(
        widen_high_u_vec_i16x8_to_vec_i32x4,
        BinaryenWidenHighUVecI16x8ToVecI32x4
    );
    binop!(swizzle_vec8x16, BinaryenSwizzleVec8x16);
}
#[derive(Debug)]
pub struct ExpressionRef
{
    inner: BinaryenExpressionRef,
}
impl ExpressionRef
{
    pub fn new(expr: BinaryenExpressionRef) -> Self
    {
        return ExpressionRef { inner: expr };
    }
}
#[derive(Debug)]
pub struct Export
{
    inner: *mut BinaryenExport,
}
impl Export
{
    fn new(expr: *mut BinaryenExport) -> Self
    {
        return Self { inner: expr };
    }
}
unsafe impl Send for Export{}
unsafe impl Sync for Export{}
#[derive(Debug)]
pub struct Module
{
    inner: BinaryenModuleRef,
}
unsafe impl Send for Module{}
unsafe impl Sync for Module{}
impl Module
{
    pub fn new() -> Self
    {
        return unsafe {
            Self {
                inner: BinaryenModuleCreate(),
            }
        };
    }
    pub fn print(&mut self)
    {
        unsafe { BinaryenModulePrint(self.inner) }
    }
    pub fn print_wat(&mut self)
    {
        unsafe { BinaryenModulePrint(self.inner) }
    }
    pub fn print_asmjs(&mut self)
    {
        unsafe { BinaryenModulePrintAsmjs(self.inner) }
    }
    pub fn get_local(&mut self, index: i32, dype: Type) -> ExpressionRef
    {
        unsafe {
            ExpressionRef::new(BinaryenLocalGet(
                self.inner,
                index.try_into().unwrap(),
                dype.inner,
            ))
        }
    }
    pub fn set_local(&mut self, index: i32, value: ExpressionRef) -> ExpressionRef
    {
        unsafe {
            ExpressionRef::new(BinaryenLocalSet(
                self.inner,
                index.try_into().unwrap(),
                value.inner,
            ))
        }
    }
    /*
    Create new binary operation
    */
    pub fn binary(&mut self, op: Op, left: ExpressionRef, right: ExpressionRef) -> ExpressionRef
    {
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
    )
    {
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
    /// Bool whether the validation was successful
    pub fn validate(&mut self) -> bool
    {
        return unsafe { BinaryenModuleValidate(self.inner) == 1 };
    }
    /// Incase you want to have the raw validation number.
    pub fn validate_i(&mut self) -> i32
    {
        return unsafe { BinaryenModuleValidate(self.inner) };
    }
    /// Optimise
    pub fn optimize(&mut self)
    {
        unsafe { BinaryenModuleOptimize(self.inner) }
    }
    #[doc = "Get current optimization level, set new optimization `level`, optimize, set back to original optimization level."]
    pub fn optimize_with_level(&mut self, level: i32)
    {
        let current_level = unsafe { BinaryenGetOptimizeLevel() };
        unsafe { BinaryenSetOptimizeLevel(level) }
        unsafe { BinaryenModuleOptimize(self.inner) }
        unsafe { BinaryenSetOptimizeLevel(current_level) }
    }
    pub fn make_const(&mut self, value: Literal) -> ExpressionRef
    {
        ExpressionRef::new(unsafe { BinaryenConst(self.inner, value.inner) })
    }

    pub fn new_nameless_block(&mut self, children: Vec<ExpressionRef>, type_: Type)
        -> ExpressionRef
    {
        let mut inners = children
            .iter()
            .map(|t| t.inner)
            .collect::<Vec<BinaryenExpressionRef>>();
        ExpressionRef::new(unsafe {
            BinaryenBlock(
                self.inner,
                std::ptr::null(),
                inners.as_mut_ptr(),
                inners.len().try_into().unwrap(),
                type_.inner,
            )
        })
    }

    pub fn new_block(
        &mut self,
        name: &str,
        children: Vec<ExpressionRef>,
        type_: Type,
    ) -> ExpressionRef
    {
        let mut inners = children
            .iter()
            .map(|t| t.inner)
            .collect::<Vec<BinaryenExpressionRef>>();
        ExpressionRef::new(unsafe {
            BinaryenBlock(
                self.inner,
                CString::new(name).unwrap().as_ptr(),
                inners.as_mut_ptr(),
                inners.len().try_into().unwrap(),
                type_.inner,
            )
        })
    }
    pub fn add_export(&mut self, internal_name: &str, external_name: &str) -> Export
    {
        let c_internal_name = CString::new(internal_name.to_string()).unwrap();
        let c_external_name = CString::new(external_name.to_string()).unwrap();
        return Export::new(unsafe {
            BinaryenAddExport(
                self.inner,
                c_internal_name.as_ptr(),
                c_external_name.as_ptr(),
            )
        });
    }
    #[deprecated = "Use add_export while I try to fix this"]
    pub fn add_function_export(&mut self, internal_name: &str, external_name: &str) -> Export
    {
        let c_internal_name = CString::new(internal_name).unwrap().as_ptr();
        let c_external_name = CString::new(external_name).unwrap().as_ptr();

        return Export::new(unsafe {
            BinaryenAddFunctionExport(self.inner, c_internal_name, c_external_name)
        });
    }
    pub fn add_table_export(&mut self, internal_name: &str, external_name: &str) -> Export
    {
        let c_internal_name = CString::new(internal_name).unwrap().as_ptr();
        let c_external_name = CString::new(external_name).unwrap().as_ptr();

        return Export::new(unsafe {
            BinaryenAddTableExport(self.inner, c_internal_name, c_external_name)
        });
    }
    pub fn add_memory_export(&mut self, internal_name: &str, external_name: &str) -> Export
    {
        let c_internal_name = CString::new(internal_name).unwrap().as_ptr();
        let c_external_name = CString::new(external_name).unwrap().as_ptr();

        return Export::new(unsafe {
            BinaryenAddMemoryExport(self.inner, c_internal_name, c_external_name)
        });
    }
    //TODO: Fix
    
    // pub fn write(&mut self, filename: &str)
    // {
    //     let c = unsafe {
    //         let was_color_originally_enabled = BinaryenAreColorsEnabled();
    //         BinaryenSetColorsEnabled(0);
    //         let result = 
    //             BinaryenModuleAllocateAndWrite(self.inner, std::ptr::null_mut());
    //         BinaryenSetColorsEnabled(was_color_originally_enabled);
    //         // result
    //         // std::ffi::CStr::from_ptr(result.binary as  *const i8)
    //         // result.binaryBytes
    //         result.binary.as_ref().unwrap()
    //     };
    //     println!("{:?}", c);
    //     // std::fs::write(filename, c.to_string_lossy().to_string()).unwrap();
    // }
    pub fn write_text(&mut self, filename: &str)
    {
        let c = unsafe {
            let was_color_originally_enabled = BinaryenAreColorsEnabled();
            BinaryenSetColorsEnabled(0);
            let c: *mut ::std::os::raw::c_char = BinaryenModuleAllocateAndWriteText(self.inner);
            BinaryenSetColorsEnabled(was_color_originally_enabled);

            std::ffi::CStr::from_ptr(c)
        };
        std::fs::write(filename, c.to_string_lossy().to_string()).unwrap();
    }
}
impl Drop for Module
{
    fn drop(&mut self)
    {
        unsafe { BinaryenModuleDispose(self.inner) }
    }
}
// pub struct ModuleRef{
//     inner: BinaryenModuleRef
// }
// impl ModuleRef{
//     fn new(x: BinaryenModuleRef) -> Self{
//         Self{inner: x}
//     }
// }
#[derive(Debug)]
pub struct Type
{
    inner: BinaryenType,
}
impl Type
{
    pub fn int_32() -> Self
    {
        return Self {
            inner: { unsafe { BinaryenTypeInt32() } },
        };
    }
    pub fn int_64() -> Self
    {
        return Self {
            inner: { unsafe { BinaryenInt64() } },
        };
    }
    pub fn float_32() -> Self
    {
        return Self {
            inner: { unsafe { BinaryenFloat32() } },
        };
    }
    pub fn float_64() -> Self
    {
        return Self {
            inner: { unsafe { BinaryenFloat64() } },
        };
    }
    pub fn none() -> Self
    {
        return Self {
            inner: { unsafe { BinaryenNone() } },
        };
    }
    pub fn create(value_types: Vec<Type>) -> Self
    {
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
