// This file is generated by rust-protobuf 2.14.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `src/tensorboardrs/proto/tensor.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_14_0;

#[derive(PartialEq,Clone,Default)]
pub struct TensorProto {
    // message fields
    pub dtype: super::types::DataType,
    pub tensor_shape: ::protobuf::SingularPtrField<super::tensor_shape::TensorShapeProto>,
    pub version_number: i32,
    pub tensor_content: ::std::vec::Vec<u8>,
    pub half_val: ::std::vec::Vec<i32>,
    pub float_val: ::std::vec::Vec<f32>,
    pub double_val: ::std::vec::Vec<f64>,
    pub int_val: ::std::vec::Vec<i32>,
    pub string_val: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    pub scomplex_val: ::std::vec::Vec<f32>,
    pub int64_val: ::std::vec::Vec<i64>,
    pub bool_val: ::std::vec::Vec<bool>,
    pub dcomplex_val: ::std::vec::Vec<f64>,
    pub resource_handle_val: ::protobuf::RepeatedField<super::resource_handle::ResourceHandleProto>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TensorProto {
    fn default() -> &'a TensorProto {
        <TensorProto as ::protobuf::Message>::default_instance()
    }
}

impl TensorProto {
    pub fn new() -> TensorProto {
        ::std::default::Default::default()
    }

    // .tensorboardrs.DataType dtype = 1;


    pub fn get_dtype(&self) -> super::types::DataType {
        self.dtype
    }
    pub fn clear_dtype(&mut self) {
        self.dtype = super::types::DataType::DT_INVALID;
    }

    // Param is passed by value, moved
    pub fn set_dtype(&mut self, v: super::types::DataType) {
        self.dtype = v;
    }

    // .tensorboardrs.TensorShapeProto tensor_shape = 2;


    pub fn get_tensor_shape(&self) -> &super::tensor_shape::TensorShapeProto {
        self.tensor_shape.as_ref().unwrap_or_else(|| super::tensor_shape::TensorShapeProto::default_instance())
    }
    pub fn clear_tensor_shape(&mut self) {
        self.tensor_shape.clear();
    }

