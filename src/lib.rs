#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    // use std::ffi::CString;
    // #[test]
    // fn cgns() {
    //     unsafe {
    //         let path = CString::new("/home/saito/Downloads/bump.cgns").expect("path");
    //         let mut fd: i32 = 0;
    //         let f = cg_open(path.as_ptr(), 0, &mut fd);
    //         let mut status: f32 = 0.0;
    //         let _ = cg_version(0, &mut status);
    //     }
    // }
}
