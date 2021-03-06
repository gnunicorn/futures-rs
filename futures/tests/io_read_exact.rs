#![feature(pin, arbitrary_self_types, futures_api)]

extern crate futures;
use futures::executor::block_on;
use futures::io::AsyncReadExt;

#[test]
fn read_exact() {
    let mut reader: &[u8] = &vec![1,2,3,4,5u8];
    let mut out = [0u8; 3];

    let res = block_on(reader.read_exact(&mut out)); // read 3 bytes out
    assert!(res.is_ok());
    assert_eq!(out, [1,2,3]);
    assert_eq!(reader.len(), 2);

    let res = block_on(reader.read_exact(&mut out)); // read another 3 bytes, but only 2 bytes left
    assert!(res.is_err());
    assert_eq!(reader.len(), 0);
}
