use super::OndoSerializer;
use crate::db::db_error::DbResult;
use crate::db::entity::ondo_key::OndoKey;
use rmp_serde::{from_slice, to_vec};

impl OndoSerializer<OndoKey> for OndoKey {
    fn ondo_serialize(&self) -> DbResult<Vec<u8>> {
        let local = self.clone();
        let mut serialized_fields: Vec<Vec<u8>> = Vec::new();
        for field in local.values {
            let answer = to_vec(&field).expect("Failed to serialize field");
            serialized_fields.push(answer)
        }
        Ok(get_binary_key(serialized_fields))
    }

    fn ondo_deserialize(bytes: &[u8]) -> DbResult<OndoKey> {
        let serialized_fields = get_fields_from_key(bytes);
        let mut fields: Vec<serde_json::Value> = Vec::new();
        for serialized_field in serialized_fields {
            let field = from_slice::<serde_json::Value>(&serialized_field)
                .expect("Failed to deserialize field");
            fields.push(field);
        }
        let key = OndoKey { values: fields };
        Ok(key)
    }
}

// We use a vector of binary keys on a key-valute store, we need a separator for partial key searches.
// We convert the vector of keys to 7 bit and use the unused bit as a separator.
//    convert_to_7_bit: Converts a field to 7-bit representation.
//    get_binary_key: Creates a binary key from an array of byte fields.
//    convert_from_7_bit: Converts a 7-bit representation back to the original field.
//    get_fields_from_key: Extracts the original fields from a binary key.

fn get_binary_key(fields: Vec<Vec<u8>>) -> Vec<u8> {
    let delimiter = 0b1u8; // Delimiter byte (0x80)
    let mut key = Vec::new();

    for field in fields {
        let converted_field = convert_to_7_bit(&field);
        key.extend_from_slice(&converted_field);
        key.push(delimiter);
    }
    key
}

fn get_fields_from_key(key: &[u8]) -> Vec<Vec<u8>> {
    let delimiter = 0b1u8;
    let mut fields = Vec::new();
    let mut start = 0;

    for (i, byte) in key.iter().enumerate() {
        if *byte == delimiter {
            let field = convert_from_7_bit(&key[start..i]);
            fields.push(field);
            start = i + 1;
        }
    }

    fields
}

fn convert_to_7_bit(field: &[u8]) -> Vec<u8> {
    let mut result = field.to_vec();
    result.push(0);
    let mut carry = 0;
    for i in 0..(result.len()) {
        for j in i..(result.len()) {
            let byte = result[j];
            let shifted_byte = (byte << 1) | carry;
            carry = byte >> 7;
            result[j] = shifted_byte;
        }
    }
    result
}

fn convert_from_7_bit(encoded: &[u8]) -> Vec<u8> {
    if encoded.is_empty() {
        return Vec::new();
    }
    if encoded.len() == 1 {
        return encoded.to_vec();
    }
    let mut result = encoded.to_vec();
    for i in (0..result.len()).rev() {
        let mut prev_carry: u8 = 0;
        for j in (i..result.len()).rev() {
            let carry = prev_carry;
            let shifted_byte = result[j];
            let byte = (shifted_byte >> 1) | carry;
            let carry = (shifted_byte & 1) << 7;
            let _ = std::mem::replace(&mut prev_carry, carry);
            result[j] = byte;
        }
    }
    result.pop();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_7_bit_and_convert_from_7_bit_0() {
        let input: Vec<u8> = vec![0x0];
        let encoded = convert_to_7_bit(&input);
        let decoded = convert_from_7_bit(&encoded);
        assert_eq!(input, decoded);
    }

    #[test]
    fn test_convert_to_7_bit_and_convert_from_7_bit_1() {
        let input: Vec<u8> = vec![0x1];
        let encoded = convert_to_7_bit(&input);
        let decoded = convert_from_7_bit(&encoded);
        assert_eq!(input, decoded);
    }

    #[test]
    fn test_convert_to_7_bit_and_convert_from_7_bit_ff() {
        let input: Vec<u8> = vec![0xFF];
        let encoded = convert_to_7_bit(&input);
        let decoded = convert_from_7_bit(&encoded);
        assert_eq!(input, decoded);
    }
    #[test]
    fn test_convert_to_7_bit_and_convert_from_7_bit_ffff() {
        let input: Vec<u8> = vec![0xFF, 0xFF];
        let encoded = convert_to_7_bit(&input);
        let decoded = convert_from_7_bit(&encoded);
        assert_eq!(input, decoded);
    }
    #[test]
    fn test_convert_to_7_bit_and_convert_from_7_bit_ffffff() {
        let input: Vec<u8> = vec![0xFF, 0xFF, 0xFF];
        let encoded = convert_to_7_bit(&input);
        let decoded = convert_from_7_bit(&encoded);
        assert_eq!(input, decoded);
    }
    #[test]
    fn test_convert_to_7_bit_and_convert_from_7_bit() {
        let input: Vec<u8> = vec![0x41, 0x6E, 0x79, 0x20, 0x64, 0x61, 0x74, 0x61];
        let encoded = convert_to_7_bit(&input);
        let decoded = convert_from_7_bit(&encoded);
        assert_eq!(input, decoded);
    }

    #[test]
    fn test_binary_key_round_trip() {
        let input: Vec<Vec<u8>> = vec![
            vec![0x41, 0x6E, 0x79],
            vec![0x20, 0x64, 0x61],
            vec![0x74, 0x61],
        ];

        let binary_key = get_binary_key(input.clone());
        let fields = get_fields_from_key(&binary_key);

        let fields_as_slices: Vec<&[u8]> = fields.iter().map(AsRef::as_ref).collect();
        assert_eq!(input, fields_as_slices);
    }
}
