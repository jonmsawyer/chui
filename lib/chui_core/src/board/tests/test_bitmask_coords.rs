//! Test bitmask coords.

use crate::prelude::*;

pub mod bitmask_coords {
    use super::*;

    #[test]
    fn test_bitmask_coords_from() -> ChuiResult<()> {
        let coords: [(&str, Coord, Coord); 64] = [
            // a file
            (
                "a1",
                Coord::try_from(coord::A1).unwrap(),
                Coord::from(bitmask::A1),
            ),
            (
                "a2",
                Coord::try_from(coord::A2).unwrap(),
                Coord::from(bitmask::A2),
            ),
            (
                "a3",
                Coord::try_from(coord::A3).unwrap(),
                Coord::from(bitmask::A3),
            ),
            (
                "a4",
                Coord::try_from(coord::A4).unwrap(),
                Coord::from(bitmask::A4),
            ),
            (
                "a5",
                Coord::try_from(coord::A5).unwrap(),
                Coord::from(bitmask::A5),
            ),
            (
                "a6",
                Coord::try_from(coord::A6).unwrap(),
                Coord::from(bitmask::A6),
            ),
            (
                "a7",
                Coord::try_from(coord::A7).unwrap(),
                Coord::from(bitmask::A7),
            ),
            (
                "a8",
                Coord::try_from(coord::A8).unwrap(),
                Coord::from(bitmask::A8),
            ),
            // b file
            (
                "b1",
                Coord::try_from(coord::B1).unwrap(),
                Coord::from(bitmask::B1),
            ),
            (
                "b2",
                Coord::try_from(coord::B2).unwrap(),
                Coord::from(bitmask::B2),
            ),
            (
                "b3",
                Coord::try_from(coord::B3).unwrap(),
                Coord::from(bitmask::B3),
            ),
            (
                "b4",
                Coord::try_from(coord::B4).unwrap(),
                Coord::from(bitmask::B4),
            ),
            (
                "b5",
                Coord::try_from(coord::B5).unwrap(),
                Coord::from(bitmask::B5),
            ),
            (
                "b6",
                Coord::try_from(coord::B6).unwrap(),
                Coord::from(bitmask::B6),
            ),
            (
                "b7",
                Coord::try_from(coord::B7).unwrap(),
                Coord::from(bitmask::B7),
            ),
            (
                "b8",
                Coord::try_from(coord::B8).unwrap(),
                Coord::from(bitmask::B8),
            ),
            // c file
            (
                "c1",
                Coord::try_from(coord::C1).unwrap(),
                Coord::from(bitmask::C1),
            ),
            (
                "c2",
                Coord::try_from(coord::C2).unwrap(),
                Coord::from(bitmask::C2),
            ),
            (
                "c3",
                Coord::try_from(coord::C3).unwrap(),
                Coord::from(bitmask::C3),
            ),
            (
                "c4",
                Coord::try_from(coord::C4).unwrap(),
                Coord::from(bitmask::C4),
            ),
            (
                "c5",
                Coord::try_from(coord::C5).unwrap(),
                Coord::from(bitmask::C5),
            ),
            (
                "c6",
                Coord::try_from(coord::C6).unwrap(),
                Coord::from(bitmask::C6),
            ),
            (
                "c7",
                Coord::try_from(coord::C7).unwrap(),
                Coord::from(bitmask::C7),
            ),
            (
                "c8",
                Coord::try_from(coord::C8).unwrap(),
                Coord::from(bitmask::C8),
            ),
            // d file
            (
                "d1",
                Coord::try_from(coord::D1).unwrap(),
                Coord::from(bitmask::D1),
            ),
            (
                "d2",
                Coord::try_from(coord::D2).unwrap(),
                Coord::from(bitmask::D2),
            ),
            (
                "d3",
                Coord::try_from(coord::D3).unwrap(),
                Coord::from(bitmask::D3),
            ),
            (
                "d4",
                Coord::try_from(coord::D4).unwrap(),
                Coord::from(bitmask::D4),
            ),
            (
                "d5",
                Coord::try_from(coord::D5).unwrap(),
                Coord::from(bitmask::D5),
            ),
            (
                "d6",
                Coord::try_from(coord::D6).unwrap(),
                Coord::from(bitmask::D6),
            ),
            (
                "d7",
                Coord::try_from(coord::D7).unwrap(),
                Coord::from(bitmask::D7),
            ),
            (
                "d8",
                Coord::try_from(coord::D8).unwrap(),
                Coord::from(bitmask::D8),
            ),
            // e file
            (
                "e1",
                Coord::try_from(coord::E1).unwrap(),
                Coord::from(bitmask::E1),
            ),
            (
                "e2",
                Coord::try_from(coord::E2).unwrap(),
                Coord::from(bitmask::E2),
            ),
            (
                "e3",
                Coord::try_from(coord::E3).unwrap(),
                Coord::from(bitmask::E3),
            ),
            (
                "e4",
                Coord::try_from(coord::E4).unwrap(),
                Coord::from(bitmask::E4),
            ),
            (
                "e5",
                Coord::try_from(coord::E5).unwrap(),
                Coord::from(bitmask::E5),
            ),
            (
                "e6",
                Coord::try_from(coord::E6).unwrap(),
                Coord::from(bitmask::E6),
            ),
            (
                "e7",
                Coord::try_from(coord::E7).unwrap(),
                Coord::from(bitmask::E7),
            ),
            (
                "e8",
                Coord::try_from(coord::E8).unwrap(),
                Coord::from(bitmask::E8),
            ),
            // f file
            (
                "f1",
                Coord::try_from(coord::F1).unwrap(),
                Coord::from(bitmask::F1),
            ),
            (
                "f2",
                Coord::try_from(coord::F2).unwrap(),
                Coord::from(bitmask::F2),
            ),
            (
                "f3",
                Coord::try_from(coord::F3).unwrap(),
                Coord::from(bitmask::F3),
            ),
            (
                "f4",
                Coord::try_from(coord::F4).unwrap(),
                Coord::from(bitmask::F4),
            ),
            (
                "f5",
                Coord::try_from(coord::F5).unwrap(),
                Coord::from(bitmask::F5),
            ),
            (
                "f6",
                Coord::try_from(coord::F6).unwrap(),
                Coord::from(bitmask::F6),
            ),
            (
                "f7",
                Coord::try_from(coord::F7).unwrap(),
                Coord::from(bitmask::F7),
            ),
            (
                "f8",
                Coord::try_from(coord::F8).unwrap(),
                Coord::from(bitmask::F8),
            ),
            // g file
            (
                "g1",
                Coord::try_from(coord::G1).unwrap(),
                Coord::from(bitmask::G1),
            ),
            (
                "g2",
                Coord::try_from(coord::G2).unwrap(),
                Coord::from(bitmask::G2),
            ),
            (
                "g3",
                Coord::try_from(coord::G3).unwrap(),
                Coord::from(bitmask::G3),
            ),
            (
                "g4",
                Coord::try_from(coord::G4).unwrap(),
                Coord::from(bitmask::G4),
            ),
            (
                "g5",
                Coord::try_from(coord::G5).unwrap(),
                Coord::from(bitmask::G5),
            ),
            (
                "g6",
                Coord::try_from(coord::G6).unwrap(),
                Coord::from(bitmask::G6),
            ),
            (
                "g7",
                Coord::try_from(coord::G7).unwrap(),
                Coord::from(bitmask::G7),
            ),
            (
                "g8",
                Coord::try_from(coord::G8).unwrap(),
                Coord::from(bitmask::G8),
            ),
            // h file
            (
                "h1",
                Coord::try_from(coord::H1).unwrap(),
                Coord::from(bitmask::H1),
            ),
            (
                "h2",
                Coord::try_from(coord::H2).unwrap(),
                Coord::from(bitmask::H2),
            ),
            (
                "h3",
                Coord::try_from(coord::H3).unwrap(),
                Coord::from(bitmask::H3),
            ),
            (
                "h4",
                Coord::try_from(coord::H4).unwrap(),
                Coord::from(bitmask::H4),
            ),
            (
                "h5",
                Coord::try_from(coord::H5).unwrap(),
                Coord::from(bitmask::H5),
            ),
            (
                "h6",
                Coord::try_from(coord::H6).unwrap(),
                Coord::from(bitmask::H6),
            ),
            (
                "h7",
                Coord::try_from(coord::H7).unwrap(),
                Coord::from(bitmask::H7),
            ),
            (
                "h8",
                Coord::try_from(coord::H8).unwrap(),
                Coord::from(bitmask::H8),
            ),
        ];

        for (_alpha, c1, c2) in coords.into_iter() {
            // println!("({}) {} = {}", _alpha, c1, c2);
            assert_eq!(c1, c2);
        }

        Ok(())
    }

