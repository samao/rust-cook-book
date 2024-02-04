use std::ffi::{c_char, CString};

extern "C" {
    fn hello();
    fn greet(name: *const c_char);
}

fn promp(s: &str) -> clap_cmd::Result<String> {
    use std::io::Write;
    print!("{}", s);
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn main() -> clap_cmd::Result<()> {
    unsafe { hello() }
    let name = promp("What's your name? ")?;
    let c_name = CString::new(name)?;
    unsafe { greet(c_name.as_ptr()) }
    Ok(())
}
