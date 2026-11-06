#[repr(C)]
struct S {
    a: u8,  // 1 byte -  align 1
    b: u32, // 4 bytes - align 4
}

#[repr(C, packed)]
struct SPacked {
    a: u8,  // 1 byte -  align 1
    b: u32, // 4 bytes - align 4, but packed => no padding
}

struct SNoRepr {
    a: u8,
    b: u32,
}

fn show_s() {
    println!("--- S repr(C) ---");
    let s_align = std::mem::align_of::<S>();
    /*
    align_of::<u8> = 1
    align_of::<u32> = 4

    align_of::<S> = max(1,4)
    max alignment of its fields. Because b: u32 needs align 4, whole struct needs align 4.
    */
    println!("Align of S is: {s_align}");

    let s_size = std::mem::size_of::<S>();
    /*
    Size 8 now. Lay out fields in repr(C) order (so a first, b second):

    offset 0: [ a ]        <- u8, 1 byte, can be anywhere
    offset 1: [ _ ]        <- padding (gap)
    offset 2: [ _ ]        <- padding
    offset 3: [ _ ]        <- padding
    offset 4: [ b ][ b ][ b ][ b ] <- u32, must start at multiple of 4


    b must start at 0, 4, 8... After a ends at offset 1,
    the next legal start for b is 4.
    So compiler inserts 3 padding bytes at offsets 1,2,3.
    Size = 1 (a) + 3 (padding) + 4 (b) = 8
    */
    println!("Size of S is: {s_size}");
    println!(
        "offsets S - a: {}, b: {}",
        std::mem::offset_of!(S, a),
        std::mem::offset_of!(S, b)
    );
    println!("****");
    println!();
}

fn show_spacked() {
    println!("--- packed version ---");
    let sp_align = std::mem::align_of::<SPacked>();
    /*
    align_of::<u8> = 1
    align_of::<u32> = 4 normally, but #[repr(C, packed)] forces
    largest field align down to 1, and removes all padding.

    align_of::<SPacked> = 1
    */
    println!("Align of SPacked is: {sp_align}");

    let sp_size = std::mem::size_of::<SPacked>();
    /*
    Size 5 now. Layout for SPacked:

    offset 0: [ a ]        <- u8, 1 byte, can be anywhere
    offset 1: [ b ][ b ][ b ][ b ] <- u32 at offset 1, unaligned!

    No padding bytes. b is allowed to be unaligned (but you cannot take &u32 to it).
    Size = 1 (a) + 4 (b) = 5
    */
    println!("Size of SPacked is: {sp_size}");
    println!(
        "offsets SPacked - a: {}, b: {}",
        std::mem::offset_of!(SPacked, a),
        std::mem::offset_of!(SPacked, b)
    );
    println!("****");
    println!();
}

fn show_s_no_repr() {
    println!("--- no repr (layout unspecified) ---");
    let snr_align = std::mem::align_of::<SNoRepr>();
    /*
    Same fields a: u8, b: u32.
    align is still 4 (max of fields), but ORDER is not guaranteed.
    Compiler may reorder fields to save space.
    Example: it may put b first at offset 0, a second at offset 4.
    You cannot assume offsets = declaration order.
    */
    println!("Align of SNoRepr is: {snr_align}");

    let snr_size = std::mem::size_of::<SNoRepr>();
    println!("Size of SNoRepr is: {snr_size}");

    println!(
        "offsets SNoRepr - a: {}, b: {}",
        std::mem::offset_of!(SNoRepr, a),
        std::mem::offset_of!(SNoRepr, b)
    );
    println!("****");
    println!();
}

fn packed_is_dangerous() {
    println!("--- packed danger demo (copy, not reference) ---");
    let sp: SPacked = SPacked {
        a: 7,
        b: 0x11223344,
    };

    /*
    reference to field of packed struct is unaligned
    this struct is 1-byte aligned, but the type of this field may require higher alignment
    creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
    copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)rustcClick for full compiler diagnostic
    */
    //let r = &sp.b; // uncomment -> E0793 error
    let copy = sp.b;
    println!("{copy:x}");
    // decimal
    println!("{copy:x}");
    println!("****");
    println!();
}
fn main() {
    println!("=== alignment padding demo ===");
    show_s();
    show_spacked();
    packed_is_dangerous();
    show_s_no_repr();
}
