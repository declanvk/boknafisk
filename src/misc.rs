// use alloc::heap::allocate;
// use std::mem::{align_of, size_of, transmute};
// use std::ptr::write_bytes;
// use std::boxed::Box;
// use std::slice::from_raw_parts_mut;

#[inline]
pub fn range<'a, T: Ord>(ref data: &'a Vec<T>) -> (Option<&T>, Option<&T>) {
    (data.iter().min(), data.iter().max())
}

// #[inline]
// pub fn make_array_unsafe<T>(len: usize) -> Box<[T]> {
//     let size: usize = len * size_of::<T>();
//     unsafe {
//         let mem_ptr: *mut T = transmute::<*mut u8, *mut T>(allocate(size, align_of::<T>()));
//         write_bytes(mem_ptr, 0, size);
//         Box::from_raw(from_raw_parts_mut(mem_ptr, len))
//     }
// }
