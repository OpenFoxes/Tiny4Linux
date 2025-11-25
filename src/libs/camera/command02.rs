// SPDX-License-Identifier: EUPL-1.2

pub struct Command02 {
    pub function_group: Option<[u8; 6]>,
    pub sequence_nr: Option<[u8; 2]>,
    pub checksum: Option<[u8; 2]>,
    pub command: Option<[u8; 6]>,
    pub appendix: Option<[u8; 16]>,
}

impl Command02 {
    pub fn new() -> Self {
        Self {
            function_group: None,
            sequence_nr: None,
            checksum: None,
            command: None,
            appendix: None,
        }
    }

    pub fn function_group(mut self, function_group: [u8; 6]) -> Self {
        self.function_group = Some(function_group);
        self
    }

    pub fn sequence_nr(mut self, sequence_number: [u8; 2]) -> Self {
        self.sequence_nr = Some(sequence_number);
        self
    }

    pub fn checksum(mut self, checksum: [u8; 2]) -> Self {
        self.checksum = Some(checksum);
        self
    }

    pub fn command(mut self, cmd: [u8; 6]) -> Self {
        self.command = Some(cmd);
        self
    }

    pub fn appendix(mut self, app: [u8; 16]) -> Self {
        self.appendix = Some(app);
        self
    }

    pub fn build(self) -> [u8; 36] {
        const FRAME_ID: [u8; 2] = [0xaa, 0x25];
        const SEGMENT_SIZE: [u8; 2] = [0x0c, 0x00];

        [
            FRAME_ID.as_slice(),
            self.sequence_nr
                .expect("sequence_nr is required")
                .as_slice(),
            SEGMENT_SIZE.as_slice(),
            self.checksum.expect("checksum is required").as_slice(),
            self.function_group
                .expect("function_group is required")
                .as_slice(),
            self.command.expect("command is required").as_slice(),
            self.appendix.unwrap_or([0x00; 16]).as_slice(),
        ]
        .concat()
        .try_into()
        .unwrap()
    }
}
