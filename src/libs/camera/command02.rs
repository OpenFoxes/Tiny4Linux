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
