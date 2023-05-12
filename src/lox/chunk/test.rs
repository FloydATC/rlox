

use super::Chunk;


#[test]
fn new() {
    let _chunk: Chunk = Chunk::new();
}

#[test]
fn write_1_bytes() {
    let mut chunk = Chunk::new();
    chunk.code = vec![0x00];
    chunk.write_bytes(0x12, 0, 1);
    assert_eq!(chunk.code, vec![0x12])
}

#[test]
fn write_2_bytes() {
    let mut chunk = Chunk::new();
    chunk.code = vec![0x00, 0x00];
    chunk.write_bytes(0x1234, 0, 2);
    assert_eq!(chunk.code, vec![0x12, 0x34])
}

#[test]
fn write_4_bytes() {
    let mut chunk = Chunk::new();
    chunk.code = vec![0x00, 0x00, 0x00, 0x00];
    chunk.write_bytes(0x12345678, 0, 4);
    assert_eq!(chunk.code, vec![0x12, 0x34, 0x56, 0x78])
}

#[test]
#[should_panic]
fn write_1_bytes_outside_bounds() {
    let mut chunk = Chunk::new();
    chunk.write_bytes(0x12, 0, 1);
}

#[test]
#[should_panic]
fn write_2_bytes_outside_bounds() {
    let mut chunk = Chunk::new();
    chunk.code = vec![0x00]; // 1 byte short
    chunk.write_bytes(0x1234, 0, 2);
}

#[test]
#[should_panic]
fn write_4_bytes_outside_bounds() {
    let mut chunk = Chunk::new();
    chunk.code = vec![0x00, 0x00, 0x00]; // 1 byte short
    chunk.write_bytes(0x12345678, 0, 4);
}

#[test]
fn read_1_bytes() {
    let mut chunk = Chunk::new();
    chunk.code = vec![0x12];
    let dword = chunk.read_bytes(0, 1);
    assert_eq!(dword, 0x12)
}

#[test]
fn read_2_bytes() {
    let mut chunk = Chunk::new();
    chunk.code = vec![0x12, 0x34];
    let dword = chunk.read_bytes(0, 2);
    assert_eq!(dword, 0x1234)
}

#[test]
fn read_4_bytes() {
    let mut chunk = Chunk::new();
    chunk.code = vec![0x12, 0x34, 0x56, 0x78];
    let dword = chunk.read_bytes(0, 4);
    assert_eq!(dword, 0x12345678)
}

#[test]
#[should_panic]
fn read_1_bytes_outside_bounds() {
    let chunk = Chunk::new();
    let _dword = chunk.read_bytes(0, 1);
}

#[test]
#[should_panic]
fn read_2_bytes_outside_bounds() {
    let mut chunk = Chunk::new();
    chunk.code = vec![0x00]; // 1 byte short
    let _dword = chunk.read_bytes(0, 2);
}

#[test]
#[should_panic]
fn read_4_bytes_outside_bounds() {
    let mut chunk = Chunk::new();
    chunk.code = vec![0x00, 0x00, 0x00]; // 1 byte short
    let _dword = chunk.read_bytes(0, 4);
}

#[test]
fn append_1_bytes() {
    let mut chunk = Chunk::new();
    chunk.code = vec![0xff];
    chunk.append_bytes(0x12, 1);
    assert_eq!(chunk.code, vec![0xff, 0x12])
}

#[test]
fn append_2_bytes() {
    let mut chunk = Chunk::new();
    chunk.code = vec![0xff];
    chunk.append_bytes(0x1234, 2);
    assert_eq!(chunk.code, vec![0xff, 0x12, 0x34])
}

#[test]
fn append_4_bytes() {
    let mut chunk = Chunk::new();
    chunk.code = vec![0xff];
    chunk.append_bytes(0x12345678, 4);
    assert_eq!(chunk.code, vec![0xff, 0x12, 0x34, 0x56, 0x78])
}

