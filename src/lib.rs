#![feature(std_misc)]

extern crate "libxdo-sys" as sys;
extern crate libc;

use std::ffi::{CString, NulError};
use std::error::FromError;
use libc::c_int;
use std::ptr::null;
use std::time::Duration;

pub struct XDo {
    handle: *mut sys::xdo
}

#[derive(Debug)]
pub enum XDoCreationError {
    NulError(NulError),
    Unknown
}

impl FromError<NulError> for XDoCreationError {
    fn from_error(err: NulError) -> XDoCreationError {
        XDoCreationError::NulError(err)
    }
}

#[derive(Debug)]
enum XDoOperationErrorKind {
    IntParamOutOfRange{value: i64, min: i64, max: i64},
    NulError(NulError),
    OperationFailed
}

#[derive(Debug)]
struct XDoOperationError {
    kind: XDoOperationErrorKind
}

impl FromError<NulError> for XDoOperationError {
    fn from_error(err: NulError) -> XDoOperationError {
        XDoOperationError {
            kind: XDoOperationErrorKind::NulError(err)
        }
    }
}

pub type OpResult = Result<(), XDoOperationError>;

macro_rules! xdo (
    ($fncall: expr) => {
        unsafe {
            match $fncall {
                0 => Ok(()),
                _ => Err(XDoOperationError{ kind: XDoOperationErrorKind::OperationFailed })
            }
        }
    }
);

macro_rules! try_microsecs (
    ($duration: expr) => {{
        let microsecs = match $duration.num_microseconds() {
            Some(msecs) => msecs,
            None => return Err(XDoOperationError{
                kind: XDoOperationErrorKind::IntParamOutOfRange{value: -1, min: 0, max: 1000000}
            })
        };
        if microsecs < 0 || microsecs > 1000000 {
            return Err(XDoOperationError{
                kind: XDoOperationErrorKind::IntParamOutOfRange{
                    value: microsecs, min: 0, max: 1000000
                }
            });
        }
        microsecs
    }}
);

impl XDo {
    pub fn new(display: Option<&str>) -> Result<XDo, XDoCreationError> {
        let display = match display {
            Some(display) => {
                let cstr = try!(CString::new(display));
                cstr.as_ptr()
            },
            None => null()
        };
        let handle = unsafe { sys::xdo_new(display) };
        if handle.is_null() {
            return Err(XDoCreationError::Unknown);
        }
        Ok(XDo {
            handle: handle
        })
    }
    pub fn move_mouse(&self, x: i32, y: i32, screen: i32) -> OpResult {
        xdo!(sys::xdo_mousemove(self.handle, x as c_int, y as c_int, screen as c_int))
    }
    pub fn click(&self, button: i32) -> OpResult {
        xdo!(sys::xdo_click(self.handle, sys::CURRENTWINDOW, button as c_int))
    }
    pub fn type_text(&self, text: &str, delay: Duration) -> OpResult {
        let microsecs = try_microsecs!(delay);
        let string = try!(CString::new(text));
        xdo!(sys::xdo_type(self.handle, sys::CURRENTWINDOW, string.as_ptr(), microsecs as u32))
    }
    pub fn key_sequence(&self, sequence: &str, delay: Duration) -> OpResult {
        let microsecs = try_microsecs!(delay);
        let string = try!(CString::new(sequence));
        xdo!(sys::xdo_keysequence(self.handle, sys::CURRENTWINDOW, string.as_ptr(),
                                  microsecs as u32))
    }
    pub fn key_sequence_up(&self, sequence: &str, delay: Duration) -> OpResult {
        let microsecs = try_microsecs!(delay);
        let string = try!(CString::new(sequence));
        xdo!(sys::xdo_keysequence_up(self.handle, sys::CURRENTWINDOW, string.as_ptr(),
                                     microsecs as u32))
    }
    pub fn key_sequence_down(&self, sequence: &str, delay: Duration) -> OpResult {
        let microsecs = try_microsecs!(delay);
        let string = try!(CString::new(sequence));
        xdo!(sys::xdo_keysequence_down(self.handle, sys::CURRENTWINDOW, string.as_ptr(),
                                       microsecs as u32))
    }
}

impl Drop for XDo {
    fn drop(&mut self) {
        unsafe { sys::xdo_free(self.handle); }
    }
}
