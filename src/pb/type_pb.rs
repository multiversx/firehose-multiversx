// This file is generated by rust-protobuf 2.28.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `sf/multiversx/type/v1/type.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(PartialEq,Clone,Default)]
pub struct BlockHeader {
    // message fields
    pub height: u64,
    pub hash: ::std::string::String,
    pub previous_num: u64,
    pub previous_hash: ::std::string::String,
    pub final_num: u64,
    pub final_hash: ::std::string::String,
    pub timestamp: u64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a BlockHeader {
    fn default() -> &'a BlockHeader {
        <BlockHeader as ::protobuf::Message>::default_instance()
    }
}

impl BlockHeader {
    pub fn new() -> BlockHeader {
        ::std::default::Default::default()
    }

    // uint64 height = 1;


    pub fn get_height(&self) -> u64 {
        self.height
    }
    pub fn clear_height(&mut self) {
        self.height = 0;
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u64) {
        self.height = v;
    }

    // string hash = 2;


    pub fn get_hash(&self) -> &str {
        &self.hash
    }
    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::string::String) {
        self.hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::string::String {
        &mut self.hash
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.hash, ::std::string::String::new())
    }

    // uint64 previous_num = 3;


    pub fn get_previous_num(&self) -> u64 {
        self.previous_num
    }
    pub fn clear_previous_num(&mut self) {
        self.previous_num = 0;
    }

    // Param is passed by value, moved
    pub fn set_previous_num(&mut self, v: u64) {
        self.previous_num = v;
    }

    // string previous_hash = 4;


    pub fn get_previous_hash(&self) -> &str {
        &self.previous_hash
    }
    pub fn clear_previous_hash(&mut self) {
        self.previous_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_previous_hash(&mut self, v: ::std::string::String) {
        self.previous_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_previous_hash(&mut self) -> &mut ::std::string::String {
        &mut self.previous_hash
    }

    // Take field
    pub fn take_previous_hash(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.previous_hash, ::std::string::String::new())
    }

    // uint64 final_num = 5;


    pub fn get_final_num(&self) -> u64 {
        self.final_num
    }
    pub fn clear_final_num(&mut self) {
        self.final_num = 0;
    }

    // Param is passed by value, moved
    pub fn set_final_num(&mut self, v: u64) {
        self.final_num = v;
    }

    // string final_hash = 6;


    pub fn get_final_hash(&self) -> &str {
        &self.final_hash
    }
    pub fn clear_final_hash(&mut self) {
        self.final_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_final_hash(&mut self, v: ::std::string::String) {
        self.final_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_final_hash(&mut self) -> &mut ::std::string::String {
        &mut self.final_hash
    }

    // Take field
    pub fn take_final_hash(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.final_hash, ::std::string::String::new())
    }

    // uint64 timestamp = 7;


    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
    pub fn clear_timestamp(&mut self) {
        self.timestamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u64) {
        self.timestamp = v;
    }
}

impl ::protobuf::Message for BlockHeader {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.height = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.hash)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.previous_num = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.previous_hash)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.final_num = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.final_hash)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.timestamp = tmp;
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
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(1, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.hash);
        }
        if self.previous_num != 0 {
            my_size += ::protobuf::rt::value_size(3, self.previous_num, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.previous_hash.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.previous_hash);
        }
        if self.final_num != 0 {
            my_size += ::protobuf::rt::value_size(5, self.final_num, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.final_hash.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.final_hash);
        }
        if self.timestamp != 0 {
            my_size += ::protobuf::rt::value_size(7, self.timestamp, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.height != 0 {
            os.write_uint64(1, self.height)?;
        }
        if !self.hash.is_empty() {
            os.write_string(2, &self.hash)?;
        }
        if self.previous_num != 0 {
            os.write_uint64(3, self.previous_num)?;
        }
        if !self.previous_hash.is_empty() {
            os.write_string(4, &self.previous_hash)?;
        }
        if self.final_num != 0 {
            os.write_uint64(5, self.final_num)?;
        }
        if !self.final_hash.is_empty() {
            os.write_string(6, &self.final_hash)?;
        }
        if self.timestamp != 0 {
            os.write_uint64(7, self.timestamp)?;
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

    fn new() -> BlockHeader {
        BlockHeader::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "height",
                |m: &BlockHeader| { &m.height },
                |m: &mut BlockHeader| { &mut m.height },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "hash",
                |m: &BlockHeader| { &m.hash },
                |m: &mut BlockHeader| { &mut m.hash },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "previous_num",
                |m: &BlockHeader| { &m.previous_num },
                |m: &mut BlockHeader| { &mut m.previous_num },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "previous_hash",
                |m: &BlockHeader| { &m.previous_hash },
                |m: &mut BlockHeader| { &mut m.previous_hash },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "final_num",
                |m: &BlockHeader| { &m.final_num },
                |m: &mut BlockHeader| { &mut m.final_num },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "final_hash",
                |m: &BlockHeader| { &m.final_hash },
                |m: &mut BlockHeader| { &mut m.final_hash },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "timestamp",
                |m: &BlockHeader| { &m.timestamp },
                |m: &mut BlockHeader| { &mut m.timestamp },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<BlockHeader>(
                "BlockHeader",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static BlockHeader {
        static instance: ::protobuf::rt::LazyV2<BlockHeader> = ::protobuf::rt::LazyV2::INIT;
        instance.get(BlockHeader::new)
    }
}

impl ::protobuf::Clear for BlockHeader {
    fn clear(&mut self) {
        self.height = 0;
        self.hash.clear();
        self.previous_num = 0;
        self.previous_hash.clear();
        self.final_num = 0;
        self.final_hash.clear();
        self.timestamp = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Block {
    // message fields
    pub header: ::protobuf::SingularPtrField<BlockHeader>,
    pub guardians: ::protobuf::SingularPtrField<super::guardians::Guardians>,
    pub blockk: ::protobuf::SingularPtrField<super::outportBlock::OutportBlock>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Block {
    fn default() -> &'a Block {
        <Block as ::protobuf::Message>::default_instance()
    }
}

impl Block {
    pub fn new() -> Block {
        ::std::default::Default::default()
    }

    // .sf.multiversx.type.v1.BlockHeader header = 1;


    pub fn get_header(&self) -> &BlockHeader {
        self.header.as_ref().unwrap_or_else(|| <BlockHeader as ::protobuf::Message>::default_instance())
    }
    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: BlockHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut BlockHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> BlockHeader {
        self.header.take().unwrap_or_else(|| BlockHeader::new())
    }

    // .guardians.Guardians guardians = 2;


    pub fn get_guardians(&self) -> &super::guardians::Guardians {
        self.guardians.as_ref().unwrap_or_else(|| <super::guardians::Guardians as ::protobuf::Message>::default_instance())
    }
    pub fn clear_guardians(&mut self) {
        self.guardians.clear();
    }

    pub fn has_guardians(&self) -> bool {
        self.guardians.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guardians(&mut self, v: super::guardians::Guardians) {
        self.guardians = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_guardians(&mut self) -> &mut super::guardians::Guardians {
        if self.guardians.is_none() {
            self.guardians.set_default();
        }
        self.guardians.as_mut().unwrap()
    }

    // Take field
    pub fn take_guardians(&mut self) -> super::guardians::Guardians {
        self.guardians.take().unwrap_or_else(|| super::guardians::Guardians::new())
    }

    // .proto.OutportBlock blockk = 3;


    pub fn get_blockk(&self) -> &super::outportBlock::OutportBlock {
        self.blockk.as_ref().unwrap_or_else(|| <super::outportBlock::OutportBlock as ::protobuf::Message>::default_instance())
    }
    pub fn clear_blockk(&mut self) {
        self.blockk.clear();
    }

    pub fn has_blockk(&self) -> bool {
        self.blockk.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockk(&mut self, v: super::outportBlock::OutportBlock) {
        self.blockk = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blockk(&mut self) -> &mut super::outportBlock::OutportBlock {
        if self.blockk.is_none() {
            self.blockk.set_default();
        }
        self.blockk.as_mut().unwrap()
    }

    // Take field
    pub fn take_blockk(&mut self) -> super::outportBlock::OutportBlock {
        self.blockk.take().unwrap_or_else(|| super::outportBlock::OutportBlock::new())
    }
}

impl ::protobuf::Message for Block {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.guardians {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.blockk {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.guardians)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.blockk)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.guardians.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.blockk.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.guardians.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.blockk.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn new() -> Block {
        Block::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BlockHeader>>(
                "header",
                |m: &Block| { &m.header },
                |m: &mut Block| { &mut m.header },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::guardians::Guardians>>(
                "guardians",
                |m: &Block| { &m.guardians },
                |m: &mut Block| { &mut m.guardians },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::outportBlock::OutportBlock>>(
                "blockk",
                |m: &Block| { &m.blockk },
                |m: &mut Block| { &mut m.blockk },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Block>(
                "Block",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Block {
        static instance: ::protobuf::rt::LazyV2<Block> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Block::new)
    }
}

impl ::protobuf::Clear for Block {
    fn clear(&mut self) {
        self.header.clear();
        self.guardians.clear();
        self.blockk.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Block {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Block {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20sf/multiversx/type/v1/type.proto\x12\x15sf.multiversx.type.v1\x1a\
    \x19guardians/guardians.proto\x1aFgithub.com/multiversx/mx-chain-core-go\
    /data/outport/outportBlock.proto\"\xdb\x01\n\x0bBlockHeader\x12\x16\n\
    \x06height\x18\x01\x20\x01(\x04R\x06height\x12\x12\n\x04hash\x18\x02\x20\
    \x01(\tR\x04hash\x12!\n\x0cprevious_num\x18\x03\x20\x01(\x04R\x0bpreviou\
    sNum\x12#\n\rprevious_hash\x18\x04\x20\x01(\tR\x0cpreviousHash\x12\x1b\n\
    \tfinal_num\x18\x05\x20\x01(\x04R\x08finalNum\x12\x1d\n\nfinal_hash\x18\
    \x06\x20\x01(\tR\tfinalHash\x12\x1c\n\ttimestamp\x18\x07\x20\x01(\x04R\t\
    timestamp\"\xa4\x01\n\x05Block\x12:\n\x06header\x18\x01\x20\x01(\x0b2\".\
    sf.multiversx.type.v1.BlockHeaderR\x06header\x122\n\tguardians\x18\x02\
    \x20\x01(\x0b2\x14.guardians.GuardiansR\tguardians\x12+\n\x06blockk\x18\
    \x03\x20\x01(\x0b2\x13.proto.OutportBlockR\x06blockkb\x06proto3\
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