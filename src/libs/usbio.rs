// SPDX-License-Identifier: EUPL-1.2

use crate::libs::errors::T4lError;
use enum_dispatch::enum_dispatch;
use errno::Errno;
use glob::MatchOptions;
use glob::glob_with;
use nix::{Error, ioctl_read_buf, ioctl_readwrite_buf};
use std::fs::File;
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::PathBuf;
use std::str;

#[cfg(not(test))]
pub(crate) mod deps {
    use super::*;
    pub fn open_file(path: &str) -> std::io::Result<File> { File::open(path) }
    pub fn glob_video_paths() -> Vec<PathBuf> {
        let options = MatchOptions {
            case_sensitive: true,
            require_literal_separator: true,
            require_literal_leading_dot: true,
        };
        glob_with("/dev/video*", options).unwrap().flatten().collect()
    }
    pub unsafe fn ioctl_querycap(fd: RawFd, buf: &mut [v4l2_capability]) -> Result<i32, nix::errno::Errno> {
        ioctl_videoc_querycap(fd, buf)
    }
    pub unsafe fn uvc_ctrl_query(fd: RawFd, q: &mut [uvc_xu_control_query]) -> Result<i32, nix::errno::Errno> {
        uvcioc_ctrl_query(fd, q)
    }
}

#[cfg(test)]
pub(super) mod deps {
    use super::*;
    use std::cell::RefCell;
    thread_local! {
        static OPEN_FILE_HOOK: RefCell<Option<Box<dyn Fn(&str) -> std::io::Result<File>>>> = RefCell::new(None);
        static GLOB_HOOK: RefCell<Option<Box<dyn Fn() -> Vec<PathBuf>>>> = RefCell::new(None);
        static QUERYCAP_HOOK: RefCell<Option<Box<dyn Fn(RawFd, &mut [v4l2_capability]) -> Result<i32, nix::errno::Errno>>>> = RefCell::new(None);
        static UVC_QUERY_HOOK: RefCell<Option<Box<dyn Fn(RawFd, &mut [uvc_xu_control_query]) -> Result<i32, nix::errno::Errno>>>> = RefCell::new(None);
    }
    pub fn set_open_file<F: 'static + Fn(&str) -> std::io::Result<File>>(f: F) { OPEN_FILE_HOOK.with(|h| *h.borrow_mut() = Some(Box::new(f))); }
    pub fn set_glob<F: 'static + Fn() -> Vec<PathBuf>>(f: F) { GLOB_HOOK.with(|h| *h.borrow_mut() = Some(Box::new(f))); }
    pub fn set_ioctl_querycap<F: 'static + Fn(RawFd, &mut [v4l2_capability]) -> Result<i32, nix::errno::Errno>>(f: F) { QUERYCAP_HOOK.with(|h| *h.borrow_mut() = Some(Box::new(f))); }
    pub fn set_uvc_ctrl_query<F: 'static + Fn(RawFd, &mut [uvc_xu_control_query]) -> Result<i32, nix::errno::Errno>>(f: F) { UVC_QUERY_HOOK.with(|h| *h.borrow_mut() = Some(Box::new(f))); }
    pub fn reset_hooks() {
        OPEN_FILE_HOOK.with(|h| *h.borrow_mut() = None);
        GLOB_HOOK.with(|h| *h.borrow_mut() = None);
        QUERYCAP_HOOK.with(|h| *h.borrow_mut() = None);
        UVC_QUERY_HOOK.with(|h| *h.borrow_mut() = None);
    }
    pub fn open_file(path: &str) -> std::io::Result<File> {
        OPEN_FILE_HOOK.with(|h| {
            if let Some(f) = &*h.borrow() { return f(path); }
            File::open(path)
        })
    }
    pub fn glob_video_paths() -> Vec<PathBuf> {
        GLOB_HOOK.with(|h| {
            if let Some(f) = &*h.borrow() { return f(); }
            Vec::new()
        })
    }
    pub unsafe fn ioctl_querycap(_fd: RawFd, _buf: &mut [v4l2_capability]) -> Result<i32, nix::errno::Errno> {
        QUERYCAP_HOOK.with(|h| {
            if let Some(f) = &*h.borrow() { return f(_fd, _buf); }
            Err(nix::errno::Errno::EINVAL)
        })
    }
    pub unsafe fn uvc_ctrl_query(_fd: RawFd, _q: &mut [uvc_xu_control_query]) -> Result<i32, nix::errno::Errno> {
        UVC_QUERY_HOOK.with(|h| {
            if let Some(f) = &*h.borrow() { return f(_fd, _q); }
            Err(nix::errno::Errno::EINVAL)
        })
    }
}

