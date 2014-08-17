extern crate serialize;
extern crate core;

use std::iter::Zip;
use core::iter::Map;
use core::slice::Items;
use serialize::hex::FromHex;
use serialize::hex::FromHexError;
use serialize::hex::ToHex;

fn main() {
    match std::os::args().as_slice() {
        [_, ref left, ref right] => {
            match hex_xor(left, right) {
                Ok(stri) => println!("{}", stri),
                Err(_) => std::os::set_exit_status(2)
            }
        }
        _ => std::os::set_exit_status(1)
    }
}

fn hex_xor(first: &String, second: &String) -> Result<String, FromHexError> {
    let (a, b) = try!(hexify(first, second));

    let aiter: Items<u8> = a.iter();
    let iter: Zip<Items<u8>, Items<u8>> = aiter.zip(b.iter());
    let xored: Map<(&u8, &u8), u8, Zip<Items<u8>, Items<u8>>> = iter.map( |(left, right)|
        *left ^ *right
    );
    let vec: Vec<u8> = FromIterator::from_iter(xored);

    Ok(vec.as_slice().to_hex())
}

fn hexify(first: &String, second: &String) -> Result<(Vec<u8>, Vec<u8>), FromHexError> {
    Ok((try!(first.as_slice().from_hex()), try!(second.as_slice().from_hex())))
}
