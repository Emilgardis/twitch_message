use std::io::Write;

/// A trait to encode a type onto this [`std::io::Write`]
pub trait Encode: Write {
    fn encode_msg(&mut self, msg: impl Encodable) -> std::io::Result<()> {
        msg.encode(self)
    }
}

impl<T> Encode for T where T: Write {}

/// A trait for encoding a message with [`std::io::Write`]
pub trait Encodable {
    fn encode(&self, writer: impl Write) -> std::io::Result<()>;
}
