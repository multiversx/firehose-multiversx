// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.31.0
// 	protoc        v3.6.1
// source: sf/multiversx/type/v1/type.proto

package pbmultiversx

import (
	hyperOutportBlocks "github.com/multiversx/mx-chain-ws-connector-firehose-go/data/hyperOutportBlocks"
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type HyperOutportBlock struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	MetaOutportBlock            *hyperOutportBlocks.MetaOutportBlock             `protobuf:"bytes,1,opt,name=MetaOutportBlock,proto3" json:"MetaOutportBlock,omitempty"`
	NotarizedHeadersOutportData []*hyperOutportBlocks.NotarizedHeaderOutportData `protobuf:"bytes,2,rep,name=NotarizedHeadersOutportData,proto3" json:"NotarizedHeadersOutportData,omitempty"`
}

func (x *HyperOutportBlock) Reset() {
	*x = HyperOutportBlock{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sf_multiversx_type_v1_type_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *HyperOutportBlock) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*HyperOutportBlock) ProtoMessage() {}

func (x *HyperOutportBlock) ProtoReflect() protoreflect.Message {
	mi := &file_sf_multiversx_type_v1_type_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use HyperOutportBlock.ProtoReflect.Descriptor instead.
func (*HyperOutportBlock) Descriptor() ([]byte, []int) {
	return file_sf_multiversx_type_v1_type_proto_rawDescGZIP(), []int{0}
}

func (x *HyperOutportBlock) GetMetaOutportBlock() *hyperOutportBlocks.MetaOutportBlock {
	if x != nil {
		return x.MetaOutportBlock
	}
	return nil
}

func (x *HyperOutportBlock) GetNotarizedHeadersOutportData() []*hyperOutportBlocks.NotarizedHeaderOutportData {
	if x != nil {
		return x.NotarizedHeadersOutportData
	}
	return nil
}

var File_sf_multiversx_type_v1_type_proto protoreflect.FileDescriptor

var file_sf_multiversx_type_v1_type_proto_rawDesc = []byte{
	0x0a, 0x20, 0x73, 0x66, 0x2f, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x76, 0x65, 0x72, 0x73, 0x78, 0x2f,
	0x74, 0x79, 0x70, 0x65, 0x2f, 0x76, 0x31, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x12, 0x15, 0x73, 0x66, 0x2e, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x76, 0x65, 0x72, 0x73,
	0x78, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x76, 0x31, 0x1a, 0x2d, 0x73, 0x66, 0x2f, 0x6d, 0x75,
	0x6c, 0x74, 0x69, 0x76, 0x65, 0x72, 0x73, 0x78, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x2f, 0x76, 0x31,
	0x2f, 0x68, 0x79, 0x70, 0x65, 0x72, 0x4f, 0x75, 0x74, 0x70, 0x6f, 0x72, 0x74, 0x42, 0x6c, 0x6f,
	0x63, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xbd, 0x01, 0x0a, 0x11, 0x48, 0x79, 0x70,
	0x65, 0x72, 0x4f, 0x75, 0x74, 0x70, 0x6f, 0x72, 0x74, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x43,
	0x0a, 0x10, 0x4d, 0x65, 0x74, 0x61, 0x4f, 0x75, 0x74, 0x70, 0x6f, 0x72, 0x74, 0x42, 0x6c, 0x6f,
	0x63, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x2e, 0x4d, 0x65, 0x74, 0x61, 0x4f, 0x75, 0x74, 0x70, 0x6f, 0x72, 0x74, 0x42, 0x6c, 0x6f, 0x63,
	0x6b, 0x52, 0x10, 0x4d, 0x65, 0x74, 0x61, 0x4f, 0x75, 0x74, 0x70, 0x6f, 0x72, 0x74, 0x42, 0x6c,
	0x6f, 0x63, 0x6b, 0x12, 0x63, 0x0a, 0x1b, 0x4e, 0x6f, 0x74, 0x61, 0x72, 0x69, 0x7a, 0x65, 0x64,
	0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x4f, 0x75, 0x74, 0x70, 0x6f, 0x72, 0x74, 0x44, 0x61,
	0x74, 0x61, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x2e, 0x4e, 0x6f, 0x74, 0x61, 0x72, 0x69, 0x7a, 0x65, 0x64, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72,
	0x4f, 0x75, 0x74, 0x70, 0x6f, 0x72, 0x74, 0x44, 0x61, 0x74, 0x61, 0x52, 0x1b, 0x4e, 0x6f, 0x74,
	0x61, 0x72, 0x69, 0x7a, 0x65, 0x64, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x4f, 0x75, 0x74,
	0x70, 0x6f, 0x72, 0x74, 0x44, 0x61, 0x74, 0x61, 0x42, 0x51, 0x5a, 0x4f, 0x67, 0x69, 0x74, 0x68,
	0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x76, 0x65, 0x72, 0x73,
	0x78, 0x2f, 0x66, 0x69, 0x72, 0x65, 0x68, 0x6f, 0x73, 0x65, 0x2d, 0x6d, 0x75, 0x6c, 0x74, 0x69,
	0x76, 0x65, 0x72, 0x73, 0x78, 0x2f, 0x70, 0x62, 0x2f, 0x73, 0x66, 0x2f, 0x6d, 0x75, 0x6c, 0x74,
	0x69, 0x76, 0x65, 0x72, 0x73, 0x78, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x2f, 0x76, 0x31, 0x3b, 0x70,
	0x62, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x76, 0x65, 0x72, 0x73, 0x78, 0x62, 0x06, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x33,
}

var (
	file_sf_multiversx_type_v1_type_proto_rawDescOnce sync.Once
	file_sf_multiversx_type_v1_type_proto_rawDescData = file_sf_multiversx_type_v1_type_proto_rawDesc
)

func file_sf_multiversx_type_v1_type_proto_rawDescGZIP() []byte {
	file_sf_multiversx_type_v1_type_proto_rawDescOnce.Do(func() {
		file_sf_multiversx_type_v1_type_proto_rawDescData = protoimpl.X.CompressGZIP(file_sf_multiversx_type_v1_type_proto_rawDescData)
	})
	return file_sf_multiversx_type_v1_type_proto_rawDescData
}

var file_sf_multiversx_type_v1_type_proto_msgTypes = make([]protoimpl.MessageInfo, 1)
var file_sf_multiversx_type_v1_type_proto_goTypes = []interface{}{
	(*HyperOutportBlock)(nil),                             // 0: sf.multiversx.type.v1.HyperOutportBlock
	(*hyperOutportBlocks.MetaOutportBlock)(nil),           // 1: proto.MetaOutportBlock
	(*hyperOutportBlocks.NotarizedHeaderOutportData)(nil), // 2: proto.NotarizedHeaderOutportData
}
var file_sf_multiversx_type_v1_type_proto_depIdxs = []int32{
	1, // 0: sf.multiversx.type.v1.HyperOutportBlock.MetaOutportBlock:type_name -> proto.MetaOutportBlock
	2, // 1: sf.multiversx.type.v1.HyperOutportBlock.NotarizedHeadersOutportData:type_name -> proto.NotarizedHeaderOutportData
	2, // [2:2] is the sub-list for method output_type
	2, // [2:2] is the sub-list for method input_type
	2, // [2:2] is the sub-list for extension type_name
	2, // [2:2] is the sub-list for extension extendee
	0, // [0:2] is the sub-list for field type_name
}

func init() { file_sf_multiversx_type_v1_type_proto_init() }
func file_sf_multiversx_type_v1_type_proto_init() {
	if File_sf_multiversx_type_v1_type_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_sf_multiversx_type_v1_type_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*HyperOutportBlock); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_sf_multiversx_type_v1_type_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   1,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_sf_multiversx_type_v1_type_proto_goTypes,
		DependencyIndexes: file_sf_multiversx_type_v1_type_proto_depIdxs,
		MessageInfos:      file_sf_multiversx_type_v1_type_proto_msgTypes,
	}.Build()
	File_sf_multiversx_type_v1_type_proto = out.File
	file_sf_multiversx_type_v1_type_proto_rawDesc = nil
	file_sf_multiversx_type_v1_type_proto_goTypes = nil
	file_sf_multiversx_type_v1_type_proto_depIdxs = nil
}