#[enum_dispatch(CameraHandleType)]
pub trait UvcUsbIo {
    fn info(&self) -> Result<(), Errno>;
    fn io(&self, unit: u8, selector: u8, query: u8, data: &mut [u8]) -> Result<(), Errno>;
}

#[derive(Debug)]
pub struct CameraHandle(File);

impl From<File> for CameraHandle {
    fn from(file: File) -> Self {
        CameraHandle(file)
    }
}

impl UvcUsbIo for CameraHandle {
    fn info(&self) -> Result<(), Errno> {
        match v4l2_capability::new(&self.0) {
            Ok(video_info) => {
                println!(
                    "Card: {}\nBus : {}",
                    str::from_utf8(&video_info.card).unwrap(),
                    str::from_utf8(&video_info.bus_info).unwrap()
                );
                Ok(())
            }
            _ => {
                println!("Failed");
                Err(Errno(Error::last_raw()))
            }
        }
    }

    fn io(&self, unit: u8, selector: u8, query: u8, data: &mut [u8]) -> Result<(), Errno> {
        let dev = &self.0;

        let query = uvc_xu_control_query {
            unit,
            selector,
            query,
            size: data.len() as u16,
            data: data.as_mut_ptr(),
        };

        unsafe {
            match deps::uvc_ctrl_query(dev.as_raw_fd(), &mut [query]) {
                Ok(_) => Ok(()),
                _ => Err(Errno(Error::last_raw())),
            }
        }
    }
}

pub(crate) fn open_camera(hint: &str) -> Result<CameraHandle, T4lError> {
    if let Ok(file) = deps::open_file(hint) {
        return Ok(file.into());
    }

    if let Ok(file) = deps::open_file(&format!("/dev/{hint}")) {
        return Ok(file.into());
    }

    // enumerate all cameras and check for match
    for path in deps::glob_video_paths().into_iter() {
        if let Ok(device) = deps::open_file(&path.to_string_lossy()) {
            if let Ok(video_info) = v4l2_capability::new(&device) {
                if (str::from_utf8(&video_info.card).unwrap().contains(hint)
                    || str::from_utf8(&video_info.bus_info).unwrap().contains(hint))
                    && (video_info.device_caps & 0x800000 == 0)
                {
                    return Ok(device.into());
                }
            }
        }
    }
    Err(T4lError::NoCameraFound)
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default, Debug)]
pub struct v4l2_capability {
    driver: [u8; 16],
    card: [u8; 32],
    bus_info: [u8; 32],
    version: u32,
    capabilities: u32,
    device_caps: u32,
    reserved: [u32; 3],
}

impl v4l2_capability {
    fn new(dev: &File) -> Result<Self, Errno> {
        let mut query = [v4l2_capability {
            ..Default::default()
        }];

        unsafe {
            match deps::ioctl_querycap(dev.as_raw_fd(), &mut query) {
                Ok(_) => Ok(query[0]),
                _ => Err(Errno(Error::last_raw())),
            }
        }
    }
}

