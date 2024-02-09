// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.25.0
// 	protoc        v3.17.3
// source: sf/multiversx/type/v1/type.proto

package pbmultiversx

import (
	_ "github.com/gogo/protobuf/gogoproto"
	proto "github.com/golang/protobuf/proto"
	outport "github.com/multiversx/mx-chain-core-go/data/outport"
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

// This is a compile-time assertion that a sufficiently up-to-date version
// of the legacy proto package is being used.
const _ = proto.ProtoPackageIsVersion4

type Block struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Height          uint64                `protobuf:"varint,1,opt,name=height,proto3" json:"height,omitempty"`
	PrevHash        string                `protobuf:"bytes,2,opt,name=prevHash,proto3" json:"prevHash,omitempty"`
	Timestamp       uint64                `protobuf:"varint,3,opt,name=timestamp,proto3" json:"timestamp,omitempty"`
	MultiversxBlock *outport.OutportBlock `protobuf:"bytes,4,opt,name=MultiversxBlock,proto3" json:"MultiversxBlock,omitempty"`
}

func (x *Block) Reset() {
	*x = Block{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sf_multiversx_type_v1_type_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Block) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Block) ProtoMessage() {}

func (x *Block) ProtoReflect() protoreflect.Message {
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

// Deprecated: Use Block.ProtoReflect.Descriptor instead.
func (*Block) Descriptor() ([]byte, []int) {
	return file_sf_multiversx_type_v1_type_proto_rawDescGZIP(), []int{0}
}

func (x *Block) GetHeight() uint64 {
	if x != nil {
		return x.Height
	}
	return 0
}

func (x *Block) GetPrevHash() string {
	if x != nil {
		return x.PrevHash
	}
	return ""
}

func (x *Block) GetTimestamp() uint64 {
	if x != nil {
		return x.Timestamp
	}
	return 0
}

func (x *Block) GetMultiversxBlock() *outport.OutportBlock {
	if x != nil {
		return x.MultiversxBlock
	}
	return nil
}

var File_sf_multiversx_type_v1_type_proto protoreflect.FileDescriptor

var file_sf_multiversx_type_v1_type_proto_rawDesc = []byte{
	0x0a, 0x20, 0x73, 0x66, 0x2f, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x76, 0x65, 0x72, 0x73, 0x78, 0x2f,
	0x74, 0x79, 0x70, 0x65, 0x2f, 0x76, 0x31, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x12, 0x15, 0x73, 0x66, 0x2e, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x76, 0x65, 0x72, 0x73,
	0x78, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x76, 0x31, 0x1a, 0x2d, 0x67, 0x69, 0x74, 0x68, 0x75,
	0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x67, 0x6f, 0x67, 0x6f, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x62, 0x75, 0x66, 0x2f, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x67, 0x6f,
	0x67, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x46, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62,
	0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x76, 0x65, 0x72, 0x73, 0x78, 0x2f,
	0x6d, 0x78, 0x2d, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x2d, 0x63, 0x6f, 0x72, 0x65, 0x2d, 0x67, 0x6f,
	0x2f, 0x64, 0x61, 0x74, 0x61, 0x2f, 0x6f, 0x75, 0x74, 0x70, 0x6f, 0x72, 0x74, 0x2f, 0x6f, 0x75,
	0x74, 0x70, 0x6f, 0x72, 0x74, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x22, 0x98, 0x01, 0x0a, 0x05, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x16, 0x0a, 0x06, 0x68, 0x65,
	0x69, 0x67, 0x68, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x06, 0x68, 0x65, 0x69, 0x67,
	0x68, 0x74, 0x12, 0x1a, 0x0a, 0x08, 0x70, 0x72, 0x65, 0x76, 0x48, 0x61, 0x73, 0x68, 0x18, 0x02,
	0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x70, 0x72, 0x65, 0x76, 0x48, 0x61, 0x73, 0x68, 0x12, 0x1c,
	0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x03, 0x20, 0x01, 0x28,
	0x04, 0x52, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x3d, 0x0a, 0x0f,
	0x4d, 0x75, 0x6c, 0x74, 0x69, 0x76, 0x65, 0x72, 0x73, 0x78, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x18,
	0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x4f, 0x75,
	0x74, 0x70, 0x6f, 0x72, 0x74, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x52, 0x0f, 0x4d, 0x75, 0x6c, 0x74,
	0x69, 0x76, 0x65, 0x72, 0x73, 0x78, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x42, 0x5f, 0x5a, 0x59, 0x67,
	0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x76,
	0x65, 0x72, 0x73, 0x78, 0x2f, 0x66, 0x69, 0x72, 0x65, 0x68, 0x6f, 0x73, 0x65, 0x2d, 0x6d, 0x75,
	0x6c, 0x74, 0x69, 0x76, 0x65, 0x72, 0x73, 0x78, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2f, 0x70,
	0x62, 0x2f, 0x73, 0x66, 0x2f, 0x66, 0x69, 0x72, 0x65, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x76, 0x65,
	0x72, 0x73, 0x78, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x2f, 0x76, 0x31, 0x3b, 0x70, 0x62, 0x6d, 0x75,
	0x6c, 0x74, 0x69, 0x76, 0x65, 0x72, 0x73, 0x78, 0xd8, 0xe2, 0x1e, 0x01, 0x62, 0x06, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x33,
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
	(*Block)(nil),                // 0: sf.multiversx.type.v1.Block
	(*outport.OutportBlock)(nil), // 1: proto.OutportBlock
}
var file_sf_multiversx_type_v1_type_proto_depIdxs = []int32{
	1, // 0: sf.multiversx.type.v1.Block.MultiversxBlock:type_name -> proto.OutportBlock
	1, // [1:1] is the sub-list for method output_type
	1, // [1:1] is the sub-list for method input_type
	1, // [1:1] is the sub-list for extension type_name
	1, // [1:1] is the sub-list for extension extendee
	0, // [0:1] is the sub-list for field type_name
}

func init() { file_sf_multiversx_type_v1_type_proto_init() }
func file_sf_multiversx_type_v1_type_proto_init() {
	if File_sf_multiversx_type_v1_type_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_sf_multiversx_type_v1_type_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Block); i {
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
