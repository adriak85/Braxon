pub fn braxon_build_tool_probe(x: u64) -> u64 {
    (x * 37) ^ 0xBADC0FFEE
}

fn main() {
    let _ = braxon_build_tool_probe(7);
}
