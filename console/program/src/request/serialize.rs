// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use super::*;

use snarkvm_utilities::DeserializeExt;

impl<N: Network> Serialize for Request<N> {
    /// Serializes the request into string or bytes.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match serializer.is_human_readable() {
            true => {
                let mut transition = serializer.serialize_struct("Request", 9)?;
                transition.serialize_field("caller", &self.caller)?;
                transition.serialize_field("network", &self.network_id)?;
                transition.serialize_field("program", &self.program_id)?;
                transition.serialize_field("function", &self.function_name)?;
                transition.serialize_field("input_ids", &self.input_ids)?;
                transition.serialize_field("inputs", &self.inputs)?;
                transition.serialize_field("signature", &self.signature)?;
                transition.serialize_field("sk_tag", &self.sk_tag)?;
                transition.serialize_field("tvk", &self.tvk)?;
                transition.serialize_field("tsk", &self.tsk)?;
                transition.serialize_field("tcm", &self.tcm)?;
                transition.end()
            }
            false => ToBytesSerializer::serialize_with_size_encoding(self, serializer),
        }
    }
}

impl<'de, N: Network> Deserialize<'de> for Request<N> {
    /// Deserializes the request from a string or bytes.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match deserializer.is_human_readable() {
            true => {
                // Parse the request from a string into a value.
                let mut request = serde_json::Value::deserialize(deserializer)?;
                // Recover the request.
                Ok(Self::from((
                    // Retrieve the caller.
                    DeserializeExt::take_from_value::<D>(&mut request, "caller")?,
                    // Retrieve the network ID.
                    DeserializeExt::take_from_value::<D>(&mut request, "network")?,
                    // Retrieve the program ID.
                    DeserializeExt::take_from_value::<D>(&mut request, "program")?,
                    // Retrieve the function name.
                    DeserializeExt::take_from_value::<D>(&mut request, "function")?,
                    // Retrieve the input IDs.
                    DeserializeExt::take_from_value::<D>(&mut request, "input_ids")?,
                    // Retrieve the inputs.
                    DeserializeExt::take_from_value::<D>(&mut request, "inputs")?,
                    // Retrieve the signature.
                    DeserializeExt::take_from_value::<D>(&mut request, "signature")?,
                    // Retrieve the `sk_tag`.
                    DeserializeExt::take_from_value::<D>(&mut request, "sk_tag")?,
                    // Retrieve the `tvk`.
                    DeserializeExt::take_from_value::<D>(&mut request, "tvk")?,
                    // Retrieve the `tsk`.
                    DeserializeExt::take_from_value::<D>(&mut request, "tsk")?,
                    // Retrieve the `tcm`.
                    DeserializeExt::take_from_value::<D>(&mut request, "tcm")?,
                )))
            }
            false => FromBytesDeserializer::<Self>::deserialize_with_size_encoding(deserializer, "request"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_json() -> Result<()> {
        let mut rng = TestRng::default();

        for expected in test_helpers::sample_requests(&mut rng).into_iter() {
            // Serialize
            let expected_string = &expected.to_string();
            let candidate_string = serde_json::to_string(&expected)?;

            // Deserialize
            assert_eq!(expected, Request::from_str(expected_string)?);
            assert_eq!(expected, serde_json::from_str(&candidate_string)?);
        }
        Ok(())
    }

    #[test]
    fn test_bincode() {
        let mut rng = TestRng::default();

        for expected in test_helpers::sample_requests(&mut rng).into_iter() {
            // Serialize
            let expected_bytes = expected.to_bytes_le().unwrap();
            let expected_bytes_with_size_encoding = bincode::serialize(&expected).unwrap();
            assert_eq!(&expected_bytes[..], &expected_bytes_with_size_encoding[8..]);

            // Deserialize
            assert_eq!(expected, Request::read_le(&expected_bytes[..]).unwrap());
            assert_eq!(expected, bincode::deserialize(&expected_bytes_with_size_encoding).unwrap());
        }
    }
}
