//! Implementation of the Generic Attribute Profile (GATT).
//!
//! GATT describes a service framework that uses the Attribute Protocol for discovery and
//! interaction

pub mod characteristic;

use crate::att::{AttUuid, Attribute, AttributeProvider, Handle, HandleRange};
use crate::uuid::{Uuid128, Uuid16};
use crate::Error;
use core::cmp;

/// A demo `AttributeProvider` that will enumerate as a *Battery Service*.
pub struct BatteryServiceAttrs {
    attributes: [Attribute<&'static [u8]>; 3],
}

impl Default for BatteryServiceAttrs {
    fn default() -> Self {
        Self {
            attributes: [
                Attribute::new(
                    Uuid16(0x2800).into(), // "Primary Service"
                    Handle::from_raw(0x0001),
                    &[0x0F, 0x18], // "Battery Service" = 0x180F
                ),
                Attribute::new(
                    Uuid16(0x2803).into(), // "Characteristic"
                    Handle::from_raw(0x0002),
                    &[
                        0x02, // 1 byte properties: READ = 0x02
                        0x03, 0x00, // 2 bytes handle = 0x0003
                        0x19, 0x2A, // 2 bytes UUID = 0x2A19 (Battery Level)
                    ],
                ),
                // Characteristic value (Battery Level)
                Attribute::new(
                    AttUuid::Uuid16(Uuid16(0x2A19)), // "Battery Level"
                    Handle::from_raw(0x0003),
                    &[48u8],
                ),
            ],
        }
    }
}

impl AttributeProvider for BatteryServiceAttrs {
    fn for_attrs_in_range(
        &mut self,
        range: HandleRange,
        mut f: impl FnMut(&Self, &Attribute<dyn AsRef<[u8]>>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let count = self.attributes.len();
        let start = usize::from(range.start().as_u16() - 1); // handles start at 1, not 0
        let end = usize::from(range.end().as_u16() - 1);

        let attrs = if start >= count {
            &[]
        } else {
            let end = cmp::min(count - 1, end);
            &self.attributes[start..=end]
        };

        for attr in attrs {
            f(self, attr)?;
        }
        Ok(())
    }

    fn is_grouping_attr(&self, uuid: AttUuid) -> bool {
        uuid == Uuid16(0x2800) // FIXME not characteristics?
    }

    fn group_end(&self, handle: Handle) -> Option<&Attribute<dyn AsRef<[u8]>>> {
        match handle.as_u16() {
            0x0001 => Some(&self.attributes[2]),
            0x0002 => Some(&self.attributes[2]),
            _ => None,
        }
    }
}

/// A demo `AttributeProvider` that will enumerate as a *Midi Service*.
///
/// Also refer to <https://www.midi.org/specifications-old/item/bluetooth-le-midi>.
pub struct MidiServiceAttrs {
    attributes: [Attribute<&'static [u8]>; 4],
}

// MIDI Service (UUID: 03B80E5A-EDE8-4B33-A751-6CE34EC4C700)
// MIDI Data I/O Characteristic (UUID: 7772E5DB-3868-4112-A1A9-F2669D106BF3)

impl Default for MidiServiceAttrs {
    fn default() -> Self {
        Self {
            attributes: [
                Attribute::new(
                    Uuid16(0x2800).into(), // "Primary Service"
                    Handle::from_raw(0x0001),
                    &[
                        0x00, 0xC7, 0xC4, 0x4E, 0xE3, 0x6C, /* - */
                        0x51, 0xA7, /* - */
                        0x33, 0x4B, /* - */
                        0xE8, 0xED, /* - */
                        0x5A, 0x0E, 0xB8, 0x03,
                    ], // "Midi Service"
                ),
                Attribute::new(
                    Uuid16(0x2803).into(), // "Characteristic"
                    Handle::from_raw(0x0002),
                    &[
                        0x02 | 0x08 | 0x04 | 0x10, // 1 byte properties: READ = 0x02, WRITE_REQ = 0x08, WRITE_CMD = 0x04, NOTIFICATION = 0x10
                        0x03,
                        0x00, // 2 bytes handle = 0x0003
                        // the actual UUID
                        0xF3,
                        0x6B,
                        0x10,
                        0x9D,
                        0x66,
                        0xF2, /*-*/
                        0xA9,
                        0xA1, /*-*/
                        0x12,
                        0x41, /*-*/
                        0x68,
                        0x38, /*-*/
                        0xDB,
                        0xE5,
                        0x72,
                        0x77,
                    ],
                ),
                // Characteristic value (Empty Packet)
                Attribute::new(
                    AttUuid::Uuid128(Uuid128::from_bytes([
                        0xF3, 0x6B, 0x10, 0x9D, 0x66, 0xF2, /*-*/
                        0xA9, 0xA1, /*-*/
                        0x12, 0x41, /*-*/
                        0x68, 0x38, /*-*/
                        0xDB, 0xE5, 0x72, 0x77,
                    ])),
                    Handle::from_raw(0x0003),
                    &[],
                ),
                // CCCD
                Attribute::new(
                    AttUuid::Uuid16(Uuid16(0x2902)),
                    Handle::from_raw(0x0004),
                    &[0x00, 0x00],
                ),
            ],
        }
    }
}

impl AttributeProvider for MidiServiceAttrs {
    fn for_attrs_in_range(
        &mut self,
        range: HandleRange,
        mut f: impl FnMut(&Self, &Attribute<dyn AsRef<[u8]>>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let count = self.attributes.len();
        let start = usize::from(range.start().as_u16() - 1); // handles start at 1, not 0
        let end = usize::from(range.end().as_u16() - 1);

        let attrs = if start >= count {
            &[]
        } else {
            let end = cmp::min(count - 1, end);
            &self.attributes[start..=end]
        };

        for attr in attrs {
            f(self, attr)?;
        }
        Ok(())
    }

    fn is_grouping_attr(&self, uuid: AttUuid) -> bool {
        uuid == Uuid16(0x2800) // FIXME not characteristics?
    }

    fn group_end(&self, handle: Handle) -> Option<&Attribute<dyn AsRef<[u8]>>> {
        match handle.as_u16() {
            0x0001 => Some(&self.attributes[3]),
            0x0002 => Some(&self.attributes[3]),
            _ => None,
        }
    }
}
