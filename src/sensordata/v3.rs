use sensordata::{ParseError, SensorData};

#[derive(Debug, PartialEq)]
pub struct SensorDataV3 {
    humidity: u8,
    temperature: u16,
    pressure: u16,
    acceleration: AccelerationVectorV3,
    battery_potential: u16,
}

#[derive(Debug, PartialEq)]
pub struct AccelerationVectorV3(i16, i16, i16);

impl SensorDataV3 {
    pub fn from_manufacturer_specific_data(value: &[u8]) -> Result<Self, ParseError> {
        if value.len() == 14 {
            Ok(Self {
                humidity: value[1],
                temperature: u16_from_two_bytes(value[2], value[3]),
                pressure: u16_from_two_bytes(value[4], value[5]),
                acceleration: AccelerationVectorV3(
                    i16_from_two_bytes(value[6], value[7]),
                    i16_from_two_bytes(value[8], value[9]),
                    i16_from_two_bytes(value[10], value[11]),
                ),
                battery_potential: u16_from_two_bytes(value[12], value[13]),
            })
        } else {
            Err(ParseError::InvalidValueLength {
                version: 3,
                length: value.len(),
                expected: 14,
            })
        }
    }
}

impl Into<SensorData> for SensorDataV3 {
    fn into(self) -> SensorData {
        unimplemented!()
    }
}

fn u16_from_two_bytes(b1: u8, b2: u8) -> u16 {
    ((b1 as u16) << 8) | b2 as u16
}

fn i16_from_two_bytes(b1: u8, b2: u8) -> i16 {
    u16_from_two_bytes(b1, b2) as i16
}

#[cfg(test)]
mod tests {
    use sensordata::AccelerationVector;

    use super::*;

    #[test]
    fn parse_version_3_data_with_invalid_length() {
        let value = vec![3, 103, 22, 50, 60, 70];
        let result = SensorDataV3::from_manufacturer_specific_data(&value);
        assert_eq!(
            result,
            Err(ParseError::InvalidValueLength {
                version: 3,
                length: 6,
                expected: 14
            })
        );
    }

    #[test]
    fn parse_valid_version_3_data() {
        let value = vec![
            3, 0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
        ];
        let result = SensorDataV3::from_manufacturer_specific_data(&value);
        assert_eq!(
            result,
            Ok(SensorDataV3 {
                humidity: 0x17,
                temperature: 0x0145,
                pressure: 0x3558,
                acceleration: AccelerationVectorV3(1000, 1255, 1510),
                battery_potential: 0x0886
            })
        );
    }

    #[test]
    fn parse_version_3_into_generic_structure() {
        let value = vec![
            3, 0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
        ];
        let result = SensorDataV3::from_manufacturer_specific_data(&value);
        assert!(result.is_ok());

        assert_eq!(
            result.map(|data| data.into()),
            Ok(SensorData {
                humidity: Some(115_000),
                temperature: Some(1690),
                pressure: Some(63656),
                acceleration: Some(AccelerationVector(1000, 1255, 1510)),
                battery_potential: Some(2182)
            })
        );
    }
}
