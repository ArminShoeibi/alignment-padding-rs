# alignment-padding-rs

This is a small practice project for exploring Rust struct alignment, padding, and field offsets. It compares three layouts for the same fields — `#[repr(C)]` with padding, `#[repr(C, packed)]` without padding, and no `#[repr]` with unspecified reordering — using `std::mem::size_of`, `align_of`, and `offset_of` to make the results visible.
