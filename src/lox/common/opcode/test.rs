
use super::OpCode;


#[test]
fn as_byte_0x00() {
    let byte: u8 = OpCode::Debug.as_byte();
    assert_eq!(byte, 0x00u8);
}

#[test]
fn as_byte_0x01() {
    let byte: u8 = OpCode::Exit.as_byte();
    assert_eq!(byte, 0x01u8);
}

#[test]
fn as_byte_0xff() {
    let byte: u8 = OpCode::BAD.as_byte();
    assert_eq!(byte, 0xffu8);
}

#[test]
fn from_u8() {
    for byte in 0x00u8 .. 0xffu8 {
        let opcode: OpCode = byte.into();
        if opcode == OpCode::BAD { continue; } // This is fine
        assert_eq!(opcode.as_byte(), byte);
    }
}

