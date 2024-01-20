fn main() {
    println!("main no defined entry");
    let refstr = String::from("GUUD");
    let mstr = refstr.as_str();
    let slistr = &refstr[..];
    let pstr: &str = refstr.as_ref();
    assert_eq!(mstr, refstr);
    let ptr = [
        refstr.as_ptr(),
        mstr.as_ptr(),
        slistr.as_ptr(),
        pstr.as_ptr(),
    ];
    println!("{refstr}\n{mstr}\n{slistr}\n{pstr}\n{ptr:?}");
}