    pub fn has_tensor_shape(&self) -> bool {
        self.tensor_shape.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tensor_shape(&mut self, v: super::tensor_shape::TensorShapeProto) {
        self.tensor_shape = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tensor_shape(&mut self) -> &mut super::tensor_shape::TensorShapeProto {
        if self.tensor_shape.is_none() {
            self.tensor_shape.set_default();
        }
        self.tensor_shape.as_mut().unwrap()
    }

    // Take field
    pub fn take_tensor_shape(&mut self) -> super::tensor_shape::TensorShapeProto {
        self.tensor_shape.take().unwrap_or_else(|| super::tensor_shape::TensorShapeProto::new())
    }

    // int32 version_number = 3;


    pub fn get_version_number(&self) -> i32 {
        self.version_number
    }
    pub fn clear_version_number(&mut self) {
        self.version_number = 0;
    }

    // Param is passed by value, moved
    pub fn set_version_number(&mut self, v: i32) {
        self.version_number = v;
    }

    // bytes tensor_content = 4;


    pub fn get_tensor_content(&self) -> &[u8] {
        &self.tensor_content
    }
    pub fn clear_tensor_content(&mut self) {
        self.tensor_content.clear();
    }

    // Param is passed by value, moved
    pub fn set_tensor_content(&mut self, v: ::std::vec::Vec<u8>) {
        self.tensor_content = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tensor_content(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.tensor_content
    }

    // Take field
    pub fn take_tensor_content(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.tensor_content, ::std::vec::Vec::new())
    }

    // repeated int32 half_val = 13;


    pub fn get_half_val(&self) -> &[i32] {
        &self.half_val
    }
    pub fn clear_half_val(&mut self) {
        self.half_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_half_val(&mut self, v: ::std::vec::Vec<i32>) {
        self.half_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_half_val(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.half_val
    }

    // Take field
    pub fn take_half_val(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.half_val, ::std::vec::Vec::new())
    }

    // repeated float float_val = 5;


    pub fn get_float_val(&self) -> &[f32] {
        &self.float_val
    }
    pub fn clear_float_val(&mut self) {
        self.float_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_float_val(&mut self, v: ::std::vec::Vec<f32>) {
        self.float_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_float_val(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.float_val
    }

    // Take field
    pub fn take_float_val(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.float_val, ::std::vec::Vec::new())
    }

    // repeated double double_val = 6;


    pub fn get_double_val(&self) -> &[f64] {
        &self.double_val
    }
    pub fn clear_double_val(&mut self) {
        self.double_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_double_val(&mut self, v: ::std::vec::Vec<f64>) {
        self.double_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_double_val(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.double_val
    }

    // Take field
    pub fn take_double_val(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.double_val, ::std::vec::Vec::new())
    }

    // repeated int32 int_val = 7;


    pub fn get_int_val(&self) -> &[i32] {
        &self.int_val
    }
    pub fn clear_int_val(&mut self) {
        self.int_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_int_val(&mut self, v: ::std::vec::Vec<i32>) {
        self.int_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_int_val(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.int_val
    }

    // Take field
    pub fn take_int_val(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.int_val, ::std::vec::Vec::new())
    }

    // repeated bytes string_val = 8;


    pub fn get_string_val(&self) -> &[::std::vec::Vec<u8>] {
        &self.string_val
    }
    pub fn clear_string_val(&mut self) {
        self.string_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_string_val(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.string_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_string_val(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.string_val
    }

    // Take field
    pub fn take_string_val(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.string_val, ::protobuf::RepeatedField::new())
    }

    // repeated float scomplex_val = 9;


    pub fn get_scomplex_val(&self) -> &[f32] {
        &self.scomplex_val
    }
    pub fn clear_scomplex_val(&mut self) {
        self.scomplex_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_scomplex_val(&mut self, v: ::std::vec::Vec<f32>) {
        self.scomplex_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_scomplex_val(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.scomplex_val
    }

    // Take field
    pub fn take_scomplex_val(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.scomplex_val, ::std::vec::Vec::new())
    }

    // repeated int64 int64_val = 10;


    pub fn get_int64_val(&self) -> &[i64] {
        &self.int64_val
    }
    pub fn clear_int64_val(&mut self) {
        self.int64_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_int64_val(&mut self, v: ::std::vec::Vec<i64>) {
        self.int64_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_int64_val(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.int64_val
    }

    // Take field
    pub fn take_int64_val(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.int64_val, ::std::vec::Vec::new())
    }

    // repeated bool bool_val = 11;


    pub fn get_bool_val(&self) -> &[bool] {
        &self.bool_val
    }
    pub fn clear_bool_val(&mut self) {
        self.bool_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_bool_val(&mut self, v: ::std::vec::Vec<bool>) {
        self.bool_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_bool_val(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.bool_val
    }

    // Take field
    pub fn take_bool_val(&mut self) -> ::std::vec::Vec<bool> {
        ::std::mem::replace(&mut self.bool_val, ::std::vec::Vec::new())
    }

    // repeated double dcomplex_val = 12;


    pub fn get_dcomplex_val(&self) -> &[f64] {
        &self.dcomplex_val
    }
    pub fn clear_dcomplex_val(&mut self) {
        self.dcomplex_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_dcomplex_val(&mut self, v: ::std::vec::Vec<f64>) {
        self.dcomplex_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_dcomplex_val(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.dcomplex_val
    }

    // Take field
    pub fn take_dcomplex_val(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.dcomplex_val, ::std::vec::Vec::new())
    }

    // repeated .tensorboardrs.ResourceHandleProto resource_handle_val = 14;


    pub fn get_resource_handle_val(&self) -> &[super::resource_handle::ResourceHandleProto] {
        &self.resource_handle_val
    }
    pub fn clear_resource_handle_val(&mut self) {
        self.resource_handle_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_resource_handle_val(&mut self, v: ::protobuf::RepeatedField<super::resource_handle::ResourceHandleProto>) {
        self.resource_handle_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_resource_handle_val(&mut self) -> &mut ::protobuf::RepeatedField<super::resource_handle::ResourceHandleProto> {
        &mut self.resource_handle_val
    }

    // Take field
    pub fn take_resource_handle_val(&mut self) -> ::protobuf::RepeatedField<super::resource_handle::ResourceHandleProto> {
        ::std::mem::replace(&mut self.resource_handle_val, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for TensorProto {
    fn is_initialized(&self) -> bool {
        for v in &self.tensor_shape {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.resource_handle_val {
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
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.dtype, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tensor_shape)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.version_number = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.tensor_content)?;
                },
                13 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.half_val)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.float_val)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.double_val)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.int_val)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.string_val)?;
                },
                9 => {
                    ::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.scomplex_val)?;
                },
                10 => {
                    ::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.int64_val)?;
                },
                11 => {
                    ::protobuf::rt::read_repeated_bool_into(wire_type, is, &mut self.bool_val)?;
                },
                12 => {
                    ::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.dcomplex_val)?;
                },
                14 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.resource_handle_val)?;
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
        if self.dtype != super::types::DataType::DT_INVALID {
            my_size += ::protobuf::rt::enum_size(1, self.dtype);
        }
        if let Some(ref v) = self.tensor_shape.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.version_number != 0 {
            my_size += ::protobuf::rt::value_size(3, self.version_number, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.tensor_content.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.tensor_content);
        }
        if !self.half_val.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(13, &self.half_val);
        }
        if !self.float_val.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size((self.float_val.len() * 4) as u32) + (self.float_val.len() * 4) as u32;
        }
        if !self.double_val.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size((self.double_val.len() * 8) as u32) + (self.double_val.len() * 8) as u32;
        }
        if !self.int_val.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(7, &self.int_val);
        }
        for value in &self.string_val {
            my_size += ::protobuf::rt::bytes_size(8, &value);
        };
        if !self.scomplex_val.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size((self.scomplex_val.len() * 4) as u32) + (self.scomplex_val.len() * 4) as u32;
        }
        if !self.int64_val.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(10, &self.int64_val);
        }
        if !self.bool_val.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size((self.bool_val.len() * 1) as u32) + (self.bool_val.len() * 1) as u32;
        }
        if !self.dcomplex_val.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size((self.dcomplex_val.len() * 8) as u32) + (self.dcomplex_val.len() * 8) as u32;
        }
        for value in &self.resource_handle_val {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.dtype != super::types::DataType::DT_INVALID {
            os.write_enum(1, self.dtype.value())?;
        }
        if let Some(ref v) = self.tensor_shape.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.version_number != 0 {
            os.write_int32(3, self.version_number)?;
        }
        if !self.tensor_content.is_empty() {
            os.write_bytes(4, &self.tensor_content)?;
        }
        if !self.half_val.is_empty() {
            os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.half_val))?;
            for v in &self.half_val {
                os.write_int32_no_tag(*v)?;
            };
        }
        if !self.float_val.is_empty() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.float_val.len() * 4) as u32)?;
            for v in &self.float_val {
                os.write_float_no_tag(*v)?;
            };
        }
        if !self.double_val.is_empty() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.double_val.len() * 8) as u32)?;
            for v in &self.double_val {
                os.write_double_no_tag(*v)?;
            };
        }
        if !self.int_val.is_empty() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.int_val))?;
            for v in &self.int_val {
                os.write_int32_no_tag(*v)?;
            };
        }
        for v in &self.string_val {
            os.write_bytes(8, &v)?;
        };
        if !self.scomplex_val.is_empty() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.scomplex_val.len() * 4) as u32)?;
            for v in &self.scomplex_val {
                os.write_float_no_tag(*v)?;
            };
        }
        if !self.int64_val.is_empty() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.int64_val))?;
            for v in &self.int64_val {
                os.write_int64_no_tag(*v)?;
            };
        }
        if !self.bool_val.is_empty() {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.bool_val.len() * 1) as u32)?;
            for v in &self.bool_val {
                os.write_bool_no_tag(*v)?;
            };
        }
        if !self.dcomplex_val.is_empty() {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.dcomplex_val.len() * 8) as u32)?;
            for v in &self.dcomplex_val {
                os.write_double_no_tag(*v)?;
            };
        }
        for v in &self.resource_handle_val {
            os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> TensorProto {
        TensorProto::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::types::DataType>>(
                    "dtype",
                    |m: &TensorProto| { &m.dtype },
                    |m: &mut TensorProto| { &mut m.dtype },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::tensor_shape::TensorShapeProto>>(
                    "tensor_shape",
                    |m: &TensorProto| { &m.tensor_shape },
                    |m: &mut TensorProto| { &mut m.tensor_shape },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "version_number",
                    |m: &TensorProto| { &m.version_number },
                    |m: &mut TensorProto| { &mut m.version_number },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "tensor_content",
                    |m: &TensorProto| { &m.tensor_content },
                    |m: &mut TensorProto| { &mut m.tensor_content },
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "half_val",
                    |m: &TensorProto| { &m.half_val },
                    |m: &mut TensorProto| { &mut m.half_val },
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "float_val",
                    |m: &TensorProto| { &m.float_val },
                    |m: &mut TensorProto| { &mut m.float_val },
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "double_val",
                    |m: &TensorProto| { &m.double_val },
                    |m: &mut TensorProto| { &mut m.double_val },
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "int_val",
                    |m: &TensorProto| { &m.int_val },
                    |m: &mut TensorProto| { &mut m.int_val },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "string_val",
                    |m: &TensorProto| { &m.string_val },
                    |m: &mut TensorProto| { &mut m.string_val },
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "scomplex_val",
                    |m: &TensorProto| { &m.scomplex_val },
                    |m: &mut TensorProto| { &mut m.scomplex_val },
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "int64_val",
                    |m: &TensorProto| { &m.int64_val },
                    |m: &mut TensorProto| { &mut m.int64_val },
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "bool_val",
                    |m: &TensorProto| { &m.bool_val },
                    |m: &mut TensorProto| { &mut m.bool_val },
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "dcomplex_val",
                    |m: &TensorProto| { &m.dcomplex_val },
                    |m: &mut TensorProto| { &mut m.dcomplex_val },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::resource_handle::ResourceHandleProto>>(
                    "resource_handle_val",
                    |m: &TensorProto| { &m.resource_handle_val },
                    |m: &mut TensorProto| { &mut m.resource_handle_val },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<TensorProto>(
                    "TensorProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static TensorProto {
        static mut instance: ::protobuf::lazy::Lazy<TensorProto> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(TensorProto::new)
        }
    }
}

impl ::protobuf::Clear for TensorProto {
    fn clear(&mut self) {
        self.dtype = super::types::DataType::DT_INVALID;
        self.tensor_shape.clear();
        self.version_number = 0;
        self.tensor_content.clear();
        self.half_val.clear();
        self.float_val.clear();
        self.double_val.clear();
        self.int_val.clear();
        self.string_val.clear();
        self.scomplex_val.clear();
        self.int64_val.clear();
        self.bool_val.clear();
        self.dcomplex_val.clear();
        self.resource_handle_val.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TensorProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TensorProto {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n$src/tensorboardrs/proto/tensor.proto\x12\rtensorboardrs\x1a-src/tenso\
    rboardrs/proto/resource_handle.proto\x1a*src/tensorboardrs/proto/tensor_\
    shape.proto\x1a#src/tensorboardrs/proto/types.proto\"\xcf\x04\n\x0bTenso\
    rProto\x12-\n\x05dtype\x18\x01\x20\x01(\x0e2\x17.tensorboardrs.DataTypeR\
    \x05dtype\x12B\n\x0ctensor_shape\x18\x02\x20\x01(\x0b2\x1f.tensorboardrs\
    .TensorShapeProtoR\x0btensorShape\x12%\n\x0eversion_number\x18\x03\x20\
    \x01(\x05R\rversionNumber\x12%\n\x0etensor_content\x18\x04\x20\x01(\x0cR\
    \rtensorContent\x12\x1d\n\x08half_val\x18\r\x20\x03(\x05R\x07halfValB\
    \x02\x10\x01\x12\x1f\n\tfloat_val\x18\x05\x20\x03(\x02R\x08floatValB\x02\
    \x10\x01\x12!\n\ndouble_val\x18\x06\x20\x03(\x01R\tdoubleValB\x02\x10\
    \x01\x12\x1b\n\x07int_val\x18\x07\x20\x03(\x05R\x06intValB\x02\x10\x01\
    \x12\x1d\n\nstring_val\x18\x08\x20\x03(\x0cR\tstringVal\x12%\n\x0cscompl\
    ex_val\x18\t\x20\x03(\x02R\x0bscomplexValB\x02\x10\x01\x12\x1f\n\tint64_\
    val\x18\n\x20\x03(\x03R\x08int64ValB\x02\x10\x01\x12\x1d\n\x08bool_val\
    \x18\x0b\x20\x03(\x08R\x07boolValB\x02\x10\x01\x12%\n\x0cdcomplex_val\
    \x18\x0c\x20\x03(\x01R\x0bdcomplexValB\x02\x10\x01\x12R\n\x13resource_ha\
    ndle_val\x18\x0e\x20\x03(\x0b2\".tensorboardrs.ResourceHandleProtoR\x11r\
    esourceHandleValB-\n\x18org.tensorflow.frameworkB\x0cTensorProtosP\x01\
    \xf8\x01\x01J\xee\x16\n\x06\x12\x04\0\0J\x02\n\x08\n\x01\x0c\x12\x03\0\0\
    \x12\n\x08\n\x01\x02\x12\x03\x02\0\x16\n\x08\n\x01\x08\x12\x03\x03\0\x1f\
    \n\t\n\x02\x08\x1f\x12\x03\x03\0\x1f\n\x08\n\x01\x08\x12\x03\x04\0-\n\t\
    \n\x02\x08\x08\x12\x03\x04\0-\n\x08\n\x01\x08\x12\x03\x05\0\"\n\t\n\x02\
    \x08\n\x12\x03\x05\0\"\n\x08\n\x01\x08\x12\x03\x06\01\n\t\n\x02\x08\x01\
    \x12\x03\x06\01\n\t\n\x02\x03\0\x12\x03\x08\07\n\t\n\x02\x03\x01\x12\x03\
    \t\04\n\t\n\x02\x03\x02\x12\x03\n\0-\n4\n\x02\x04\0\x12\x04\r\0J\x01\x1a\
    (\x20Protocol\x20buffer\x20representing\x20a\x20tensor.\n\n\n\n\x03\x04\
    \0\x01\x12\x03\r\x08\x13\n\x0b\n\x04\x04\0\x02\0\x12\x03\x0e\x02\x15\n\
    \x0c\n\x05\x04\0\x02\0\x06\x12\x03\x0e\x02\n\n\x0c\n\x05\x04\0\x02\0\x01\
    \x12\x03\x0e\x0b\x10\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0e\x13\x14\nM\
    \n\x04\x04\0\x02\x01\x12\x03\x11\x02$\x1a@\x20Shape\x20of\x20the\x20tens\
    or.\x20\x20TODO(touts):\x20sort\x20out\x20the\x200-rank\x20issues.\n\n\
    \x0c\n\x05\x04\0\x02\x01\x06\x12\x03\x11\x02\x12\n\x0c\n\x05\x04\0\x02\
    \x01\x01\x12\x03\x11\x13\x1f\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x11\"\
    #\n\xc9\x03\n\x04\x04\0\x02\x02\x12\x03\x1c\x02\x1b\x1a\xd7\x01\x20Versi\
    on\x20number.\n\n\x20In\x20version\x200,\x20if\x20the\x20\"repeated\x20x\
    xx\"\x20representations\x20contain\x20only\x20one\n\x20element,\x20that\
    \x20element\x20is\x20repeated\x20to\x20fill\x20the\x20shape.\x20\x20This\
    \x20makes\x20it\x20easy\n\x20to\x20represent\x20a\x20constant\x20Tensor\
    \x20with\x20a\x20single\x20value.\n2\xe1\x01\x20Only\x20one\x20of\x20the\
    \x20representations\x20below\x20is\x20set,\x20one\x20of\x20\"tensor_cont\
    ents\"\x20and\n\x20the\x20\"xxx_val\"\x20attributes.\x20\x20We\x20are\
    \x20not\x20using\x20oneof\x20because\x20as\x20oneofs\x20cannot\n\x20cont\
    ain\x20repeated\x20fields\x20it\x20would\x20require\x20another\x20extra\
    \x20set\x20of\x20messages.\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x1c\
    \x02\x07\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x1c\x08\x16\n\x0c\n\x05\
    \x04\0\x02\x02\x03\x12\x03\x1c\x19\x1a\n\xd8\x02\n\x04\x04\0\x02\x03\x12\
    \x03#\x02\x1b\x1a\xca\x02\x20Serialized\x20raw\x20tensor\x20content\x20f\
    rom\x20either\x20Tensor::AsProtoTensorContent\x20or\n\x20memcpy\x20in\
    \x20tensorflow::grpc::EncodeTensorToByteBuffer.\x20This\x20representatio\
    n\n\x20can\x20be\x20used\x20for\x20all\x20tensor\x20types.\x20The\x20pur\
    pose\x20of\x20this\x20representation\x20is\x20to\n\x20reduce\x20serializ\
    ation\x20overhead\x20during\x20RPC\x20call\x20by\x20avoiding\x20serializ\
    ation\x20of\n\x20many\x20repeated\x20small\x20items.\n\n\x0c\n\x05\x04\0\
    \x02\x03\x05\x12\x03#\x02\x07\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03#\x08\
    \x16\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03#\x19\x1a\n\xef\x02\n\x04\x04\
    \0\x02\x04\x12\x03,\x02/\x1as\x20DT_HALF.\x20Note\x20that\x20since\x20pr\
    otobuf\x20has\x20no\x20int16\x20type,\x20we'll\x20have\x20some\n\x20poin\
    tless\x20zero\x20padding\x20for\x20each\x20value\x20here.\n2\xec\x01\x20\
    Type\x20specific\x20representations\x20that\x20make\x20it\x20easy\x20to\
    \x20create\x20tensor\x20protos\x20in\n\x20all\x20languages.\x20\x20Only\
    \x20the\x20representation\x20corresponding\x20to\x20\"dtype\"\x20can\n\
    \x20be\x20set.\x20\x20The\x20values\x20hold\x20the\x20flattened\x20repre\
    sentation\x20of\x20the\x20tensor\x20in\n\x20row\x20major\x20order.\n\n\
    \x0c\n\x05\x04\0\x02\x04\x04\x12\x03,\x02\n\n\x0c\n\x05\x04\0\x02\x04\
    \x05\x12\x03,\x0b\x10\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03,\x11\x19\n\
    \x0c\n\x05\x04\0\x02\x04\x03\x12\x03,\x1c\x1e\n\x0c\n\x05\x04\0\x02\x04\
    \x08\x12\x03,\x1f.\n\r\n\x06\x04\0\x02\x04\x08\x02\x12\x03,\x20-\n\x18\n\
    \x04\x04\0\x02\x05\x12\x03/\x02/\x1a\x0b\x20DT_FLOAT.\n\n\x0c\n\x05\x04\
    \0\x02\x05\x04\x12\x03/\x02\n\n\x0c\n\x05\x04\0\x02\x05\x05\x12\x03/\x0b\
    \x10\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03/\x11\x1a\n\x0c\n\x05\x04\0\
    \x02\x05\x03\x12\x03/\x1d\x1e\n\x0c\n\x05\x04\0\x02\x05\x08\x12\x03/\x1f\
    .\n\r\n\x06\x04\0\x02\x05\x08\x02\x12\x03/\x20-\n\x19\n\x04\x04\0\x02\
    \x06\x12\x032\x021\x1a\x0c\x20DT_DOUBLE.\n\n\x0c\n\x05\x04\0\x02\x06\x04\
    \x12\x032\x02\n\n\x0c\n\x05\x04\0\x02\x06\x05\x12\x032\x0b\x11\n\x0c\n\
    \x05\x04\0\x02\x06\x01\x12\x032\x12\x1c\n\x0c\n\x05\x04\0\x02\x06\x03\
    \x12\x032\x1f\x20\n\x0c\n\x05\x04\0\x02\x06\x08\x12\x032!0\n\r\n\x06\x04\
    \0\x02\x06\x08\x02\x12\x032\"/\n5\n\x04\x04\0\x02\x07\x12\x035\x02-\x1a(\
    \x20DT_INT32,\x20DT_INT16,\x20DT_INT8,\x20DT_UINT8.\n\n\x0c\n\x05\x04\0\
    \x02\x07\x04\x12\x035\x02\n\n\x0c\n\x05\x04\0\x02\x07\x05\x12\x035\x0b\
    \x10\n\x0c\n\x05\x04\0\x02\x07\x01\x12\x035\x11\x18\n\x0c\n\x05\x04\0\
    \x02\x07\x03\x12\x035\x1b\x1c\n\x0c\n\x05\x04\0\x02\x07\x08\x12\x035\x1d\
    ,\n\r\n\x06\x04\0\x02\x07\x08\x02\x12\x035\x1e+\n\x18\n\x04\x04\0\x02\
    \x08\x12\x038\x02\x20\x1a\x0b\x20DT_STRING\n\n\x0c\n\x05\x04\0\x02\x08\
    \x04\x12\x038\x02\n\n\x0c\n\x05\x04\0\x02\x08\x05\x12\x038\x0b\x10\n\x0c\
    \n\x05\x04\0\x02\x08\x01\x12\x038\x11\x1b\n\x0c\n\x05\x04\0\x02\x08\x03\
    \x12\x038\x1e\x1f\n\x86\x01\n\x04\x04\0\x02\t\x12\x03<\x022\x1ay\x20DT_C\
    OMPLEX64.\x20scomplex_val(2*i)\x20and\x20scomplex_val(2*i+1)\x20are\x20r\
    eal\n\x20and\x20imaginary\x20parts\x20of\x20i-th\x20single\x20precision\
    \x20complex.\n\n\x0c\n\x05\x04\0\x02\t\x04\x12\x03<\x02\n\n\x0c\n\x05\
    \x04\0\x02\t\x05\x12\x03<\x0b\x10\n\x0c\n\x05\x04\0\x02\t\x01\x12\x03<\
    \x11\x1d\n\x0c\n\x05\x04\0\x02\t\x03\x12\x03<\x20!\n\x0c\n\x05\x04\0\x02\
    \t\x08\x12\x03<\"1\n\r\n\x06\x04\0\x02\t\x08\x02\x12\x03<#0\n\x17\n\x04\
    \x04\0\x02\n\x12\x03?\x020\x1a\n\x20DT_INT64\n\n\x0c\n\x05\x04\0\x02\n\
    \x04\x12\x03?\x02\n\n\x0c\n\x05\x04\0\x02\n\x05\x12\x03?\x0b\x10\n\x0c\n\
    \x05\x04\0\x02\n\x01\x12\x03?\x11\x1a\n\x0c\n\x05\x04\0\x02\n\x03\x12\
    \x03?\x1d\x1f\n\x0c\n\x05\x04\0\x02\n\x08\x12\x03?\x20/\n\r\n\x06\x04\0\
    \x02\n\x08\x02\x12\x03?!.\n\x16\n\x04\x04\0\x02\x0b\x12\x03B\x02.\x1a\t\
    \x20DT_BOOL\n\n\x0c\n\x05\x04\0\x02\x0b\x04\x12\x03B\x02\n\n\x0c\n\x05\
    \x04\0\x02\x0b\x05\x12\x03B\x0b\x0f\n\x0c\n\x05\x04\0\x02\x0b\x01\x12\
    \x03B\x10\x18\n\x0c\n\x05\x04\0\x02\x0b\x03\x12\x03B\x1b\x1d\n\x0c\n\x05\
    \x04\0\x02\x0b\x08\x12\x03B\x1e-\n\r\n\x06\x04\0\x02\x0b\x08\x02\x12\x03\
    B\x1f,\n\x87\x01\n\x04\x04\0\x02\x0c\x12\x03F\x024\x1az\x20DT_COMPLEX128\
    .\x20dcomplex_val(2*i)\x20and\x20dcomplex_val(2*i+1)\x20are\x20real\n\
    \x20and\x20imaginary\x20parts\x20of\x20i-th\x20double\x20precision\x20co\
    mplex.\n\n\x0c\n\x05\x04\0\x02\x0c\x04\x12\x03F\x02\n\n\x0c\n\x05\x04\0\
    \x02\x0c\x05\x12\x03F\x0b\x11\n\x0c\n\x05\x04\0\x02\x0c\x01\x12\x03F\x12\
    \x1e\n\x0c\n\x05\x04\0\x02\x0c\x03\x12\x03F!#\n\x0c\n\x05\x04\0\x02\x0c\
    \x08\x12\x03F$3\n\r\n\x06\x04\0\x02\x0c\x08\x02\x12\x03F%2\n\x1a\n\x04\
    \x04\0\x02\r\x12\x03I\x028\x1a\r\x20DT_RESOURCE\n\n\x0c\n\x05\x04\0\x02\
    \r\x04\x12\x03I\x02\n\n\x0c\n\x05\x04\0\x02\r\x06\x12\x03I\x0b\x1e\n\x0c\
    \n\x05\x04\0\x02\r\x01\x12\x03I\x1f2\n\x0c\n\x05\x04\0\x02\r\x03\x12\x03\
    I57b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
