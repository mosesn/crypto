extern crate serialize;

use serialize::base64::Config;
use serialize::hex::FromHex;
use serialize::hex::FromHexError;
use serialize::base64::ToBase64;
use serialize::base64::Standard;

fn main() {
    let args = std::os::args();

    match args.as_slice() {
        [_, ref arg] =>  {
            let value = String::from_str(arg.as_slice());
            let res = hex_to_base64(value);
            match res {
                Ok(stri)   => println!("{}", stri),
                Err(_) => std::os::set_exit_status(2)
            }
        },
        _     => std::os::set_exit_status(1)
    }
}

fn hex_to_base64(hex: String) -> Result<String, FromHexError> {
    let conf = Config {
        char_set: Standard,
        pad: false,
        line_length: None
    };
    hex.as_slice().from_hex().map( |bytes|
      bytes.as_slice().to_base64(conf)
    )
}
