use crate::decoder;
use anyhow::Error;

fn test_helper(encoded_data: Vec<u8>, comparison: Vec<u8>) {
    let mut decoder = decoder::Decoder::default();

    let decoded_data = decoder.push(encoded_data);

    eprintln!("{:?}", decoded_data);

    assert_eq!(decoded_data.unwrap().unwrap(), comparison);
}

#[test]
fn basic_decoding() {
    test_helper(vec![5, 1, 1, 1, 1, 0], vec![1, 1, 1, 1])
}

#[test]
fn with_extra_zeroes() {
    test_helper(vec![3, 1, 2, 2, 1, 0], vec![1, 2, 0, 1]);
}

#[test]
fn incomplete_data() {
    let mut decoder = decoder::Decoder::default();
    let test1 = vec![
        37, 115, 111, 117, 116, 50, 44, 51, 48, 46, 48, 48, 48, 48, 48, 48, 44, 51, 48, 46, 48, 48,
        48, 48, 48, 48,
    ];

    let test2 = vec![44, 51, 48, 46, 48, 48, 48, 48, 48, 48, 10, 0];

    let result1 = decoder.push(test1).unwrap();
    let result2 = decoder.push(test2).unwrap().unwrap();

    assert_eq!(result1, None);
    assert_eq!(
        result2,
        vec![
            115, 111, 117, 116, 50, 44, 51, 48, 46, 48, 48, 48, 48, 48, 48, 44, 51, 48, 46, 48, 48,
            48, 48, 48, 48, 44, 51, 48, 46, 48, 48, 48, 48, 48, 48, 10,
        ]
    )
}

#[test]
fn invalid_data() {
    let mut decoder = decoder::Decoder::default();
    let decoded_data = decoder.push(vec![20, 2, 1, 1, 0]);
    eprintln!("{:?}", decoded_data);
    match decoded_data {
        Ok(_) => panic!("the data was valid, but it shouldn't have been"),
        Err(_) => {}
    }
}
