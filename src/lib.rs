extern crate "libxdo-sys" as sys;
extern crate libc;

use std::ffi::{CString, NulError};
use std::error::FromError;
use libc::c_int;

use std::ptr::null;

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
struct XDoOperationError;
pub type OpResult = Result<(), XDoOperationError>;

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
        unsafe {
            match sys::xdo_mousemove(self.handle, x as c_int, y as c_int, screen as c_int) {
                0 => Ok(()),
                _ => Err(XDoOperationError)
            }
        }
    }
}

impl Drop for XDo {
    fn drop(&mut self) {
        unsafe { sys::xdo_free(self.handle); }
    }
}
