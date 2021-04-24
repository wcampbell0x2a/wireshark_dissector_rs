
// https://gitlab.com/wireshark/wireshark/-/blob/master/doc/README.dissector
// https://www.wireshark.org/docs/wsdg_html/#ChDissectDetails
// /usr/include/wireshark/epan

// THis seems useful?
// https://stackoverflow.com/a/55323693

#[repr(C)]
pub struct proto_plugin {
    pub register_protoinfo: Option<extern "C" fn()>,/* routine to call to register protocol information */
    pub register_handoff: Option<extern "C" fn()>,/* routine to call to register protocol information */
}

impl Default for proto_plugin {
    fn default() -> Self {
        proto_plugin {
            register_protoinfo: None,
            register_handoff: None,
        }
    }
}

#[repr(C)] pub struct dissector_handle { _private: [u8; 0] }
type dissector_handle_t = *mut dissector_handle;
#[repr(C)] pub struct tvbuff_t { _private: [u8; 0] }

#[repr(C)] pub struct packet_info { _private: [u8; 0] }
type pinfo = packet_info;
// Hmm, packet_info is enormous, but we have to reach into it for column info. Let skip that for now.


#[repr(C)] pub struct proto_tree { _private: [u8; 0] }
#[repr(C)] pub struct proto_item { _private: [u8; 0] }


type dissector_t = Option<extern "C" fn(*mut tvbuff_t, *mut packet_info, *mut proto_tree, *mut libc::c_void) -> u32>;
//typedef struct capture_dissector_handle* capture_dissector_handle_t;

//'hf' is short for 'header field'
#[repr(C)]
#[allow(dead_code)]
pub enum hf_ref_type {
    NONE,
    INDIRECT,
    DIRECT,
}
impl Default for hf_ref_type {
    fn default() -> Self { hf_ref_type::NONE }
}
unsafe impl Send for hf_ref_type {}

#[repr(C)]
#[allow(dead_code)]
pub enum ftenum {
    NONE,
    PROTOCOL,
    BOOLEAN,
    CHAR,
    UINT8,
    UINT16,
    UINT24,
    UINT32,
}
impl Default for ftenum {
    fn default() -> Self { ftenum::NONE }
}
unsafe impl Send for ftenum {}

#[repr(C)]
//~ #[derive(Default)]
pub struct header_field_info {
    pub name: *const libc::c_char,
    pub abbrev: *const libc::c_char,
    pub type_ : ftenum,
    pub display: i32,
    pub strings: *const libc::c_char, // actually void ptr
    pub bitmask: u64,
    pub blurb: *const libc::c_char,

    //
    pub id: i32,
    pub parent: i32,
    pub ref_type: hf_ref_type,
    pub same_name_pref_id: i32,
    pub same_name_next: *mut header_field_info,
}
impl Default for header_field_info {
    fn default() -> Self { header_field_info{
        name: 0 as *const libc::c_char,
        abbrev: 0 as *const libc::c_char,
        type_: Default::default(),
        display: Default::default(),
        strings: 0 as *const libc::c_char,
        bitmask: Default::default(),
        blurb: 0 as *const libc::c_char,
        id: Default::default(),
        parent: Default::default(),
        ref_type: Default::default(),
        same_name_pref_id: Default::default(),
        same_name_next: 0 as *mut header_field_info,

    }}
}
unsafe impl Send for header_field_info {}
#[derive(Default)]

pub struct ThreadUnSafeHeaderFieldInfoHolder
{
    pub data: Option<header_field_info>
}
unsafe impl Sync for ThreadUnSafeHeaderFieldInfoHolder {}
unsafe impl Send for ThreadUnSafeHeaderFieldInfoHolder {}

#[repr(C)]
pub struct hf_register_info
{
    pub p_id:  *mut i32, // written to by register() function
    pub hfinfo: header_field_info	// < the field info to be registered 
}
impl Default for hf_register_info {
    fn default() -> Self { hf_register_info{
        p_id: 0 as *mut i32,
        hfinfo: Default::default(),
    }
    }
}
unsafe impl Send for hf_register_info {}

pub struct ThreadUnSafeHeaderFieldRegisterInfoHolder
{
    pub data: Option<hf_register_info>
}
unsafe impl Sync for ThreadUnSafeHeaderFieldRegisterInfoHolder {}
unsafe impl Send for ThreadUnSafeHeaderFieldRegisterInfoHolder {}


#[link(name = "wireshark")]
extern {
    pub fn tvb_reported_length(tvb : *const tvbuff_t) -> i32;

    pub fn proto_tree_add_protocol_format(tree : *mut proto_tree, hfindex: i32, tvb: *mut tvbuff_t, start: i32, length: i32, format: *const libc::c_char, ...) -> *mut proto_item;

    pub fn proto_register_protocol(name: *const libc::c_char, short_name: *const libc::c_char, filter_name: *const libc::c_char) -> i32;
    pub fn proto_register_plugin(plugin: *const proto_plugin);

    pub fn create_dissector_handle(dissector : dissector_t, proto: i32) -> dissector_handle_t;
    pub fn register_postdissector(handle: dissector_handle_t);
    pub fn g_print(string: *const libc::c_char);

    pub fn proto_register_field_array(parent: i32, hf: *mut hf_register_info, num_records: i32);
    pub fn proto_register_subtree_array();
}

