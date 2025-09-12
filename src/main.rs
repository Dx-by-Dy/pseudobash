use {
    pseudobash::{executor::Executor, pipeline::Pipeline, program::Program},
    std::ffi::CString,
};

fn main() {
    let pipeline = Pipeline::new(vec![
        // Program::new(
        //     CString::new("/home/none/Rust_test/target/release/Rust_test").unwrap(),
        //     vec![],
        //     false,
        // ),
        Program::new(
            CString::new("/usr/bin/bash").unwrap(),
            vec![
                CString::new("-c").unwrap(),
                CString::new("echo 120").unwrap(),
            ],
            false,
        ),
    ]);

    match unsafe { Executor::execute_pipeline_linear(pipeline) } {
        Ok(result) => print!("{}", result.to_string_lossy()),
        Err(e) => eprintln!("{}", e),
    }

    // let rt: Vec<u8> = vec![
    //     CString::new("/usr/bin/bash").unwrap(),
    //     CString::new("-c").unwrap(),
    //     CString::new("echo 120").unwrap(),
    // ]
    // .into_iter()
    // .flat_map(|cstring| cstring.into_bytes_with_nul())
    // .collect();

    // let mut rt_ptr: Vec<*const i8> = Vec::with_capacity(rt.len() + 1);

    // let mut last_byte: u8 = 0;
    // for idx in 0..rt.len() {
    //     if last_byte == 0 {
    //         rt_ptr.push(&rt[idx] as *const u8 as *const i8);
    //     }
    //     last_byte = rt[idx];
    // }

    // //String::new().push_str("\0");
    // rt_ptr.push(ptr::null());

    // let env_ptr: Vec<*const i8> = vec![ptr::null()];

    // match unsafe { execve(rt_ptr[0], rt_ptr.as_ptr(), env_ptr.as_ptr()) } {
    //     -1 => {
    //         println!("error: {}", unsafe { *libc::__errno_location() })
    //     }
    //     _ => {}
    // }

    //et rt_ptr: Vec<*const i8> = rt.iter().map(|bytes| )
}
