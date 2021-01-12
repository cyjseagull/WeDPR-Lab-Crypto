// Copyright 2020 WeDPR Lab Project Authors. Licensed under Apache-2.0.

// This file is generated by rust-protobuf 2.20.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `crypto/common.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_20_0;

#[derive(PartialEq,Clone,Default)]
pub struct Keypair {
    // message fields
    pub public_key: ::std::vec::Vec<u8>,
    pub private_key: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Keypair {
    fn default() -> &'a Keypair {
        <Keypair as ::protobuf::Message>::default_instance()
    }
}

impl Keypair {
    pub fn new() -> Keypair {
        ::std::default::Default::default()
    }

    // bytes public_key = 1;


    pub fn get_public_key(&self) -> &[u8] {
        &self.public_key
    }
    pub fn clear_public_key(&mut self) {
        self.public_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_public_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.public_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_public_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.public_key
    }

    // Take field
    pub fn take_public_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.public_key, ::std::vec::Vec::new())
    }

    // bytes private_key = 2;


    pub fn get_private_key(&self) -> &[u8] {
        &self.private_key
    }
    pub fn clear_private_key(&mut self) {
        self.private_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_private_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.private_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_private_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.private_key
    }

    // Take field
    pub fn take_private_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.private_key, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Keypair {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.public_key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.private_key)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.public_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.public_key);
        }
        if !self.private_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.private_key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.public_key.is_empty() {
            os.write_bytes(1, &self.public_key)?;
        }
        if !self.private_key.is_empty() {
            os.write_bytes(2, &self.private_key)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Keypair {
        Keypair::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "public_key",
                |m: &Keypair| { &m.public_key },
                |m: &mut Keypair| { &mut m.public_key },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "private_key",
                |m: &Keypair| { &m.private_key },
                |m: &mut Keypair| { &mut m.private_key },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Keypair>(
                "Keypair",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Keypair {
        static instance: ::protobuf::rt::LazyV2<Keypair> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Keypair::new)
    }
}

impl ::protobuf::Clear for Keypair {
    fn clear(&mut self) {
        self.public_key.clear();
        self.private_key.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Keypair {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Keypair {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13crypto/common.proto\x12\"com.webank.blockchain.crypto.proto\"I\n\
    \x07Keypair\x12\x1d\n\npublic_key\x18\x01\x20\x01(\x0cR\tpublicKey\x12\
    \x1f\n\x0bprivate_key\x18\x02\x20\x01(\x0cR\nprivateKeyB&\n\"com.webank.\
    blockchain.crypto.protoP\x01b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