const VIDIOC_QUERYCAP_MAGIC: u8 = b'V';
const VIDIOC_QUERYCAP_QUERY_MESSAGE: u8 = 0x00;
ioctl_read_buf!(
    ioctl_videoc_querycap,
    VIDIOC_QUERYCAP_MAGIC,
    VIDIOC_QUERYCAP_QUERY_MESSAGE,
    v4l2_capability
);

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct uvc_xu_control_query {
    unit: u8,
    selector: u8,
    query: u8, /* Video Class-Specific Request Code, */
    /* defined in linux/usb/video.h A.8.  */
    size: u16,
    data: *mut u8,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[allow(dead_code)]
pub struct uvc_menu_info {
    name: [u8; 32],
    value: u32,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[allow(dead_code)]
pub struct uvc_xu_control_mapping {
    id: u32,
    name: [u8; 32],
    entity: u8,
    selector: u8,
    size: u16,
    offset: u16,
    v4l2_type: u32,
    data_type: u32,
    uvc_menu_info: u32,
    uvc_menu_count: u32,
}

const UVCIOC_CTRL_MAGIC: u8 = b'u'; // Defined in linux/uvcvideo.h
const UVCIOC_CTRL_QUERY_MESSAGE: u8 = 0x21; // Defined in linux/uvcvideo.h
ioctl_readwrite_buf!(
    uvcioc_ctrl_query,
    UVCIOC_CTRL_MAGIC,
    UVCIOC_CTRL_QUERY_MESSAGE,
    uvc_xu_control_query
);
ioctl_read_buf!(
    uvcioc_ctrl_query_read,
    UVCIOC_CTRL_MAGIC,
    UVCIOC_CTRL_QUERY_MESSAGE,
    uvc_xu_control_query
);

/* A.8. Video Class-Specific Request Codes */
#[allow(dead_code)]
const UVC_RC_UNDEFINED: u8 = 0x00;
#[allow(dead_code)]
pub const UVC_SET_CUR: u8 = 0x01;
#[allow(dead_code)]
pub const UVC_GET_CUR: u8 = 0x81;
#[allow(dead_code)]
const UVC_GET_MIN: u8 = 0x82;
#[allow(dead_code)]
const UVC_GET_MAX: u8 = 0x83;
#[allow(dead_code)]
const UVC_GET_RES: u8 = 0x84;
#[allow(dead_code)]
pub const UVC_GET_LEN: u8 = 0x85;
#[allow(dead_code)]
const UVC_GET_INFO: u8 = 0x86;
#[allow(dead_code)]
const UVC_GET_DEF: u8 = 0x87;

#[cfg(test)]
mod tests {
    use super::{open_camera, CameraHandle, UvcUsbIo, UVC_GET_CUR, UVC_GET_LEN, UVC_SET_CUR, v4l2_capability};
    use super::deps;
    use errno::Errno;
    use mockall::mock;
    use mockall::predicate::{always, eq};
    use nix::errno::Errno as NixErrno;
    use std::io;

    // Mock the trait without changing production code
    mock! {
        pub UsbIoMock {}
        impl UvcUsbIo for UsbIoMock {
            fn info(&self) -> Result<(), Errno>;
            fn io(&self, unit: u8, selector: u8, query: u8, data: &mut [u8]) -> Result<(), Errno>;
        }
    }

    #[test]
    fn uvc_constants_values() {
        assert_eq!(UVC_SET_CUR, 0x01);
        assert_eq!(UVC_GET_CUR, 0x81);
        assert_eq!(UVC_GET_LEN, 0x85);
    }

    #[test]
    fn mocked_info_succeeds() {
        let mut mock = MockUsbIoMock::new();
        mock.expect_info().times(1).returning(|| Ok(()));
        assert!(mock.info().is_ok());
    }

    #[test]
    fn mocked_io_modifies_buffer() {
        let mut mock = MockUsbIoMock::new();
        mock.expect_io()
            .with(eq(2u8), eq(6u8), eq(UVC_SET_CUR), always())
            .times(1)
            .returning(|_, _, _, data: &mut [u8]| {
                if !data.is_empty() { data[0] = 0xAB; }
                Ok(())
            });

        let mut buf = [0u8; 4];
        let res = mock.io(2, 6, UVC_SET_CUR, &mut buf);
        assert!(res.is_ok());
        assert_eq!(buf[0], 0xAB);
    }

    #[test]
    fn mocked_io_returns_error() {
        let mut mock = MockUsbIoMock::new();
        mock.expect_io()
            .with(eq(1u8), eq(1u8), eq(UVC_GET_CUR), always())
            .times(1)
            .returning(|_, _, _, _| Err(Errno(5))); // EIO
        let mut buf = [0u8; 2];
        let res = mock.io(1, 1, UVC_GET_CUR, &mut buf);
        assert!(res.is_err());
    }

    // ---------- Production code tests via deps hooks (no real hardware) ----------

    #[test]
    fn open_camera_direct_path_succeeds() {
        deps::reset_hooks();
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path_str = String::from("/tmp/fake_cam0");
        let file_for_return = tmp.reopen().unwrap();
        deps::set_open_file(move |path| {
            if path == path_str { Ok(file_for_return.try_clone().unwrap()) } else { Err(io::Error::from(io::ErrorKind::NotFound)) }
        });

        let cam = open_camera(&path_str);
        assert!(cam.is_ok());
    }

    #[test]
    fn open_camera_dev_prefix_fallback() {
        deps::reset_hooks();
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let file_for_return = tmp.reopen().unwrap();
        deps::set_open_file(move |path| {
            if path == "/dev/video2" { Ok(file_for_return.try_clone().unwrap()) }
            else { Err(io::Error::from(io::ErrorKind::NotFound)) }
        });

        let cam = open_camera("video2");
        assert!(cam.is_ok());
    }

    #[test]
    fn open_camera_glob_and_capability_match() {
        deps::reset_hooks();
        // Fail direct and /dev/ opens
        deps::set_open_file(|path| {
            if path == "/dev/video9" { // will be used from glob
                let tmp = tempfile::NamedTempFile::new().unwrap();
                Ok(tmp.reopen().unwrap())
            } else {
                Err(io::Error::from(io::ErrorKind::NotFound))
            }
        });
        deps::set_glob(|| vec![std::path::PathBuf::from("/dev/video9")]);
        // Prepare ioctl to fill capability with matching strings and device_caps OK
        deps::set_ioctl_querycap(|_fd, buf| {
            let mut cap = v4l2_capability { ..Default::default() };
            let card_bytes = b"OBSBOT Tiny 2";
            let bus_bytes = b"usb-0000";
            cap.card[..card_bytes.len()].copy_from_slice(card_bytes);
            cap.bus_info[..bus_bytes.len()].copy_from_slice(bus_bytes);
            cap.device_caps = 0; // ensures (device_caps & 0x800000) == 0
            buf[0] = cap;
            Ok(0)
        });

        let cam = open_camera("OBSBOT Tiny");
        assert!(cam.is_ok());
    }

    #[test]
    fn camera_handle_info_uses_ioctl_and_succeeds() {
        deps::reset_hooks();
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let file = tmp.reopen().unwrap();
        let handle = CameraHandle::from(file);
        deps::set_ioctl_querycap(|_fd, buf| {
            let mut cap = v4l2_capability { ..Default::default() };
            let card_bytes = b"CardName";
            let bus_bytes = b"BusInfo";
            cap.card[..card_bytes.len()].copy_from_slice(card_bytes);
            cap.bus_info[..bus_bytes.len()].copy_from_slice(bus_bytes);
            buf[0] = cap;
            Ok(0)
        });
        let res = handle.info();
        assert!(res.is_ok());
    }

    #[test]
    fn camera_handle_io_success_and_error() {
        deps::reset_hooks();
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let file = tmp.reopen().unwrap();
        let handle = CameraHandle::from(file);

        // Success path: modify buffer via pointer and return Ok
        deps::set_uvc_ctrl_query(|_fd, q| {
            assert_eq!(q.len(), 1);
            let query = &q[0];
            assert_eq!(query.unit, 2);
            assert_eq!(query.selector, 6);
            assert_eq!(query.query, UVC_SET_CUR);
            unsafe {
                if !query.data.is_null() && query.size > 0 {
                    *query.data = 0xCD;
                }
            }
            Ok(0)
        });
        let mut buf = [0u8; 4];
        let ok = handle.io(2, 6, UVC_SET_CUR, &mut buf);
        assert!(ok.is_ok());
        assert_eq!(buf[0], 0xCD);

        // Error path
        deps::set_uvc_ctrl_query(|_fd, _q| Err(NixErrno::EIO));
        let mut buf2 = [0u8; 2];
        let err = handle.io(1, 1, UVC_GET_CUR, &mut buf2);
        assert!(err.is_err());
    }
}
