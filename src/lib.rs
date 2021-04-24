// https://doc.rust-lang.org/nomicon/ffi.html
extern crate libc;


#[macro_use]
extern crate lazy_static;
#[no_mangle]
static plugin_version: [libc::c_char;  4] = [50, 46, 54, 0];  // "2.6"
#[no_mangle]
static plugin_release: [libc::c_char;  4] = [50, 46, 54, 0];  // "2.6"


mod util;
mod wireshark;

use std::ffi::CStr;
use std::ffi::CString;
// https://doc.rust-lang.org/std/ffi/struct.CStr.html

// https://github.com/wireshark/wireshark/blob/master/epan/dissectors/packet-g723.c

static mut proto_hello_hf : i32 = 0;

extern "C" fn dissect_hello(tvb: *mut wireshark::tvbuff_t, packet_info: *mut wireshark::packet_info, tree: *mut wireshark::proto_tree, data: *mut libc::c_void) -> u32
{
    unsafe
    {
        println!("Dissector hello called!");
        //~ let proto_hello: i32 = -1;
        wireshark::proto_tree_add_protocol_format(tree, proto_hello_hf, tvb, 0, -1, util::perm_string_ptr("This is Hello version, a Wireshark postdissector plugin prototype"));
        return wireshark::tvb_reported_length(tvb) as u32;
    }
    return 0;
}

//https://stackoverflow.com/a/55323803
trait AsMutPtr<T> {
    fn as_mut_ptr(&mut self) -> *mut T;
}

impl<'a, T> AsMutPtr<T> for Option<&'a mut T> {
    fn as_mut_ptr(&mut self) -> *mut T {
        match self {
            Some(v) => *v,
            None => {
                println!("Its a nullptr :( ");
                ptr::null_mut()
            }
            ,
        }
    }
}


extern "C" fn proto_register_hello()
{
    println!("proto_register_hello");
    //~ let cstr = CString::new("hello").unwrap();

    static mut  hf : [wireshark::ThreadUnSafeHeaderFieldRegisterInfoHolder; 1] = [wireshark::ThreadUnSafeHeaderFieldRegisterInfoHolder{ data: None} ];
    static mut header_int : i32 = -1;
    unsafe
    {
        hf[0].data = Some(wireshark::hf_register_info{
            p_id: &mut header_int as *mut i32,
            hfinfo: {
                wireshark::header_field_info{
                    name: util::perm_string_ptr("KSDJFLSDJ"),
                    abbrev: util::perm_string_ptr("thign.type"),
                    type_: wireshark::ftenum::PROTOCOL,
                    ..Default::default()}
                }
            });
    }

    let cstr = util::perm_string("hello");
    unsafe 
    {
        let proto_int = wireshark::proto_register_protocol(util::perm_string_ptr("The thingy"), cstr.as_ptr(), cstr.as_ptr());
        println!("Proto proto_int: {:?}", proto_int);


        let proto_hello: i32 = -1;
        let z = wireshark::create_dissector_handle(Some(dissect_hello), proto_hello);
        println!("Proto hello: {:?}", proto_hello);
        wireshark::register_postdissector(z);
        //~ let p = hf[0].data.map_or_else(ptr::null, |x| x);
        //~ unsafe { ffi_call(p) }
        let rawptr = hf[0].data.as_mut().as_mut_ptr() as *mut wireshark::hf_register_info;
        println!("rawptr hello: {:?}", rawptr);
        println!("hf[0].data.thing: {}", hf[0].data.is_some());
        wireshark::proto_register_field_array(proto_int, rawptr, 1);
        proto_hello_hf = header_int;
    }
    //~ register_postdissector(handle_hello);
}

extern "C" fn proto_reg_handoff_hello()
{
    println!("proto_reg_handoff_hello");
}

//~ static mut five : Box<MaybeUninit<wireshark::proto_plugin>>;
//~ static mut five: Box<wireshark::proto_plugin> = Box::new(Default::default());

use std::ptr;

#[no_mangle]
pub fn plugin_register() {
    println!("plugin_register");
    let cstr = CString::new("hello").unwrap();
    unsafe 
    {
        wireshark::g_print(cstr.as_ptr());  // Yas, we're in business!
        //~ five = Default::default();
        let mut five: Box<wireshark::proto_plugin> = Box::new(Default::default());
        //~ let mut plug : wireshark::proto_plugin = Default::default();
        five.register_protoinfo = Some(proto_register_hello);
        five.register_handoff = Some(proto_reg_handoff_hello);
        wireshark::proto_register_plugin(Box::leak(five)); // This kinda sucks lol.
        //~ wireshark::proto_register_plugin(&plug);
    }
}


