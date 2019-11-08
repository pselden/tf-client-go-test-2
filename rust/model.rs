// This file is generated by rust-protobuf 2.8.1. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
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
//! Generated file from `tensorflow_serving/apis/model.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_1;

#[derive(PartialEq,Clone,Default)]
pub struct ModelSpec {
    // message fields
    pub name: ::std::string::String,
    pub signature_name: ::std::string::String,
    // message oneof groups
    pub version_choice: ::std::option::Option<ModelSpec_oneof_version_choice>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ModelSpec {
    fn default() -> &'a ModelSpec {
        <ModelSpec as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum ModelSpec_oneof_version_choice {
    version(::protobuf::well_known_types::Int64Value),
    version_label(::std::string::String),
}

impl ModelSpec {
    pub fn new() -> ModelSpec {
        ::std::default::Default::default()
    }

    // string name = 1;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // .google.protobuf.Int64Value version = 2;


    pub fn get_version(&self) -> &::protobuf::well_known_types::Int64Value {
        match self.version_choice {
            ::std::option::Option::Some(ModelSpec_oneof_version_choice::version(ref v)) => v,
            _ => ::protobuf::well_known_types::Int64Value::default_instance(),
        }
    }
    pub fn clear_version(&mut self) {
        self.version_choice = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        match self.version_choice {
            ::std::option::Option::Some(ModelSpec_oneof_version_choice::version(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::protobuf::well_known_types::Int64Value) {
        self.version_choice = ::std::option::Option::Some(ModelSpec_oneof_version_choice::version(v))
    }

    // Mutable pointer to the field.
    pub fn mut_version(&mut self) -> &mut ::protobuf::well_known_types::Int64Value {
        if let ::std::option::Option::Some(ModelSpec_oneof_version_choice::version(_)) = self.version_choice {
        } else {
            self.version_choice = ::std::option::Option::Some(ModelSpec_oneof_version_choice::version(::protobuf::well_known_types::Int64Value::new()));
        }
        match self.version_choice {
            ::std::option::Option::Some(ModelSpec_oneof_version_choice::version(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_version(&mut self) -> ::protobuf::well_known_types::Int64Value {
        if self.has_version() {
            match self.version_choice.take() {
                ::std::option::Option::Some(ModelSpec_oneof_version_choice::version(v)) => v,
                _ => panic!(),
            }
        } else {
            ::protobuf::well_known_types::Int64Value::new()
        }
    }

    // string version_label = 4;


    pub fn get_version_label(&self) -> &str {
        match self.version_choice {
            ::std::option::Option::Some(ModelSpec_oneof_version_choice::version_label(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_version_label(&mut self) {
        self.version_choice = ::std::option::Option::None;
    }

    pub fn has_version_label(&self) -> bool {
        match self.version_choice {
            ::std::option::Option::Some(ModelSpec_oneof_version_choice::version_label(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_version_label(&mut self, v: ::std::string::String) {
        self.version_choice = ::std::option::Option::Some(ModelSpec_oneof_version_choice::version_label(v))
    }

    // Mutable pointer to the field.
    pub fn mut_version_label(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(ModelSpec_oneof_version_choice::version_label(_)) = self.version_choice {
        } else {
            self.version_choice = ::std::option::Option::Some(ModelSpec_oneof_version_choice::version_label(::std::string::String::new()));
        }
        match self.version_choice {
            ::std::option::Option::Some(ModelSpec_oneof_version_choice::version_label(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_version_label(&mut self) -> ::std::string::String {
        if self.has_version_label() {
            match self.version_choice.take() {
                ::std::option::Option::Some(ModelSpec_oneof_version_choice::version_label(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // string signature_name = 3;


    pub fn get_signature_name(&self) -> &str {
        &self.signature_name
    }
    pub fn clear_signature_name(&mut self) {
        self.signature_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_signature_name(&mut self, v: ::std::string::String) {
        self.signature_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature_name(&mut self) -> &mut ::std::string::String {
        &mut self.signature_name
    }

    // Take field
    pub fn take_signature_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.signature_name, ::std::string::String::new())
    }
}

impl ::protobuf::Message for ModelSpec {
    fn is_initialized(&self) -> bool {
        if let Some(ModelSpec_oneof_version_choice::version(ref v)) = self.version_choice {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.version_choice = ::std::option::Option::Some(ModelSpec_oneof_version_choice::version(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.version_choice = ::std::option::Option::Some(ModelSpec_oneof_version_choice::version_label(is.read_string()?));
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.signature_name)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.signature_name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.signature_name);
        }
        if let ::std::option::Option::Some(ref v) = self.version_choice {
            match v {
                &ModelSpec_oneof_version_choice::version(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ModelSpec_oneof_version_choice::version_label(ref v) => {
                    my_size += ::protobuf::rt::string_size(4, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.signature_name.is_empty() {
            os.write_string(3, &self.signature_name)?;
        }
        if let ::std::option::Option::Some(ref v) = self.version_choice {
            match v {
                &ModelSpec_oneof_version_choice::version(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ModelSpec_oneof_version_choice::version_label(ref v) => {
                    os.write_string(4, v)?;
                },
            };
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
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> ModelSpec {
        ModelSpec::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    |m: &ModelSpec| { &m.name },
                    |m: &mut ModelSpec| { &mut m.name },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ::protobuf::well_known_types::Int64Value>(
                    "version",
                    ModelSpec::has_version,
                    ModelSpec::get_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "version_label",
                    ModelSpec::has_version_label,
                    ModelSpec::get_version_label,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "signature_name",
                    |m: &ModelSpec| { &m.signature_name },
                    |m: &mut ModelSpec| { &mut m.signature_name },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ModelSpec>(
                    "ModelSpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static ModelSpec {
        static mut instance: ::protobuf::lazy::Lazy<ModelSpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ModelSpec,
        };
        unsafe {
            instance.get(ModelSpec::new)
        }
    }
}

impl ::protobuf::Clear for ModelSpec {
    fn clear(&mut self) {
        self.name.clear();
        self.version_choice = ::std::option::Option::None;
        self.version_choice = ::std::option::Option::None;
        self.signature_name.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ModelSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ModelSpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#tensorflow_serving/apis/model.proto\x12\x12tensorflow.serving\x1a\x1e\
    google/protobuf/wrappers.proto\"\xb8\x01\n\tModelSpec\x12\x12\n\x04name\
    \x18\x01\x20\x01(\tR\x04name\x127\n\x07version\x18\x02\x20\x01(\x0b2\x1b\
    .google.protobuf.Int64ValueH\0R\x07version\x12%\n\rversion_label\x18\x04\
    \x20\x01(\tH\0R\x0cversionLabel\x12%\n\x0esignature_name\x18\x03\x20\x01\
    (\tR\rsignatureNameB\x10\n\x0eversion_choiceB\x03\xf8\x01\x01J\x90\x08\n\
    \x06\x12\x04\0\0\x20\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\
    \x12\x03\x02\0\x1b\n\x08\n\x01\x08\x12\x03\x03\0\x1f\n\t\n\x02\x08\x1f\
    \x12\x03\x03\0\x1f\n\t\n\x02\x03\0\x12\x03\x05\0(\nS\n\x02\x04\0\x12\x04\
    \x08\0\x20\x01\x1aG\x20Metadata\x20for\x20an\x20inference\x20request\x20\
    such\x20as\x20the\x20model\x20name\x20and\x20version.\n\n\n\n\x03\x04\0\
    \x01\x12\x03\x08\x08\x11\n&\n\x04\x04\0\x02\0\x12\x03\n\x02\x12\x1a\x19\
    \x20Required\x20servable\x20name.\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\
    \n\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\n\t\r\n\x0c\n\x05\x04\0\
    \x02\0\x03\x12\x03\n\x10\x11\n\xbb\x03\n\x04\x04\0\x08\0\x12\x04\x15\x02\
    \x1b\x03\x1a\xac\x03\x20Optional\x20choice\x20of\x20which\x20version\x20\
    of\x20the\x20model\x20to\x20use.\n\n\x20Recommended\x20to\x20be\x20left\
    \x20unset\x20in\x20the\x20common\x20case.\x20Should\x20be\x20specified\
    \x20only\n\x20when\x20there\x20is\x20a\x20strong\x20version\x20consisten\
    cy\x20requirement.\n\n\x20When\x20left\x20unspecified,\x20the\x20system\
    \x20will\x20serve\x20the\x20best\x20available\x20version.\n\x20This\x20i\
    s\x20typically\x20the\x20latest\x20version,\x20though\x20during\x20versi\
    on\x20transitions,\n\x20notably\x20when\x20serving\x20on\x20a\x20fleet\
    \x20of\x20instances,\x20may\x20be\x20either\x20the\x20previous\x20or\n\
    \x20new\x20version.\n\n\x0c\n\x05\x04\0\x08\0\x01\x12\x03\x15\x08\x16\n0\
    \n\x04\x04\0\x02\x01\x12\x03\x17\x04+\x1a#\x20Use\x20this\x20specific\
    \x20version\x20number.\n\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\x17\x04\
    \x1e\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x17\x1f&\n\x0c\n\x05\x04\0\
    \x02\x01\x03\x12\x03\x17)*\n?\n\x04\x04\0\x02\x02\x12\x03\x1a\x04\x1d\
    \x1a2\x20Use\x20the\x20version\x20associated\x20with\x20the\x20given\x20\
    label.\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x1a\x04\n\n\x0c\n\x05\x04\
    \0\x02\x02\x01\x12\x03\x1a\x0b\x18\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\
    \x1a\x1b\x1c\nb\n\x04\x04\0\x02\x03\x12\x03\x1f\x02\x1c\x1aU\x20A\x20nam\
    ed\x20signature\x20to\x20evaluate.\x20If\x20unspecified,\x20the\x20defau\
    lt\x20signature\x20will\n\x20be\x20used.\n\n\x0c\n\x05\x04\0\x02\x03\x05\
    \x12\x03\x1f\x02\x08\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x1f\t\x17\n\
    \x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x1f\x1a\x1bb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

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