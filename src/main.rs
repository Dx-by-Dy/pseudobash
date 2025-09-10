use {pseudobash::program::Program, std::ffi::CString};

fn main() {
    let captures = vec![
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
    ];

    let mut threads = Vec::new();

    for capture in captures {
        threads.push(capture.execute_in_thread());
    }

    for (i, thread) in threads.into_iter().enumerate() {
        match thread.join() {
            Ok(Ok(output)) => println!("Thread {}:\n{:?}", i, CString::new(output)),
            Ok(Err(e)) => eprintln!("Thread {} execution error: \n{}", i, e),
            Err(_) => eprintln!("Thread {} killed", i),
        }
    }
}
