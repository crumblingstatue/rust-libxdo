//! High level bindings to [libxdo](http://www.semicomplete.com/files/xdotool/docs/html/)

#![warn(missing_docs)]

extern crate libxdo_sys as sys;

use std::ffi::{CString, NulError};
use std::convert::From;
use std::ptr::null;

/// An XDo instance
pub struct XDo {
    handle: *mut sys::xdo_t,
}

/// An error that can happen when trying to create an XDo instance.
#[derive(Debug)]
pub enum XDoCreationError {
    /// The parameter passed in contained a nul character.
    NulError(NulError),
    /// Unknown error
    Unknown,
}

impl From<NulError> for XDoCreationError {
    fn from(err: NulError) -> XDoCreationError {
        XDoCreationError::NulError(err)
    }
}

#[derive(Debug)]
enum XDoOperationErrorKind {
    NulError(NulError),
    OperationFailed,
}

/// An error originating from an XDo operation.
#[derive(Debug)]
pub struct XDoOperationError {
    kind: XDoOperationErrorKind,
}

impl From<NulError> for XDoOperationError {
    fn from(err: NulError) -> XDoOperationError {
        XDoOperationError { kind: XDoOperationErrorKind::NulError(err) }
    }
}

/// Result of an XDo operation.
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

impl XDo {
    /// Creates a new XDo instance.
    ///
    /// # Parameters
    ///
    /// display - An optional string display name, such as ":0". If None, uses the environment
    /// DISPLAY.
    ///
    /// # Returns
    ///
    /// Returns a new XDo instance, or an XDoCreationError on error.
    pub fn new(display: Option<&str>) -> Result<XDo, XDoCreationError> {
        let display = match display {
            Some(display) => {
                let cstr = try!(CString::new(display));
                cstr.as_ptr()
            }
            None => null(),
        };
        let handle = unsafe { sys::xdo_new(display) };
        if handle.is_null() {
            return Err(XDoCreationError::Unknown);
        }
        Ok(XDo { handle: handle })
    }
    /// Moves the mouse to the specified position.
    pub fn move_mouse(&self, x: i32, y: i32, screen: i32) -> OpResult {
        xdo!(sys::xdo_move_mouse(self.handle, x, y, screen))
    }
    /// Moves the mouse relative to the current position.
    pub fn move_mouse_relative(&self, x: i32, y: i32) -> OpResult {
        xdo!(sys::xdo_move_mouse_relative(self.handle, x, y))
    }
    /// Does a mouse click.
    pub fn click(&self, button: i32) -> OpResult {
        xdo!(sys::xdo_click_window(self.handle, sys::CURRENTWINDOW, button))
    }
    /// Holds a mouse button down.
    pub fn mouse_down(&self, button: i32) -> OpResult {
        xdo!(sys::xdo_mouse_down(self.handle, sys::CURRENTWINDOW, button))
    }
    /// Releases a mouse button.
    pub fn mouse_up(&self, button: i32) -> OpResult {
        xdo!(sys::xdo_mouse_up(self.handle, sys::CURRENTWINDOW, button))
    }
    /// Types the specified text.
    pub fn enter_text(&self, text: &str, delay_microsecs: u32) -> OpResult {
        let string = try!(CString::new(text));
        xdo!(sys::xdo_enter_text_window(self.handle,
                                        sys::CURRENTWINDOW,
                                        string.as_ptr(),
                                        delay_microsecs))
    }
    /// Does the specified key sequence.
    pub fn send_keysequence(&self, sequence: &str, delay_microsecs: u32) -> OpResult {
        let string = try!(CString::new(sequence));
        xdo!(sys::xdo_send_keysequence_window(self.handle,
                                              sys::CURRENTWINDOW,
                                              string.as_ptr(),
                                              delay_microsecs))
    }
    /// Releases the specified key sequence.
    pub fn send_keysequence_up(&self, sequence: &str, delay_microsecs: u32) -> OpResult {
        let string = try!(CString::new(sequence));
        xdo!(sys::xdo_send_keysequence_window_up(self.handle,
                                                 sys::CURRENTWINDOW,
                                                 string.as_ptr(),
                                                 delay_microsecs))
    }
    /// Presses the specified key sequence down.
    pub fn send_keysequence_down(&self, sequence: &str, delay_microsecs: u32) -> OpResult {
        let string = try!(CString::new(sequence));
        xdo!(sys::xdo_send_keysequence_window_down(self.handle,
                                                   sys::CURRENTWINDOW,
                                                   string.as_ptr(),
                                                   delay_microsecs))
    }
}

impl Drop for XDo {
    fn drop(&mut self) {
        unsafe {
            sys::xdo_free(self.handle);
        }
    }
}