    #[test]
    fn test_bitmask_coords_into() -> ChuiResult<()> {
        let coords: [(&str, Coord, u64); 64] = [
            // a file
            ("a1", Coord::try_from(coord::A1).unwrap(), bitmask::A1),
            ("a2", Coord::try_from(coord::A2).unwrap(), bitmask::A2),
            ("a3", Coord::try_from(coord::A3).unwrap(), bitmask::A3),
            ("a4", Coord::try_from(coord::A4).unwrap(), bitmask::A4),
            ("a5", Coord::try_from(coord::A5).unwrap(), bitmask::A5),
            ("a6", Coord::try_from(coord::A6).unwrap(), bitmask::A6),
            ("a7", Coord::try_from(coord::A7).unwrap(), bitmask::A7),
            ("a8", Coord::try_from(coord::A8).unwrap(), bitmask::A8),
            // b file
            ("b1", Coord::try_from(coord::B1).unwrap(), bitmask::B1),
            ("b2", Coord::try_from(coord::B2).unwrap(), bitmask::B2),
            ("b3", Coord::try_from(coord::B3).unwrap(), bitmask::B3),
            ("b4", Coord::try_from(coord::B4).unwrap(), bitmask::B4),
            ("b5", Coord::try_from(coord::B5).unwrap(), bitmask::B5),
            ("b6", Coord::try_from(coord::B6).unwrap(), bitmask::B6),
            ("b7", Coord::try_from(coord::B7).unwrap(), bitmask::B7),
            ("b8", Coord::try_from(coord::B8).unwrap(), bitmask::B8),
            // c file
            ("c1", Coord::try_from(coord::C1).unwrap(), bitmask::C1),
            ("c2", Coord::try_from(coord::C2).unwrap(), bitmask::C2),
            ("c3", Coord::try_from(coord::C3).unwrap(), bitmask::C3),
            ("c4", Coord::try_from(coord::C4).unwrap(), bitmask::C4),
            ("c5", Coord::try_from(coord::C5).unwrap(), bitmask::C5),
            ("c6", Coord::try_from(coord::C6).unwrap(), bitmask::C6),
            ("c7", Coord::try_from(coord::C7).unwrap(), bitmask::C7),
            ("c8", Coord::try_from(coord::C8).unwrap(), bitmask::C8),
            // d file
            ("d1", Coord::try_from(coord::D1).unwrap(), bitmask::D1),
            ("d2", Coord::try_from(coord::D2).unwrap(), bitmask::D2),
            ("d3", Coord::try_from(coord::D3).unwrap(), bitmask::D3),
            ("d4", Coord::try_from(coord::D4).unwrap(), bitmask::D4),
            ("d5", Coord::try_from(coord::D5).unwrap(), bitmask::D5),
            ("d6", Coord::try_from(coord::D6).unwrap(), bitmask::D6),
            ("d7", Coord::try_from(coord::D7).unwrap(), bitmask::D7),
            ("d8", Coord::try_from(coord::D8).unwrap(), bitmask::D8),
            // e file
            ("e1", Coord::try_from(coord::E1).unwrap(), bitmask::E1),
            ("e2", Coord::try_from(coord::E2).unwrap(), bitmask::E2),
            ("e3", Coord::try_from(coord::E3).unwrap(), bitmask::E3),
            ("e4", Coord::try_from(coord::E4).unwrap(), bitmask::E4),
            ("e5", Coord::try_from(coord::E5).unwrap(), bitmask::E5),
            ("e6", Coord::try_from(coord::E6).unwrap(), bitmask::E6),
            ("e7", Coord::try_from(coord::E7).unwrap(), bitmask::E7),
            ("e8", Coord::try_from(coord::E8).unwrap(), bitmask::E8),
            // f file
            ("f1", Coord::try_from(coord::F1).unwrap(), bitmask::F1),
            ("f2", Coord::try_from(coord::F2).unwrap(), bitmask::F2),
            ("f3", Coord::try_from(coord::F3).unwrap(), bitmask::F3),
            ("f4", Coord::try_from(coord::F4).unwrap(), bitmask::F4),
            ("f5", Coord::try_from(coord::F5).unwrap(), bitmask::F5),
            ("f6", Coord::try_from(coord::F6).unwrap(), bitmask::F6),
            ("f7", Coord::try_from(coord::F7).unwrap(), bitmask::F7),
            ("f8", Coord::try_from(coord::F8).unwrap(), bitmask::F8),
            // g file
            ("g1", Coord::try_from(coord::G1).unwrap(), bitmask::G1),
            ("g2", Coord::try_from(coord::G2).unwrap(), bitmask::G2),
            ("g3", Coord::try_from(coord::G3).unwrap(), bitmask::G3),
            ("g4", Coord::try_from(coord::G4).unwrap(), bitmask::G4),
            ("g5", Coord::try_from(coord::G5).unwrap(), bitmask::G5),
            ("g6", Coord::try_from(coord::G6).unwrap(), bitmask::G6),
            ("g7", Coord::try_from(coord::G7).unwrap(), bitmask::G7),
            ("g8", Coord::try_from(coord::G8).unwrap(), bitmask::G8),
            // h file
            ("h1", Coord::try_from(coord::H1).unwrap(), bitmask::H1),
            ("h2", Coord::try_from(coord::H2).unwrap(), bitmask::H2),
            ("h3", Coord::try_from(coord::H3).unwrap(), bitmask::H3),
            ("h4", Coord::try_from(coord::H4).unwrap(), bitmask::H4),
            ("h5", Coord::try_from(coord::H5).unwrap(), bitmask::H5),
            ("h6", Coord::try_from(coord::H6).unwrap(), bitmask::H6),
            ("h7", Coord::try_from(coord::H7).unwrap(), bitmask::H7),
            ("h8", Coord::try_from(coord::H8).unwrap(), bitmask::H8),
        ];

        for (_alpha, c1, c2) in coords.into_iter() {
            // println!("({}) {} = {}", _alpha, c1, c2);
            let mask = u64::from(c1);
            assert_eq!(mask, c2);
        }

        Ok(())
    }
}
