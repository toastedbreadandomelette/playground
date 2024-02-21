// use crate::utils::c64::C64;
// use core::arch::x86_64::*;
// use core::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
//
// #[derive(Debug, Clone, Copy)]
// pub struct C64x4(__m512d);
//
// impl C64x4 {
//     #[inline(always)]
//     pub fn from_slice(val: &[C64]) -> Self {
//         unsafe { Self(_mm512_loadu_pd(val.as_ptr() as *const f64)) }
//     }
//
//     #[inline(always)]
//     pub fn splat(val: C64) -> Self {
//         unsafe { Self(_mm512_set_pd(val.img, val.real, val.img, val.real, val.img, val.real, val.img, val.real)) }
//     }
//
//     #[inline(always)]
//     pub fn from_array([first, second, third, fourth]: [C64; 4]) -> Self {
//         unsafe {
//             Self(_mm512_set_pd(
//                 fourth.img,
//                 fourth.real,
//                 third.img,
//                 third.real,
//                 second.img,
//                 second.real,
//                 first.img,
//                 first.real,
//             ))
//         }
//     }
//
//     #[inline(always)]
//     pub fn scalar_mul(&self, val: f64) -> Self {
//         unsafe { Self(_mm512_mul_pd(self.0, _mm512_set1_pd(val))) }
//     }
//
//     #[inline(always)]
//     pub fn scalar_mul_vec(&self, val: C64x4) -> Self {
//         unsafe { Self(_mm512_mul_pd(self.0, val.0)) }
//     }
//
//     #[inline(always)]
//     pub fn scalar_mul_vec_self(&mut self, val: C64x4) {
//         unsafe { self.0 = _mm512_mul_pd(self.0, val.0) }
//     }
//
//     #[inline(always)]
//     pub fn copy_to_slice(&self, val: &mut [C64]) {
//         unsafe { _mm512_storeu_pd(val.as_mut_ptr() as *mut f64, self.0) }
//     }
// }
//
// impl Add for C64x4 {
//     type Output = Self;
//     #[inline(always)]
//     fn add(self, rhs: Self) -> Self::Output {
//         unsafe { Self(_mm512_add_pd(self.0, rhs.0)) }
//     }
// }
//
// impl AddAssign for C64x4 {
//     #[inline(always)]
//     fn add_assign(&mut self, rhs: Self) {
//         unsafe { self.0 = _mm512_add_pd(self.0, rhs.0) }
//     }
// }
//
// impl Sub for C64x4 {
//     type Output = Self;
//     #[inline(always)]
//     fn sub(self, rhs: Self) -> Self::Output {
//         unsafe { Self(_mm512_sub_pd(self.0, rhs.0)) }
//     }
// }
//
// impl SubAssign for C64x4 {
//     #[inline(always)]
//     fn sub_assign(&mut self, rhs: Self) {
//         unsafe { self.0 = _mm512_sub_pd(self.0, rhs.0) }
//     }
// }
//
// impl Mul for C64x4 {
//     type Output = Self;
//
//     #[inline(always)]
//     fn mul(self, rhs: Self) -> Self::Output {
//         unsafe {
//             let real = _mm512_mul_pd(self.0, rhs.0);
//             let img = _mm512_mul_pd(self.0, _mm512_permute_pd(rhs.0, 0b01010101));
//             let realf = _mm512_sub_pd(real, _mm512_permute_pd(real, 0b10101010));
//             let imgf = _mm512_add_pd(img, _mm512_permute_pd(img, 0b10101010));
//             Self(_mm512_mask_blend_pd(0b10101010, realf, imgf))
//         }
//     }
// }
//
// impl MulAssign for C64x4 {
//     #[inline(always)]
//     fn mul_assign(&mut self, rhs: Self) {
//         unsafe {
//             let real = _mm512_mul_pd(self.0, rhs.0);
//             let img = _mm512_mul_pd(self.0, _mm512_permute_pd(rhs.0, 0b01010101));
//             let realf = _mm512_sub_pd(real, _mm512_permute_pd(real, 0b10101010));
//             let imgf = _mm512_add_pd(img, _mm512_permute_pd(img, 0b10101010));
//             *self = Self(_mm512_mask_blend_pd(0b10101010, realf, imgf))
//         }
//     }
// }
