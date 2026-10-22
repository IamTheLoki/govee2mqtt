//! Support for Govee WiFi meat thermometers with probes (H5192).
//!
//! These devices do not push readings; they only answer read requests that
//! are sent as `ptReal` commands over the AWS IoT channel.
//!
//! Packets are 20 bytes long; the last byte is the XOR of the first 19.
//!  * byte 0: 0xAA = read/reply, 0x33 = write
//!  * byte 1: register
//!  * byte 2: probe number (1 or 2)
//!
//! Temperatures are signed 16-bit big-endian integers in hundredths of a
//! degree Celsius.  0xFFFF means "not present" (unplugged probe).
//!
//! Protocol notes by <https://github.com/lasswellt/govee-homeassistant/issues/174>

use std::collections::BTreeMap;

pub const PROBE_SKUS: &[&str] = &["H5192"];
pub const PROBE_NUMBERS: &[u8] = &[1, 2];

const READ: u8 = 0xaa;
const REG_CURRENT: u8 = 0x08;

pub fn is_probe_sku(sku: &str) -> bool {
    PROBE_SKUS.contains(&sku)
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ProbeReading {
    /// Core (food) temperature in degrees Celsius
    pub core: Option<f64>,
    /// Ambient temperature in degrees Celsius
    pub ambient: Option<f64>,
}

pub type ProbeReadings = BTreeMap<u8, ProbeReading>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProbeFrame {
    Current { probe: u8, reading: ProbeReading },
}

fn xor(data: &[u8]) -> u8 {
    data.iter().fold(0, |acc, b| acc ^ b)
}

/// Builds the 19 bytes (without checksum) of a read request for the
/// current probe values.
pub fn read_current_request(probe: u8) -> Vec<u8> {
    vec![READ, REG_CURRENT, probe]
}

fn decode_temperature(hi: u8, lo: u8) -> Option<f64> {
    let raw = u16::from_be_bytes([hi, lo]);
    if raw == 0xffff {
        return None;
    }
    Some(f64::from(raw as i16) / 100.)
}

/// Decodes a single 20 byte packet.
/// Only the register in byte 1 identifies the frame; byte 0 varies.
pub fn decode_frame(data: &[u8]) -> Option<ProbeFrame> {
    if data.len() != 20 || xor(&data[..19]) != data[19] {
        return None;
    }

    match data[1] {
        REG_CURRENT => {
            let probe = data[2];
            if !PROBE_NUMBERS.contains(&probe) {
                return None;
            }
            Some(ProbeFrame::Current {
                probe,
                reading: ProbeReading {
                    core: decode_temperature(data[3], data[4]),
                    ambient: decode_temperature(data[5], data[6]),
                },
            })
        }
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn with_checksum(mut data: Vec<u8>) -> Vec<u8> {
        data.resize(19, 0);
        data.push(xor(&data));
        data
    }

    #[test]
    fn decode_current_values() {
        // 0D48 0A28 -> 34.00 / 26.00
        let packet = with_checksum(vec![0xaa, 0x08, 0x01, 0x0d, 0x48, 0x0a, 0x28]);
        assert_eq!(
            decode_frame(&packet),
            Some(ProbeFrame::Current {
                probe: 1,
                reading: ProbeReading {
                    core: Some(34.0),
                    ambient: Some(26.0)
                }
            })
        );
    }

    #[test]
    fn first_byte_is_ignored() {
        for first in [0x33, 0x42, 0x44, 0x45, 0x47, 0xaa] {
            let packet = with_checksum(vec![first, 0x08, 0x02, 0x0d, 0x48, 0x0a, 0x28]);
            assert!(decode_frame(&packet).is_some(), "first byte {first:#x}");
        }
    }

    #[test]
    fn unplugged_probe_and_negative_values() {
        let packet = with_checksum(vec![0xaa, 0x08, 0x02, 0xff, 0xff, 0xfc, 0x18]);
        assert_eq!(
            decode_frame(&packet),
            Some(ProbeFrame::Current {
                probe: 2,
                reading: ProbeReading {
                    core: None,
                    ambient: Some(-10.0)
                }
            })
        );
    }

    #[test]
    fn rejects_bad_frames() {
        let mut packet = with_checksum(vec![0xaa, 0x08, 0x01, 0x0d, 0x48, 0x0a, 0x28]);
        assert!(decode_frame(&packet[..19]).is_none());
        packet[19] ^= 1;
        assert!(decode_frame(&packet).is_none());
        let packet = with_checksum(vec![0xaa, 0x08, 0x07, 0x0d, 0x48, 0x0a, 0x28]);
        assert!(decode_frame(&packet).is_none());
        let packet = with_checksum(vec![0xaa, 0x77, 0x01]);
        assert!(decode_frame(&packet).is_none());
    }
}
