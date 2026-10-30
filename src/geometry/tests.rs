use super::*;
use crate::cmn::{
    CompressedAttributes, CompressedAttributesEncoding, GeometryColor, GeometryColorType,
    GeometryFaceRange, GeometryFaceRangeType, GeometryNormal, GeometryNormalType, GeometryPosition,
    GeometryPositionType, GeometryUv, GeometryUvType,
};

fn f32s(values: &[f32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn position_only() -> GeometryBuffer {
    GeometryBuffer {
        position: Some(GeometryPosition {
            component: 3,
            r#type: GeometryPositionType::Float32,
            ..Default::default()
        }),
        ..Default::default()
    }
}

#[test]
fn decodes_positions() {
    let bytes = f32s(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let geometry =
        decode_geometry(&position_only(), &bytes, GeometryCounts::vertices(2)).expect("decode");
    assert_eq!(
        geometry.position.unwrap(),
        GeometryValues::F32(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0])
    );
    assert_eq!(geometry.vertex_count, 2);
}

#[test]
fn honours_offset_header() {
    let definition = GeometryBuffer {
        offset: Some(8),
        ..position_only()
    };
    let mut bytes = vec![0xAA; 8];
    bytes.extend(f32s(&[1.0, 2.0, 3.0]));
    let geometry =
        decode_geometry(&definition, &bytes, GeometryCounts::vertices(1)).expect("decode");
    assert_eq!(
        geometry.position.unwrap().as_f32(),
        Some(&[1.0, 2.0, 3.0][..])
    );
}

#[test]
fn reads_attributes_in_the_specified_fixed_order() {
    // position(f32x3) then normal(f32x3) then uv0(f32x2) then color(u8x4).
    // If the decoder read these in any other order the values would be
    // misinterpreted, so distinct values pin the ordering down.
    let definition = GeometryBuffer {
        position: Some(GeometryPosition {
            component: 3,
            r#type: GeometryPositionType::Float32,
            ..Default::default()
        }),
        normal: Some(GeometryNormal {
            component: 3,
            r#type: GeometryNormalType::Float32,
            ..Default::default()
        }),
        uv0: Some(GeometryUv {
            component: 2,
            r#type: GeometryUvType::Float32,
            ..Default::default()
        }),
        color: Some(GeometryColor {
            component: 4,
            r#type: GeometryColorType::UInt8,
            ..Default::default()
        }),
        ..Default::default()
    };
    let mut bytes = f32s(&[1.0, 2.0, 3.0]);
    bytes.extend(f32s(&[0.0, 0.0, 1.0]));
    bytes.extend(f32s(&[0.25, 0.75]));
    bytes.extend_from_slice(&[10, 20, 30, 255]);

    let geometry =
        decode_geometry(&definition, &bytes, GeometryCounts::vertices(1)).expect("decode");
    assert_eq!(
        geometry.position.unwrap().as_f32(),
        Some(&[1.0, 2.0, 3.0][..])
    );
    assert_eq!(
        geometry.normal.unwrap().as_f32(),
        Some(&[0.0, 0.0, 1.0][..])
    );
    assert_eq!(geometry.uv0.unwrap().as_f32(), Some(&[0.25, 0.75][..]));
    assert_eq!(
        geometry.color.unwrap(),
        GeometryValues::U8(vec![10, 20, 30, 255])
    );
}

#[test]
fn unaligned_color_block_does_not_break_following_reads() {
    // A 3-component UInt8 colour leaves the cursor 4-byte-unaligned. The
    // per-feature faceRange that follows must still read correctly.
    let definition = GeometryBuffer {
        color: Some(GeometryColor {
            component: 3,
            r#type: GeometryColorType::UInt8,
            ..Default::default()
        }),
        face_range: Some(GeometryFaceRange {
            component: 2,
            r#type: GeometryFaceRangeType::UInt32,
            ..Default::default()
        }),
        ..Default::default()
    };
    let mut bytes = vec![7u8, 8, 9];
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&5u32.to_le_bytes());

    let counts = GeometryCounts {
        vertex_count: 1,
        feature_count: 1,
    };
    let geometry = decode_geometry(&definition, &bytes, counts).expect("decode");
    assert_eq!(geometry.color.unwrap(), GeometryValues::U8(vec![7, 8, 9]));
    // faceRange is inclusive: triangles 0..=5 belong to feature 0.
    assert_eq!(
        geometry.face_range.unwrap(),
        GeometryValues::U32(vec![0, 5])
    );
}

#[test]
fn per_feature_attributes_use_the_feature_count() {
    let definition = GeometryBuffer {
        face_range: Some(GeometryFaceRange {
            component: 2,
            r#type: GeometryFaceRangeType::UInt32,
            ..Default::default()
        }),
        ..Default::default()
    };
    let bytes: Vec<u8> = [0u32, 3, 4, 9]
        .iter()
        .flat_map(|v| v.to_le_bytes())
        .collect();
    let counts = GeometryCounts {
        vertex_count: 100,
        feature_count: 2,
    };
    let geometry = decode_geometry(&definition, &bytes, counts).expect("decode");
    assert_eq!(
        geometry.face_range.unwrap(),
        GeometryValues::U32(vec![0, 3, 4, 9])
    );
}

#[test]
fn draco_buffer_is_reported_not_guessed() {
    let definition = GeometryBuffer {
        compressed_attributes: Some(CompressedAttributes {
            encoding: CompressedAttributesEncoding::Draco,
            attributes: Vec::new(),
        }),
        ..Default::default()
    };
    assert_eq!(
        decode_geometry(&definition, &[], GeometryCounts::default()),
        Err(GeometryError::Compressed)
    );
}

#[test]
fn short_buffer_is_an_error() {
    let bytes = f32s(&[1.0, 2.0]);
    assert!(matches!(
        decode_geometry(&position_only(), &bytes, GeometryCounts::vertices(1)),
        Err(GeometryError::UnexpectedEnd { .. })
    ));
}

#[test]
fn trailing_bytes_are_an_error() {
    // A payload longer than the declared layout means the definition and
    // the data disagree; decoding anything from it would be a guess.
    let mut bytes = f32s(&[1.0, 2.0, 3.0]);
    bytes.push(0);
    assert_eq!(
        decode_geometry(&position_only(), &bytes, GeometryCounts::vertices(1)),
        Err(GeometryError::TrailingBytes { trailing: 1 })
    );
}

#[test]
fn encode_round_trips_every_attribute() {
    let definition = GeometryBuffer {
        position: Some(GeometryPosition {
            component: 3,
            r#type: GeometryPositionType::Float32,
            ..Default::default()
        }),
        color: Some(GeometryColor {
            component: 4,
            r#type: GeometryColorType::UInt8,
            ..Default::default()
        }),
        face_range: Some(GeometryFaceRange {
            component: 2,
            r#type: GeometryFaceRangeType::UInt32,
            ..Default::default()
        }),
        ..Default::default()
    };
    let mut bytes = f32s(&[1.0, 2.0, 3.0]);
    bytes.extend_from_slice(&[1, 2, 3, 4]);
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&2u32.to_le_bytes());

    let counts = GeometryCounts {
        vertex_count: 1,
        feature_count: 1,
    };
    let geometry = decode_geometry(&definition, &bytes, counts).expect("decode");
    assert_eq!(encode_geometry(&definition, &geometry).unwrap(), bytes);
}

