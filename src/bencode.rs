use bendy::decoding::{Error, FromBencode, Object, ResultExt};
use std::str::from_utf8;

#[derive(Debug, Eq, PartialEq)]
struct Response {
    id: Option<[u8; 20]>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Message {
    t: String,
    y: String,
    r: Option<Response>,
}

impl FromBencode for Response {
    const EXPECTED_RECURSION_DEPTH: usize = 2;

    fn decode_bencode_object(object: Object) -> Result<Self, Error> {
        let mut id = None;

        let mut dict = object.try_into_dictionary()?;

        while let Some(pair) = dict.next_pair()? {
            let key = from_utf8(pair.0)?.to_string();

            match key.as_str() {
                "id" => {
                    let bytes = pair.1.try_into_bytes().context("r.id")?;
                    id = Some(bytes.try_into().expect("Slice of incorrect length"));
                }
                _ => {}
            }
        }

        Ok(Response { id })
    }
}

impl FromBencode for Message {
    const EXPECTED_RECURSION_DEPTH: usize = 2;

    fn decode_bencode_object(object: Object) -> Result<Self, Error> {
        let mut t = None;
        let mut y = None;
        let mut r = None;

        let mut dict = object.try_into_dictionary()?;

        while let Some(pair) = dict.next_pair()? {
            match pair {
                (b"t", value) => {
                    t = String::decode_bencode_object(value)
                        .context("t")
                        .map(Some)?;
                }
                (b"y", value) => {
                    y = String::decode_bencode_object(value)
                        .context("y")
                        .map(Some)?;
                }
                (b"r", value) => {
                    r = Response::decode_bencode_object(value)
                        .context("r")
                        .map(Some)?;
                }

                _ => {}
            }
        }

        let t = t.ok_or_else(|| Error::missing_field("t"))?;
        let y = y.ok_or_else(|| Error::missing_field("y"))?;

        Ok(Message { y, t, r })
    }
}

// Decode a message
pub fn decode(bytes: &[u8]) -> Result<Message, Error> {
    let decoded = Message::from_bencode(&bytes)?;
    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_ping() {
        let bytes = b"d1:ri0e1:y1:re";
        let decoded = decode(bytes);
        println!("decoded {:?}", decoded)

        // assert_eq!(bytes_to_hex_string(&bytes), "12AB34CD56EF");
    }
}
