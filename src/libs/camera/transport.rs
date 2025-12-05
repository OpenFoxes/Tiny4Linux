// SPDX-License-Identifier: EUPL-1.2

use crate::CameraStatus;
use crate::libs::errors::T4lError;
use crate::libs::usbio::{
    CameraHandle, UVC_GET_CUR, UVC_GET_LEN, UVC_SET_CUR, UvcUsbIo, open_camera,
};
use errno::Errno;

/// This is a wrapper around the USB camera transport.
/// It is used to send commands to a camera using a camera handle.
pub struct CameraTransport {
    handle: CameraHandle,
}

impl CameraTransport {
    /// Creates a new instance of the CameraTransport struct.
    ///
    /// This function initializes a new instance by attempting to open a camera
    /// using the provided `hint`. If the camera cannot be opened, it returns an
    /// error of type `T4lError`.
    ///
    /// # Parameters
    /// - `hint`: A string slice that serves as a hint to identify the camera to open.
    ///
    /// # Returns
    /// - `Ok(Self)`: A new instance of the struct if the camera is successfully opened.
    /// - `Err(T4lError)`: An error returned if the camera could not be opened.
    ///
    /// # Examples
    /// ```rust,ignore
    /// let instance = CameraTransport::new("OBSBOT Tiny 2");
    /// match instance {
    ///     Ok(camera) => println!("Camera opened successfully."),
    ///     Err(e) => println!("Failed to open camera: {:?}", e),
    /// }
    /// ```
    pub fn new(hint: &str) -> Result<Self, T4lError> {
        #[cfg(debug_assertions)]
        if std::env::var("T4L_MOCK").ok().as_deref() == Some("1") {
            // In mock mode during tests, use a harmless file as handle.
            let file = crate::mocks::open_mock_file().map_err(|_| T4lError::NoCameraFound)?;
            return Ok(Self { handle: file.into() });
        }
        Ok(Self {
            handle: open_camera(hint)?,
        })
    }

    /// Retrieves and returns information about the current object or instance.
    ///
    /// This method invokes the `info` method on the associated handle to gather the required details.
    /// The specific information returned depends on the implementation of the underlying handle's `info` method.
    ///
    /// # Returns
    /// * `Ok(())` - If the operation completes successfully.
    /// * `Err(Errno)` - If an error occurs during the process. The `Errno` indicates the specific error encountered.
    ///
    /// # Errors
    /// This function may return an error if the underlying `info` call fails.
    /// The error codes and their meanings are dependent on the implementation of the handle being used.
    ///
    /// # Example
    /// ```rust,ignore
    /// let result = object.info();
    /// if let Err(e) = result {
    ///     eprintln!("Failed to retrieve information: {:?}", e);
    /// } else {
    ///     println!("Information retrieved successfully.");
    /// }
    /// ```
    pub fn info(&self) -> Result<(), Errno> {
        self.handle.info()
    }

    /// Sends a hexadecimal command to the specified unit and selector using the provided command data.
    ///
    /// # Parameters
    /// - `unit`: The unit identifier to which the command is being sent.
    /// - `selector`: The selector value specifying the target operation.
    /// - `cmd`: A slice of bytes containing the command data to be sent.
    /// - `debugging`: A boolean flag indicating whether debugging information should be printed to the console.
    ///
    /// # Returns
    /// - `Ok(())`: If the command is successfully sent.
    /// - `Err(T4lError)`: If an error occurs while sending the command, wrapped in a `T4lError::USBIOError`.
    ///
    /// # Errors
    /// This method returns `T4lError::USBIOError` if the `set_cur` operation fails.
    ///
    /// # Example
    /// ```rust,ignore
    /// let cmd_data = [0x01, 0x02, 0x03];
    /// match device.send_cmd(0x2, 0x6, &cmd_data) {
    ///     Ok(_) => println!("Command sent successfully"),
    ///     Err(e) => eprintln!("Failed to send command: {:?}", e),
    /// }
    /// ```
    pub fn send_cmd(
        &self,
        unit: u8,
        selector: u8,
        cmd: &[u8],
        debugging: bool,
    ) -> Result<(), T4lError> {
        let mut data = [0u8; 60];
        data[..cmd.len()].copy_from_slice(cmd);

        self.set_cur(unit, selector, &mut data, debugging)
            .map_err(|e| T4lError::USBIOError(e.0))
    }

