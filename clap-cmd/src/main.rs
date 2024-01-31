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

    let c = Coin::from(10);
    println!("from u8: {:?}", c);
    let c = Coin::from('k');
    println!("from char: {:?}", c);

    let c: Coin = 'k'.into();
    println!("char into: {:?}", c);

    let c: Coin = 2.into();
    println!("u8 into: {:?}", c);
}

#[derive(Debug)]
enum Coin {
    Peny,
    Squart,
}

impl From<u8> for Coin {
    fn from(value: u8) -> Self {
        match value {
            1..=10 => Coin::Peny,
            _ => Coin::Squart,
        }
    }
}

impl From<char> for Coin {
    fn from(value: char) -> Self {
        match value {
            'a'..='j' => Coin::Peny,
            _ => Coin::Squart,
        }
    }
}
