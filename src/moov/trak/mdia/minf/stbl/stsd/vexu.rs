//! Apple's stereo video boxes, as a visual sample entry carries them for a
//! stereoscopic (MV-HEVC) track — "ISO Base Media File Format and Apple HEVC
//! Stereo Video", Apple, 2023-2024.
//!
//! ```text
//! vexu                      video extended usage
//!   eyes                    stereo view
//!     stri                  which eye views the track has     (full box)
//!     hero                  the eye shown in 2D               (full box)
//!     cams                  the cameras
//!       blin                their baseline, micrometres       (full box)
//!     cmfy                  comfort
//!       dadj                horizontal disparity adjustment   (full box)
//! hfov                      horizontal field of view, thousandths of a degree
//! ```

use crate::*;

/// Video extended usage.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vexu {
    pub eyes: Option<Eyes>,
}

impl Atom for Vexu {
    const KIND: FourCC = FourCC::new(b"vexu");

    nested! {
        required: [],
        optional: [ Eyes ],
        multiple: [],
    }
}

/// Stereo view.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Eyes {
    pub stri: Stri,
    pub hero: Option<Hero>,
    pub cams: Option<Cams>,
    pub cmfy: Option<Cmfy>,
}

impl Atom for Eyes {
    const KIND: FourCC = FourCC::new(b"eyes");

    nested! {
        required: [ Stri ],
        optional: [ Hero, Cams, Cmfy ],
        multiple: [],
    }
}

/// Stereo view information: which eye views the track holds.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stri {
    pub has_left_eye_view: bool,
    pub has_right_eye_view: bool,
    pub has_additional_views: bool,
    pub eye_views_reversed: bool,
}

impl AtomExt for Stri {
    type Ext = ();

    const KIND_EXT: FourCC = FourCC::new(b"stri");

    fn decode_body_ext<B: Buf>(buf: &mut B, _ext: ()) -> Result<Self> {
        let bits = u8::decode(buf)?;
        Ok(Self {
            has_left_eye_view: bits & 0x01 != 0,
            has_right_eye_view: bits & 0x02 != 0,
            has_additional_views: bits & 0x04 != 0,
            eye_views_reversed: bits & 0x08 != 0,
        })
    }

    fn encode_body_ext<B: BufMut>(&self, buf: &mut B) -> Result<()> {
        let bits = u8::from(self.has_left_eye_view)
            | (u8::from(self.has_right_eye_view) << 1)
            | (u8::from(self.has_additional_views) << 2)
            | (u8::from(self.eye_views_reversed) << 3);
        bits.encode(buf)
    }
}

/// The hero eye: the view a 2D player shows. 0 none, 1 left, 2 right.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hero {
    pub hero_eye: u8,
}

impl AtomExt for Hero {
    type Ext = ();

    const KIND_EXT: FourCC = FourCC::new(b"hero");

    fn decode_body_ext<B: Buf>(buf: &mut B, _ext: ()) -> Result<Self> {
        Ok(Self {
            hero_eye: u8::decode(buf)?,
        })
    }

    fn encode_body_ext<B: BufMut>(&self, buf: &mut B) -> Result<()> {
        self.hero_eye.encode(buf)
    }
}

/// The stereo cameras.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cams {
    pub blin: Option<Blin>,
}

impl Atom for Cams {
    const KIND: FourCC = FourCC::new(b"cams");

    nested! {
        required: [],
        optional: [ Blin ],
        multiple: [],
    }
}

/// The distance between the cameras' centres, in micrometres.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Blin {
    pub baseline_um: u32,
}

impl AtomExt for Blin {
    type Ext = ();

    const KIND_EXT: FourCC = FourCC::new(b"blin");

    fn decode_body_ext<B: Buf>(buf: &mut B, _ext: ()) -> Result<Self> {
        Ok(Self {
            baseline_um: u32::decode(buf)?,
        })
    }

    fn encode_body_ext<B: BufMut>(&self, buf: &mut B) -> Result<()> {
        self.baseline_um.encode(buf)
    }
}

/// Comfort.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cmfy {
    pub dadj: Option<Dadj>,
}

impl Atom for Cmfy {
    const KIND: FourCC = FourCC::new(b"cmfy");

    nested! {
        required: [],
        optional: [ Dadj ],
        multiple: [],
    }
}

/// The horizontal disparity adjustment, in ten-thousandths of the image's
/// width, from -10000 to 10000.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dadj {
    pub adjustment: i32,
}

impl AtomExt for Dadj {
    type Ext = ();

    const KIND_EXT: FourCC = FourCC::new(b"dadj");

    fn decode_body_ext<B: Buf>(buf: &mut B, _ext: ()) -> Result<Self> {
        Ok(Self {
            adjustment: i32::decode(buf)?,
        })
    }

    fn encode_body_ext<B: BufMut>(&self, buf: &mut B) -> Result<()> {
        self.adjustment.encode(buf)
    }
}

/// The horizontal field of view, in thousandths of a degree.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hfov {
    pub field_of_view: u32,
}

impl Atom for Hfov {
    const KIND: FourCC = FourCC::new(b"hfov");

    fn decode_body<B: Buf>(buf: &mut B) -> Result<Self> {
        Ok(Self {
            field_of_view: u32::decode(buf)?,
        })
    }

