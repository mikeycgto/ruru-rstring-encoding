#[macro_use]
extern crate ruru;

use ruru::{Class, Object, RString};

class!(StringTest);

methods!(
    StringTest,
    _itself,

    fn empty_string() -> RString {
        RString::new("")
    }
);

#[no_mangle]
pub extern fn initialize_string_test() {
    Class::new("StringTest", None).define(|itself| {
        itself.def("empty_string", empty_string);
    });
}
