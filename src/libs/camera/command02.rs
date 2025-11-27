// SPDX-License-Identifier: EUPL-1.2

use bon::builder;

#[builder(finish_fn = build)]
pub fn command02(
    function_group: [u8; 6],
    sequence_nr: [u8; 2],
    checksum: [u8; 2],
    command: [u8; 6],
    appendix: Option<[u8; 16]>,
) -> [u8; 36] {
    const FRAME_ID: [u8; 2] = [0xaa, 0x25];
    const SEGMENT_SIZE: [u8; 2] = [0x0c, 0x00];

    [
        FRAME_ID.as_slice(),
        sequence_nr.as_slice(),
        SEGMENT_SIZE.as_slice(),
        checksum.as_slice(),
        function_group.as_slice(),
        command.as_slice(),
        appendix.unwrap_or([0x00; 16]).as_slice(),
    ]
    .concat()
    .try_into()
    .unwrap()
}

#[cfg(test)]
mod tests {
    mod unit {
        use crate::libs::camera::command02::command02;

        #[test]
        fn full_builder() {
            let command = command02()
                .function_group([0x01, 0x02, 0x03, 0x04, 0x05, 0x06])
                .sequence_nr([0x10, 0x11])
                .checksum([0x20, 0x21])
                .command([0x30, 0x31, 0x32, 0x33, 0x34, 0x35])
                .appendix([
                    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c,
                    0x4d, 0x4e, 0x4f,
                ])
                .build();

            assert_eq!(
                command[0..2],
                [0xaa, 0x25],
                "Command should start with frame id"
            );
            assert_eq!(command[2..4], [0x10, 0x11], "Sequence number should be set");
            assert_eq!(command[4..6], [0x0c, 0x00], "Segment size should be fix");
            assert_eq!(command[6..8], [0x20, 0x21], "Checksum should be set");
            assert_eq!(
                command[8..14],
                [0x01, 0x02, 0x03, 0x04, 0x05, 0x06],
                "Function group should be set"
            );
            assert_eq!(
                command[14..20],
                [0x30, 0x31, 0x32, 0x33, 0x34, 0x35],
                "Command should be set"
            );
            assert_eq!(
                command[20..36],
                [
                    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c,
                    0x4d, 0x4e, 0x4f
                ],
                "Appendix should be set"
            );
        }

        #[test]
        fn minimal_builder() {
            let command = command02()
                .function_group([0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5])
                .sequence_nr([0xb0, 0xb1])
                .checksum([0xc0, 0xc1])
                .command([0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5])
                .build();

            assert_eq!(
                command[0..2],
                [0xaa, 0x25],
                "Command should start with frame id"
            );
            assert_eq!(command[2..4], [0xb0, 0xb1], "Sequence number should be set");
            assert_eq!(command[4..6], [0x0c, 0x00], "Segment size should be fix");
            assert_eq!(command[6..8], [0xc0, 0xc1], "Checksum should be set");
            assert_eq!(
                command[8..14],
                [0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5],
                "Function group should be set"
            );
            assert_eq!(
                command[14..20],
                [0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5],
                "Command should be set"
            );
            assert_eq!(
                command[20..36],
                [
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00
                ],
                "Appendix should be empty"
            );
        }
    }
}
