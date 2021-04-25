// https://doc.rust-lang.org/nomicon/ffi.html

// We can probably hook; https://github.com/wireshark/wireshark/blob/ebfbf958f6930b2dad486b33277470e8368dc111/epan/dissectors/packet-usb.c#L3516-L3518
extern crate bitflags;
extern crate libc;

#[macro_use]
extern crate lazy_static;

mod dissector;
mod util;
mod wireshark;

struct MyDissector {
    p: u32,
}
// Lift these to make it less verbose.
type FieldType = dissector::FieldType;
type FieldDisplay = dissector::FieldDisplay;
impl MyDissector {
    const field1: dissector::PacketField = dissector::PacketField {
        name: "protoname",
        abbrev: "proto.main",
        field_type: FieldType::PROTOCOL,
        display: FieldDisplay::NONE,
    };
    const field2: dissector::PacketField = dissector::PacketField {
        name: "byte0name",
        abbrev: "proto.byte0",
        field_type: FieldType::U8,
        display: FieldDisplay::HEX,
    };
}

impl dissector::Dissector for MyDissector {
    fn get_fields(self: &Self) -> Vec<dissector::PacketField> {
        let mut f = Vec::new();
        f.push(MyDissector::field1);
        f.push(MyDissector::field2);
        return f;
    }
    fn dissect(self: &Self, display: &dissector::PacketDisplay, bytes: &[u8]) {
        // do cool rust things, pass entities into the display.
    }
    fn foo(self: &mut Self) {
        self.p = self.p + 1;
        //~ println!("{}  {:p}", self.p, &self, );
        //~ println!("yes, things.");
    }
}

// This function is the main entry point where we can do our setup.
#[no_mangle]
pub fn plugin_register() {
    dissector::plugin_register_worker();

    let z = Box::new(MyDissector { p: 0 });
    dissector::setup(z);
}
