// Brave: upstream's build.rs regenerates protobuf bindings for the `proto`
// module, which this trimmed crate does not compile (only `serialization` is
// kept). Left as a no-op rather than deleted so the file continues to line up
// 1:1 with upstream's tree.
fn main() {}
