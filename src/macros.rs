// Copyright © 2016 IronNet Cybersecurity, Inc. All rights reserved.
//
// Unauthorized copying of this file, via any medium, is strictly prohibited.
//
// PROPRIETARY AND CONFIDENTIAL
//




#[cfg(debug_assertions)]
macro_rules! compile_debug {
    ($e:expr) => {
        $e;
    }
}

#[cfg(not(debug_assertions))]
macro_rules! compile_debug {
    ($e:expr) => {
    }
}

macro_rules! fill_vec {
    ($count: expr, $fill: expr) => {{
        let mut v = Vec::with_capacity($count);
        for index in 0..$count {
            v.push($fill(index));
        }
        v
    }}
}

macro_rules! name_thread {
    ($label: expr) => {{
        unsafe {
            let s = ::std::ffi::CString::new($label).unwrap();
            ::libc::prctl(15, s.as_ptr(), 0, 0, 0);
        }
    }}
}
