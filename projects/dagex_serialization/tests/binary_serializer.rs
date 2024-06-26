use std::collections::HashMap;

use dagex::{core::{ArrowDTO, DirectedGraphDTO}, phylo::PhylogeneticNetworkDTO};
use dagex_serialization::{binary::BinarySerializer, Serializer};
use immutable_string::ImmutableString;
use rstest::rstest;


// For debugging purposes only.
#[allow(dead_code)]
fn format_data(vec: &Vec<u8>) -> String {
    let mut msg = Vec::with_capacity(vec.len());
    for el in vec.iter() {
        msg.push(format!("0b{:08b}", el));
    }
    "[".to_owned() + msg.join(", ").as_str() + "]"
}

#[rstest]
#[case(0, &[0b00000001])]
#[case(1, &[0b00000011])]
#[case(2, &[0b00000101])]
#[case(3, &[0b00000111])]
#[case(255, &[0b11111110, 0b00000011])]
#[case(1000, &[0b11010000, 0b00001111])]
fn test_u32_serialization(#[case] input: u32, #[case] expected: &[u8]) {
    let mut serializer = BinarySerializer::from_stream(Vec::new());
    let result = serializer.write(&input).unwrap();
    assert_eq!(result.written_bytes(), expected.len());
    let data = serializer.release();
    assert_eq!(data, expected);
}


#[rstest]
#[case(0, &[0b00000001])]
#[case(-1, &[0b00000011])]
#[case(1, &[0b00000101])]
#[case(-2, &[0b000000111])]
#[case(2, &[0b00001001])]
#[case(-3, &[0b000001011])]
#[case(3, &[0b00001101])]
#[case(255, &[0b11111100, 0b00000111])]
#[case(-255, &[0b11111010, 0b00000111])]
fn test_i32_serialization(#[case] input: i32, #[case] expected: &[u8]) {
    let mut serializer = BinarySerializer::from_stream(Vec::new());
    let result = serializer.write(&input).unwrap();
    assert_eq!(result.written_bytes(), expected.len());
    let data = serializer.release();
    assert_eq!(data, expected);
}

#[rstest]
#[case("", &[0b00000001])]
#[case("a", &[0b00000011, 0b01100001])]
#[case("A", &[0b00000011, 0b01000001])]
#[case("xyz", &[0b00000111, 0b01111000, 0b01111001, 0b01111010])]
fn test_imm_serialization(#[case] input: &str, #[case] expected: &[u8]) {
    let value = ImmutableString::get(input).unwrap();
    let mut serializer = BinarySerializer::from_stream(Vec::new());
    let result = serializer.write(&value).unwrap();
    assert_eq!(result.written_bytes(), expected.len());
    let data = serializer.release();
    
    assert_eq!(data, expected);
}


#[rstest]
#[case(0, 0, &[0b00000001, 0b00000001])]
#[case(-1, 7, &[0b00000011, 0b00011101])]
#[case(532, -12346, &[0b01010000, 0b00010001, 0b11100110, 0b10000000, 0b00000011])]
fn test_arrow_serialization(#[case] source: i32, #[case] target: i32, #[case] expected: &[u8]) {
    let arrow = ArrowDTO::new(source, target);
    let mut serializer = BinarySerializer::from_stream(Vec::new());
    let result = serializer.write(&arrow).unwrap();
    assert_eq!(result.written_bytes(), expected.len());
    let data = serializer.release();
    assert_eq!(data, expected);
}


const DG_PN_SHARED: &[u8] = &[0b00001101, 0b00000101, 0b00000001, 0b00000101, 0b00000001, 0b00001001, 0b00000001];

#[test]
fn test_dg_serialization() {
    let arrows = vec![ArrowDTO::new(0, 1), ArrowDTO::new(0, 2)];
    let dg = DirectedGraphDTO::new(3, arrows);
    let mut serializer = BinarySerializer::from_stream(Vec::new());
    let result = serializer.write(&dg).unwrap();
    let written_bytes = result.written_bytes();
    let data = serializer.release();
    assert_eq!(written_bytes, DG_PN_SHARED.len());
    assert_eq!(data, DG_PN_SHARED);
}


#[test]
fn test_pn_serialization_1() {
    let arrows = vec![ArrowDTO::new(0, 1), ArrowDTO::new(0, 2)];
    let dg = DirectedGraphDTO::new(3, arrows);
    let pn = PhylogeneticNetworkDTO::new(dg, HashMap::new());
    let mut serializer = BinarySerializer::from_stream(Vec::new());
    let result = serializer.write(&pn).unwrap();
    let written_bytes = result.written_bytes();
    let data = serializer.release();
    assert_eq!(written_bytes, DG_PN_SHARED.len());
    assert_eq!(data, DG_PN_SHARED);
}

#[test]
fn test_pn_serialization_2() {
    // The purpose of loop is to ensure that result doesn't depend on the
    // order of iteration of HashMap.
    let expected = &[
        0b00001101, 0b00000101, 0b00000001, 0b00000101, 0b00000001,
        0b00001001, 0b00000101, 0b00000101, 0b00000011, 0b01000001,
        0b00001001, 0b00000011, 0b01000010];

    for _ in 0..100 {
        let arrows = vec![ArrowDTO::new(0, 1), ArrowDTO::new(0, 2)];
        let dg = DirectedGraphDTO::new(3, arrows);
        let mut taxa = HashMap::new();
        taxa.insert(1, ImmutableString::get("A").unwrap());
        taxa.insert(2, ImmutableString::get("B").unwrap());
        let pn = PhylogeneticNetworkDTO::new(dg, taxa);
        let mut serializer = BinarySerializer::from_stream(Vec::new());
        let result = serializer.write(&pn).unwrap();
        let written_bytes = result.written_bytes();
        let data = serializer.release();
        if data != expected {
            println!("{}", format_data(&data))
        }
        assert_eq!(written_bytes, expected.len());
        assert_eq!(data, expected);
    }
}