    fn encode_body<B: BufMut>(&self, buf: &mut B) -> Result<()> {
        self.field_of_view.encode(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The `vexu` and `hfov` boxes of a spatial video written by Apple's
    /// AVFoundation (macOS 27): left and right eyes, the left the hero, a
    /// 10 m baseline, no disparity adjustment, 72.489°.
    const AVFOUNDATION_VEXU: &[u8] = &[
        0x00, 0x00, 0x00, 0x5a, b'v', b'e', b'x', b'u', //
        0x00, 0x00, 0x00, 0x52, b'e', b'y', b'e', b's', //
        0x00, 0x00, 0x00, 0x0d, b's', b't', b'r', b'i', 0x00, 0x00, 0x00, 0x00, 0x03, //
        0x00, 0x00, 0x00, 0x0d, b'h', b'e', b'r', b'o', 0x00, 0x00, 0x00, 0x00, 0x01, //
        0x00, 0x00, 0x00, 0x18, b'c', b'a', b'm', b's', //
        0x00, 0x00, 0x00, 0x10, b'b', b'l', b'i', b'n', 0x00, 0x00, 0x00, 0x00, 0x00, 0x98, 0x96,
        0x80, //
        0x00, 0x00, 0x00, 0x18, b'c', b'm', b'f', b'y', //
        0x00, 0x00, 0x00, 0x10, b'd', b'a', b'd', b'j', 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    const AVFOUNDATION_HFOV: &[u8] = &[
        0x00, 0x00, 0x00, 0x0c, b'h', b'f', b'o', b'v', 0x00, 0x01, 0x1b, 0x29,
    ];

    fn spatial() -> Vexu {
        Vexu {
            eyes: Some(Eyes {
                stri: Stri {
                    has_left_eye_view: true,
                    has_right_eye_view: true,
                    ..Default::default()
                },
                hero: Some(Hero { hero_eye: 1 }),
                cams: Some(Cams {
                    blin: Some(Blin {
                        baseline_um: 10_000_000,
                    }),
                }),
                cmfy: Some(Cmfy {
                    dadj: Some(Dadj { adjustment: 0 }),
                }),
            }),
        }
    }

    #[test]
    fn the_boxes_avfoundation_writes_decode_and_encode_to_the_byte() {
        let mut buf = AVFOUNDATION_VEXU;
        assert_eq!(Vexu::decode(&mut buf).expect("vexu"), spatial());
        let mut out = Vec::new();
        spatial().encode(&mut out).expect("encoded");
        assert_eq!(out, AVFOUNDATION_VEXU);

        let mut buf = AVFOUNDATION_HFOV;
        let hfov = Hfov::decode(&mut buf).expect("hfov");
        assert_eq!(hfov.field_of_view, 72_489);
        let mut out = Vec::new();
        hfov.encode(&mut out).expect("encoded");
        assert_eq!(out, AVFOUNDATION_HFOV);
    }

    #[test]
    fn stri_bits_are_each_their_own() {
        for (bits, stri) in [
            (
                0x01,
                Stri {
                    has_left_eye_view: true,
                    ..Default::default()
                },
            ),
            (
                0x02,
                Stri {
                    has_right_eye_view: true,
                    ..Default::default()
                },
            ),
            (
                0x04,
                Stri {
                    has_additional_views: true,
                    ..Default::default()
                },
            ),
            (
                0x08,
                Stri {
                    eye_views_reversed: true,
                    ..Default::default()
                },
            ),
        ] {
            let mut out = Vec::new();
            stri.encode(&mut out).expect("encoded");
            assert_eq!(out.last(), Some(&bits));
            assert_eq!(Stri::decode(&mut out.as_slice()).expect("decoded"), stri);
        }
    }
}

#[cfg(test)]
mod spatial_entry {
    use super::*;

    /// The whole `hvc1` sample entry of a spatial video written by Apple's
    /// AVFoundation into a QuickTime movie: `hvcC`, `lhvC` for the second
    /// eye's layer, `vexu` and `hfov`. It reads and says what it is; written
    /// back, its records and stereo boxes are the same bytes. (The entry
    /// around them is written the ISO way: QuickTime's spatial and temporal
    /// quality fields and its four-byte terminator are not kept.)
    #[test]
    fn a_spatial_hevc_entry_round_trips_to_the_byte() {
        const ENTRY: &[u8] = include_bytes!("../../../../../../test/spatial_hvc1.bin");
        let mut buf = ENTRY;
        let hvc1 = Hvc1::decode(&mut buf).expect("hvc1");
        assert!(hvc1.lhvc.is_some(), "the second layer's record");
        let eyes = hvc1
            .vexu
            .as_ref()
            .and_then(|v| v.eyes.as_ref())
            .expect("eyes");
        assert!(eyes.stri.has_left_eye_view && eyes.stri.has_right_eye_view);
        assert_eq!(eyes.hero.as_ref().map(|h| h.hero_eye), Some(1));
        assert_eq!(
            eyes.cams
                .as_ref()
                .and_then(|c| c.blin.as_ref())
                .map(|b| b.baseline_um),
            Some(10_000_000)
        );
        assert_eq!(hvc1.hfov.as_ref().map(|h| h.field_of_view), Some(72_489));
        let mut out = Vec::new();
        hvc1.encode(&mut out).expect("encoded");
        let children = |entry: &[u8]| {
            let at = entry.windows(4).position(|w| w == b"hvcC").expect("hvcC") - 4;
            let end = entry.windows(4).position(|w| w == b"hfov").expect("hfov") + 8;
            entry[at..end].to_vec()
        };
        assert_eq!(
            children(&out),
            children(ENTRY),
            "the records and stereo boxes, to the byte"
        );
        assert_eq!(Hvc1::decode(&mut out.as_slice()).expect("again"), hvc1);
    }
}
