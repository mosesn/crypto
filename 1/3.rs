extern crate serialize;

use std::vec::Vec;
use serialize::hex::FromHex;
use serialize::hex::FromHexError;

fn main() {
    match std::os::args().as_slice() {
        [_, ref string] => {
            match score_string(string) {
                Ok(vec) => println!("{}", vec),
                Err(_) => std::os::set_exit_status(2)
            }
        }
        _ => std::os::set_exit_status(1)
    }
}

fn score_string(text: &String) -> Result<Vec<String>, FromHexError> {
    let res = try!(text.as_slice().from_hex());
    let opts = range(0, 255).filter_map(|idx| xor_slice(&res, idx));
    let vec: Vec<String> = FromIterator::from_iter(opts);
    Ok(vec)
}

fn xor_slice(res: &Vec<u8>, idx: u8) -> Option<String> {
    let vec = FromIterator::from_iter(res.iter().map(|x| x ^ idx));

    score(vec).map(|ascii| ascii.into_string())
}

fn score(vec: Vec<u8>) -> Option<Vec<Ascii>> {
    vec.into_ascii_opt().filtered(|asciis| {
        asciis.iter().all(|x| {
            x.is_alphanumeric() || x.is_blank() || (*x).to_char() == '\''
        })
    })
}