#[test]
fn decodes_a_legacy_default_geometry_schema_buffer() {
    use crate::cmn::{
        DefaultGeometrySchema, FeatureAttribute, HeaderAttribute, Value, VertexAttribute,
    };

    // Legacy buffers carry their counts in the header rather than taking
    // them from the node page.
    let schema = DefaultGeometrySchema {
        topology: DefaultGeometrySchemaTopology::PerAttributeArray,
        header: vec![
            HeaderAttribute {
                property: "vertexCount".into(),
                r#type: HeaderAttributeType::UInt32,
            },
            HeaderAttribute {
                property: "featureCount".into(),
                r#type: HeaderAttributeType::UInt32,
            },
        ],
        ordering: vec!["position".into(), "color".into()],
        vertex_attributes: VertexAttribute {
            position: GeometryAttribute {
                value_type: GeometryAttributeValueType::Float32,
                values_per_element: 3,
                byte_offset: None,
            },
            color: Some(GeometryAttribute {
                value_type: GeometryAttributeValueType::UInt8,
                values_per_element: 4,
                byte_offset: None,
            }),
            ..Default::default()
        },
        feature_attribute_order: vec!["id".into(), "faceRange".into()],
        feature_attributes: FeatureAttribute {
            id: Some(Value {
                value_type: "UInt64".into(),
                values_per_element: Some(1),
                ..Default::default()
            }),
            face_range: Some(Value {
                value_type: "UInt32".into(),
                values_per_element: Some(2),
                ..Default::default()
            }),
        },
        ..Default::default()
    };

    let mut bytes = Vec::new();
    bytes.extend_from_slice(&2u32.to_le_bytes()); // vertexCount
    bytes.extend_from_slice(&1u32.to_le_bytes()); // featureCount
    bytes.extend(f32s(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]));
    bytes.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
    bytes.extend_from_slice(&7u64.to_le_bytes());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&1u32.to_le_bytes());

    let geometry = decode_schema_geometry(&schema, &bytes).expect("decode");
    assert_eq!(geometry.vertex_count, 2);
    assert_eq!(geometry.feature_count, 1);
    assert_eq!(
        geometry.position.unwrap(),
        GeometryValues::F32(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0])
    );
    assert_eq!(
        geometry.color.unwrap(),
        GeometryValues::U8(vec![1, 2, 3, 4, 5, 6, 7, 8])
    );
    // `id` is UInt64 per feature; a narrower read would desynchronise the
    // faceRange that follows.
    assert_eq!(geometry.feature_id.unwrap(), GeometryValues::U64(vec![7]));
    assert_eq!(
        geometry.face_range.unwrap(),
        GeometryValues::U32(vec![0, 1])
    );
}

#[test]
fn encode_preserves_the_offset_header_length() {
    let definition = GeometryBuffer {
        offset: Some(8),
        ..position_only()
    };
    let geometry = Geometry {
        position: Some(GeometryValues::F32(vec![1.0, 2.0, 3.0])),
        vertex_count: 1,
        ..Default::default()
    };
    let encoded = encode_geometry(&definition, &geometry).unwrap();
    assert_eq!(encoded.len(), 8 + 12);
    let decoded =
        decode_geometry(&definition, &encoded, GeometryCounts::vertices(1)).expect("decode");
    assert_eq!(decoded.position, geometry.position);
}
