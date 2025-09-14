use {
    pseudobash::{executor::Executor, pipeline::Pipeline},
    std::ffi::CString,
};

fn main() {
    //let pipeline = Pipeline::try_from(CString::new("echo 120 | echo 130").unwrap()).unwrap();
    //let pipeline = Pipeline::try_from(CString::new("bash").unwrap()).unwrap();
    let pipeline = Pipeline::try_from(CString::new("echo on | ive | bash").unwrap()).unwrap();

    match unsafe { Executor::execute_pipeline_linear(pipeline) } {
        Ok(result) => println!("{}", String::from_utf8_lossy(&result)),
        Err(e) => eprintln!("{}", e),
    }
}
