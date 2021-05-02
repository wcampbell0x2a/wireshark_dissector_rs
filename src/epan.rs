// https://www.wireshark.org/docs/wsdg_html/#ChDissectDetails
// /usr/include/wireshark/epan

// https://github.com/wireshark/wireshark/blob/ebfbf958f6930b2dad486b33277470e8368dc111/epan/dissectors/packet-usb.c
// https://github.com/wireshark/wireshark/blob/ebfbf958f6930b2dad486b33277470e8368dc111/epan/dissectors/packet-usb-hid.c
// 1.5 Constructing the protocol tree; https://github.com/wireshark/wireshark/blob/ebfbf958f6930b2dad486b33277470e8368dc111/doc/README.dissector#L713

// 1.5.1 Field Registration; https://github.com/wireshark/wireshark/blob/ebfbf958f6930b2dad486b33277470e8368dc111/doc/README.dissector#L1270
// 1.7 Calling other dissectors; https://github.com/wireshark/wireshark/blob/ebfbf958f6930b2dad486b33277470e8368dc111/doc/README.dissector#L2471
// 1.7.1 Dissector Tables; https://github.com/wireshark/wireshark/blob/ebfbf958f6930b2dad486b33277470e8368dc111/doc/README.dissector#L2540


// 1.5.2 Adding Items and Values to the Protocol Tree. https://github.com/wireshark/wireshark/blob/ebfbf958f6930b2dad486b33277470e8368dc111/doc/README.dissector#L1351

// Reassembly 2.7.2 Modifying the pinfo struct; https://github.com/wireshark/wireshark/blob/ebfbf958f6930b2dad486b33277470e8368dc111/doc/README.dissector#L3472
// Yeah, that doesn't work for USB packets... gg.

// This seems useful?
// https://stackoverflow.com/a/55323693

#![allow(non_camel_case_types)]
#![allow(dead_code)]


// These files follow the same structure as the header files.
pub mod proto;
pub mod ftypes;
pub mod range;
pub mod tvbuff;
pub mod packet;
pub mod packet_info;

/*
    Dissector
        get_fields()
        get_tree()
        get_protocol_name()
        get_registration()

    ProtoTree
        add_**(field_index, tvb, pos, len, encoding, ....) -> returns ProtoItem
        add_boolean(field_index,tvb, start, 
        add_item_ret_uint64 -> returns (ProtoItem, u64)

    PacketInfo?
        Lets ignore for now.

    TVB
        // Raw peeking into the buffer.

    ProtoItem
        // Things like:
        proto_item_set_text(proto_item *ti, const char *format, ...) G_GNUC_PRINTF(2,3);
        proto_item_add_subtree(tree_index) -> ProtoTree

    
 */

pub trait ProtoTree
{
}


pub trait ProtoItem
{
}

pub trait TVB
{
}

