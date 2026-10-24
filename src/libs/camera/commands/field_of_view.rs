// SPDX-License-Identifier: EUPL-1.2

use crate::FieldOfView;
use crate::libs::errors::T4lError;

/// Short command (unit 2, selector 6) for the field of view:
/// `04 01 <0 wide | 1 medium | 2 narrow>`.
pub struct FieldOfViewCommand;

impl FieldOfViewCommand {
    pub fn build(fov: FieldOfView) -> Result<[u8; 3], T4lError> {
        let value = match fov {
            FieldOfView::Wide => 0x00,
            FieldOfView::Medium => 0x01,
            FieldOfView::Narrow => 0x02,
            FieldOfView::Unknown => return Err(T4lError::InvalidSetting),
        };
        Ok([0x04, 0x01, value])
    }
}

#[cfg(test)]
mod tests {
    use crate::{FieldOfView, FieldOfViewCommand};
    use assertables::assert_err;
    use test_case::test_case;

    #[test_case(FieldOfView::Wide, [0x04, 0x01, 0x00]; "wide")]
    #[test_case(FieldOfView::Medium, [0x04, 0x01, 0x01]; "medium")]
    #[test_case(FieldOfView::Narrow, [0x04, 0x01, 0x02]; "narrow")]
    fn field_of_view(fov: FieldOfView, expected: [u8; 3]) {
        assert_eq!(FieldOfViewCommand::build(fov).unwrap(), expected);
    }

    #[test]
    fn unknown_field_of_view() {
        assert_err!(FieldOfViewCommand::build(FieldOfView::Unknown));
    }
}
