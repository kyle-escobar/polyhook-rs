use polyhook2::*;

fn main() {
    let disasm = unsafe { PLH::CapstoneDisassembler::new(PLH::Mode_x64) };
}