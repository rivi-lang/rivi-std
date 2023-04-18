mod lib_test;

use std::error::Error;

extern crate rspirv;
use rspirv::dr::Module;

pub fn index_generator() -> Result<Module, Box<dyn Error>> {
    let binary = &include_bytes!("spirv/iota/indexgen.spv")[..];
    let module = rspirv::dr::load_bytes(binary)?;
    Ok(module)
}

pub fn reduce() -> Result<Module, Box<dyn Error>> {
    let binary = &include_bytes!("spirv/reduce/vec4/reduce.spv")[..];
    let module = rspirv::dr::load_bytes(binary)?;
    Ok(module)
}