    /// Retrieves the current status of the camera.
    ///
    /// This method fetches the camera's current status by communicating with
    /// the device, decoding the received data, and returning a `CameraStatus`
    /// object if successful.
    ///
    /// # Parameters
    /// - `debugging`: A boolean flag indicating whether debugging information should be printed to the console.
    ///
    /// # Returns
    /// * `Ok(CameraStatus)` - A successfully decoded `CameraStatus` object representing
    ///   the current state of the camera.
    /// * `Err(T4lError)` - An error encountered during the process, wrapped in `T4lError`.
    ///
    /// # Errors
    /// This function can return the following errors:
    /// * `T4lError::USBIOError` - If there is an issue during the USB communication.
    ///
    /// # Debugging
    /// If the debugging mode is enabled via the `debugging` field, this function
    /// will print the raw data and its hexadecimal representation to the console
    /// for debugging purposes.
    ///
    /// # Example
    /// ```rust,ignore
    /// match camera.get_status() {
    ///     Ok(status) => println!("Camera status: {:?}", status),
    ///     Err(e) => eprintln!("Failed to get camera status: {:?}", e),
    /// }
    /// ```
    pub fn get_status(&self, debugging: bool) -> Result<CameraStatus, T4lError> {
        let mut data: [u8; 60] = [0u8; 60];
        self.get_cur(0x2, 0x6, &mut data)
            .map_err(|x| T4lError::USBIOError(x.0))?;

        if debugging {
            println!("Current state: {:?} {:}", data, hex::encode(&data));
        }

        Ok(CameraStatus::decode(&data))
    }

    /// Dumps the current state of the 0x2, 0x6 data to the console in hexadecimal format.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the data is successfully retrieved and dumped to the console.
    /// - `Err(Errno)` if an error occurs during the data retrieval process.
    ///
    /// # Errors
    ///
    /// This method can return an error of type `Errno` if the `get_cur` method fails to retrieve
    /// the data.
    pub fn dump(&self) -> Result<(), Errno> {
        let mut data: [u8; 60] = [0u8; 60];
        self.get_cur(0x2, 0x6, &mut data)?;
        hexdump::hexdump(&data);
        Ok(())
    }

    /// Dumps the current state of the 0x2, 0x2 data to the console in hexadecimal format.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the data is successfully retrieved and dumped to the console.
    /// - `Err(Errno)` if an error occurs during the data retrieval process.
    ///
    /// # Errors
    ///
    /// This method can return an error of type `Errno` if the `get_cur` method fails to retrieve
    /// the data.
    pub fn dump_02(&self) -> Result<(), Errno> {
        let mut data: [u8; 60] = [0u8; 60];
        self.get_cur(0x2, 0x2, &mut data)?;
        hexdump::hexdump(&data);
        Ok(())
    }

    fn get_cur(&self, unit: u8, selector: u8, data: &mut [u8]) -> Result<(), Errno> {
        // always call get_len first
        match self.get_len(unit, selector) {
            Ok(size) => {
                if data.len() < size {
                    println!("Got size {}", size);
                    return Err(Errno(1));
                }
            }
            Err(err) => return Err(err),
        };

        match self.io(unit, selector, UVC_GET_CUR, data) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    fn set_cur(
        &self,
        unit: u8,
        selector: u8,
        data: &mut [u8],
        debugging: bool,
    ) -> Result<(), Errno> {
        match self.get_len(unit, selector) {
            Ok(size) => {
                if data.len() > size {
                    println!("Got size {}", size);
                    return Err(Errno(1));
                }
            }
            Err(err) => return Err(err),
        };

        if debugging {
            println!("{:} {:} {:}", unit, selector, hex::encode(&data));
        }

        match self.io(unit, selector, UVC_SET_CUR, data) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    fn get_len(&self, unit: u8, selector: u8) -> Result<usize, Errno> {
        let mut data = [0u8; 2];

        match self.io(unit, selector, UVC_GET_LEN, &mut data) {
            Ok(_) => Ok(u16::from_le_bytes(data).into()),
            Err(err) => Err(err),
        }
    }

    fn io(&self, unit: u8, selector: u8, query: u8, data: &mut [u8]) -> Result<(), Errno> {
        #[cfg(debug_assertions)]
        if std::env::var("T4L_MOCK").ok().as_deref() == Some("1") {
            return crate::mocks::transport_io_mock(unit, selector, query, data);
        }
        self.handle.io(unit, selector, query, data)
    }
}
