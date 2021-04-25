

pub trait Dissection {
    //~ fn display(self: &mut Self, item: &dyn DisplayItem);
    fn u8(self: &mut Self) -> u8;
    fn display_u8(self: &mut Self, item: &dyn DisplayItem) -> u8;
}



#[derive(Debug, Copy, Clone)]
struct DissectionTest
{
    pub pos: usize,
}
impl Dissection for DissectionTest
{
    //~ fn display(self: &mut Self, item: &dyn DisplayItem)
    //~ {
    //~ }

    fn u8(self: &mut Self) -> u8
    {
        return self.pos as u8;
    }

    fn display_u8(self: &mut Self, item: &dyn DisplayItem) -> u8
    {
        println!("Displaying u8");
        let val = self.pos as u8;
        self.pos += 1;
        return val;
    }

}


#[test]
fn it_works() {
    let mut z : DissectionTest = DissectionTest{pos: 0};
    let peeked_u8 = z.u8();
    println!("Peeked {}", peeked_u8);

    const FIELD1: PacketField = PacketField {
        name: "protoname",
        abbrev: "proto.main",
        field_type: FieldType::PROTOCOL,
        display: FieldDisplay::NONE,
    };
    z.display_u8(&field_to_display(FIELD1));
    println!("Peeked {}", z.u8());
}

// A trait for things that can dissect data.
pub trait Dissector {
    fn get_fields(self: &Self) -> Vec<PacketField>;
    fn dissect(
        self: &Self,
        dissection: Box<dyn Dissection>,
    );
    fn foo(self: &mut Self);
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum FieldType {
    PROTOCOL,
    U8,
}
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum FieldDisplay {
    NONE,
    DEC,
    HEX,
}

#[derive(Debug, Copy, Clone)]
pub struct PacketField {
    pub name: &'static str,
    pub abbrev: &'static str,
    pub field_type: FieldType,
    pub display: FieldDisplay,
}

// Something that is displayable in the ui.
pub trait DisplayItem {
    fn get_field(&self) -> PacketField;
}

pub struct DisplayItemField
{
    pub field: PacketField,
}
impl DisplayItem  for DisplayItemField{
    fn get_field(self: &Self) -> PacketField
    {
        return self.field;
    }
}

pub fn field_to_display(thing : PacketField) -> DisplayItemField
{
    DisplayItemField {
        field : thing
    }
}

//~ impl From<&PacketField> for DisplayItem {
    //~ fn from(thing: &PacketField) -> DisplayItemField {
        //~ DisplayItemField {
            //~ field : thing
        //~ }
    //~ }
//~ }


struct DisplayU8 {
    field: PacketField,
}
impl DisplayItem for DisplayU8 {
    fn get_field(&self) -> PacketField {
        return self.field;
    }
}

// We know that wireshark will ensure only one thread accesses the disector, I think... make this static thing to
// hold our dissector object.
struct UnsafeDissectorHolder {
    ptr: Box<dyn Dissector>,



    // The things below are usually static members in wireshark plugins.
    proto_id: i32,
    field_ids: Vec<i32>,
    fields: Vec<wireshark::hf_register_info>,
    plugin_handle: *mut wireshark::proto_plugin,
}
unsafe impl Sync for UnsafeDissectorHolder {}
unsafe impl Send for UnsafeDissectorHolder {}
impl UnsafeDissectorHolder {
    fn new(ptr: Box<dyn Dissector>) -> Self {
        UnsafeDissectorHolder {
            ptr: ptr,
            proto_id: -1,
            field_ids: Vec::new(),
            fields: Vec::new(),
            plugin_handle: 0 as *mut wireshark::proto_plugin,
        }
    }
}

static mut STATIC_DISSECTOR: Option<UnsafeDissectorHolder> = None;

pub fn setup(d: Box<dyn Dissector>) {
    // Assign the dissector to be called sequentially.
    unsafe {
        // Make our global state
        STATIC_DISSECTOR = Some(UnsafeDissectorHolder::new(d));

        // Then, make the plugin handle and bind the functions.
        let state = &mut STATIC_DISSECTOR.as_mut().unwrap();

        let mut plugin_handle_box: Box<wireshark::proto_plugin> = Box::new(Default::default());
        plugin_handle_box.register_protoinfo = Some(proto_register_protoinfo);
        plugin_handle_box.register_handoff = Some(proto_register_handoff);
        state.plugin_handle = Box::leak(plugin_handle_box);  // Need this to persist....
        wireshark::proto_register_plugin(state.plugin_handle);
    }
}

use crate::util;
use crate::wireshark;

// https://github.com/wireshark/wireshark/blob/master/epan/dissectors/packet-g723.c


extern "C" fn dissect_protocol_function(
    tvb: *mut wireshark::tvbuff_t,
    _packet_info: *mut wireshark::packet_info,
    tree: *mut wireshark::proto_tree,
    _data: *mut libc::c_void,
) -> u32 {


    unsafe {
        let state = &mut STATIC_DISSECTOR.as_mut().unwrap(); // less wordy.
                                                             //~ STATIC_DISSECTOR.as_mut().unwrap().ptr.foo();
                                                             //~ println!("Dissector hello called!");
                                                             //~ let proto_hello: i32 = -1;

        // Raw bytes to slice:
        //std::slice::from_raw_parts_mut(buf.data, buf.len)

        let _proto_item = wireshark::proto_tree_add_protocol_format(
            tree,
            state.field_ids[0],
            tvb,
            0,
            0,
            util::perm_string_ptr(
                "This is Hello version, a Wireshark postdissector plugin %d prototype",
            ),
            3,
        );
        let _thing = wireshark::proto_tree_add_item(
            tree,
            state.field_ids[1],
            tvb,
            0,
            1,
            wireshark::Encoding::STR_HEX,
        );

        return wireshark::tvb_reported_length(tvb) as u32;
    }
}

extern "C" fn proto_register_protoinfo() {
    println!("proto_register_hello");

    let cstr = util::perm_string("hello");

    unsafe {
        let state = &mut STATIC_DISSECTOR.as_mut().unwrap(); // less wordy.

        let proto_int = wireshark::proto_register_protocol(
            util::perm_string_ptr("The thingy"),
            cstr.as_ptr(),
            cstr.as_ptr(),
        );
        println!("Proto proto_int: {:?}", proto_int);

        // ok, here we get to make our header fields array, and then we can pass that to wireshark.
        let fields = state.ptr.get_fields();
        println!(
            "Registering {} fields in the protocol register.",
            fields.len()
        );
        state.field_ids.resize(fields.len(), -1);
        for i in 0..fields.len() {
            state.fields.push(wireshark::hf_register_info {
                p_id: &mut state.field_ids[i] as *mut i32,
                hfinfo: fields[i].into(),
            });
        }

        let z = wireshark::create_dissector_handle(Some(dissect_protocol_function), state.proto_id);
        println!("state.proto_id {:?}", state.proto_id);
        wireshark::register_postdissector(z);

        let rawptr = &mut state.fields[0] as *mut wireshark::hf_register_info;
        wireshark::proto_register_field_array(proto_int, rawptr, fields.len() as i32);
    }
}

extern "C" fn proto_register_handoff() {
    println!("proto_reg_handoff_hello");
}


#[no_mangle]
static plugin_version: [libc::c_char; 4] = [50, 46, 54, 0]; // "2.6"
#[no_mangle]
static plugin_release: [libc::c_char; 4] = [50, 46, 54, 0]; // "2.6"

