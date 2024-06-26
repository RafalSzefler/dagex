use std::collections::HashMap;

use dagex::{core::{ArrowDTO, DirectedGraphDTO}, phylo::PhylogeneticNetworkDTO};
use dagex_serialization::{binary::BinaryDeserializer, Deserializer};
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
fn test_u32_deserialization(#[case] expected: u32, #[case] input: &[u8]) {
    let mut deserializer = BinaryDeserializer::from_stream(input);
    let result = deserializer.read::<u32>().unwrap().release();
    assert_eq!(result.read_bytes, input.len());
    assert_eq!(result.item, expected);
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
fn test_i32_deserialization(#[case] expected: i32, #[case] input: &[u8]) {
    let mut deserializer = BinaryDeserializer::from_stream(input);
    let result = deserializer.read::<i32>().unwrap().release();
    assert_eq!(result.read_bytes, input.len());
    assert_eq!(result.item, expected);
}

#[rstest]
#[case("", &[0b00000001])]
#[case("a", &[0b00000011, 0b01100001])]
#[case("A", &[0b00000011, 0b01000001])]
#[case("xyz", &[0b00000111, 0b01111000, 0b01111001, 0b01111010])]
fn test_imm_deserialization(#[case] expected: &str, #[case] input: &[u8]) {
    let expected_imm = ImmutableString::get(expected).unwrap();
    let mut deserializer = BinaryDeserializer::from_stream(input);
    let result = deserializer.read::<ImmutableString>().unwrap().release();
    assert_eq!(result.read_bytes, input.len());
    assert_eq!(result.item, expected_imm);
}


#[rstest]
#[case(0, 0, &[0b00000001, 0b00000001])]
#[case(-1, 7, &[0b00000011, 0b00011101])]
#[case(532, -12346, &[0b01010000, 0b00010001, 0b11100110, 0b10000000, 0b00000011])]
fn test_arrow_deserialization(#[case] source: i32, #[case] target: i32, #[case] input: &[u8]) {
    let expected_arr = ArrowDTO::new(source, target);
    let mut deserializer = BinaryDeserializer::from_stream(input);
    let result = deserializer.read::<ArrowDTO>().unwrap().release();
    assert_eq!(result.read_bytes, input.len());
    assert_eq!(result.item, expected_arr);
}


const DG_PN_SHARED: &[u8] = &[0b00001101, 0b00000101, 0b00000001, 0b00000101, 0b00000001, 0b00001001, 0b00000001];

#[test]
fn test_dg_deserialization() {
    let arrows = vec![ArrowDTO::new(0, 1), ArrowDTO::new(0, 2)];
    let dg = DirectedGraphDTO::new(3, arrows);
    let mut deserializer = BinaryDeserializer::from_stream(DG_PN_SHARED);
    let result = deserializer.read::<DirectedGraphDTO>().unwrap().release();
    assert_eq!(result.read_bytes, DG_PN_SHARED.len());
    assert_eq!(result.item, dg);
}


#[test]
fn test_pn_deserialization_1() {
    let arrows = vec![ArrowDTO::new(0, 1), ArrowDTO::new(0, 2)];
    let dg = DirectedGraphDTO::new(3, arrows);
    let pn = PhylogeneticNetworkDTO::new(dg, HashMap::new());
    let mut deserializer = BinaryDeserializer::from_stream(DG_PN_SHARED);
    let result = deserializer.read::<PhylogeneticNetworkDTO>().unwrap().release();
    assert_eq!(result.read_bytes, DG_PN_SHARED.len());
    assert_eq!(result.item, pn);
}

#[test]
fn test_pn_deserialization_2() {
    // The purpose of loop is to ensure that result doesn't depend on the
    // order of iteration of HashMap.
    let buffer = [
        0b00001101, 0b00000101, 0b00000001, 0b00000101, 0b00000001,
        0b00001001, 0b00000101, 0b00000101, 0b00000011, 0b01000001,
        0b00001001, 0b00000011, 0b01000010];
    let expected: &[u8] = &buffer;

    for _ in 0..100 {
        let arrows = vec![ArrowDTO::new(0, 1), ArrowDTO::new(0, 2)];
        let dg = DirectedGraphDTO::new(3, arrows);
        let mut taxa = HashMap::new();
        taxa.insert(1, ImmutableString::get("A").unwrap());
        taxa.insert(2, ImmutableString::get("B").unwrap());
        let pn = PhylogeneticNetworkDTO::new(dg, taxa);
        let mut deserializer = BinaryDeserializer::from_stream(expected);
        let result = deserializer.read::<PhylogeneticNetworkDTO>().unwrap().release();
        assert_eq!(result.read_bytes, expected.len());
        assert_eq!(result.item, pn);
    }
}


#[test]
fn test_sequential_deserialization() {
    let buffer = [0b01010000, 0b00010001, 0b11100110, 0b10000000, 0b00000011];
    let buffer_ref: &[u8] = &buffer;

    let mut deserializer = BinaryDeserializer::from_stream(buffer_ref);
    let result1 = deserializer.read::<i32>().unwrap().release();
    let result2 = deserializer.read::<i32>().unwrap().release();
    assert_eq!(result1.item, 532);
    assert_eq!(result2.item, -12346);
}
