//! High level bindings to [libxdo](http://www.semicomplete.com/files/xdotool/docs/html/)

#![warn(missing_docs)]

extern crate libxdo_sys as sys;

use std::ffi::{CString, NulError};
use std::convert::From;
use std::error::Error;
use std::fmt;

/// The main handle type which provides access to the various operations.
pub struct XDo {
    handle: *mut sys::xdo_t,
}

/// An error that can happen when trying to create an `XDo` instance.
#[derive(Debug)]
pub enum CreationError {
    /// The provided string parameter had an interior null byte in it.
    Nul(NulError),
    /// Libxdo failed to create an instance. No further information available.
    Ffi,
}

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreationError::Nul(ref err) => {
                write!(f,
                       "Failed to create XDo instance: Nul byte in argument: {}",
                       err)
            }
            CreationError::Ffi => write!(f, "Libxdo failed to create an instance."),
        }
    }
}

impl Error for CreationError {
    fn description(&self) -> &str {
        match *self {
            CreationError::Nul(_) => "libxdo creation error: Nul byte in argument",
            CreationError::Ffi => "libxdo creation error: Ffi error",
        }
    }
    fn cause(&self) -> Option<&Error> {
        match *self {
            CreationError::Nul(ref err) => Some(err),
            CreationError::Ffi => None,
        }
    }
}

impl From<NulError> for CreationError {
    fn from(err: NulError) -> CreationError {
        CreationError::Nul(err)
    }
}

/// An error that can happen while executing an operation.
#[derive(Debug)]
pub enum OpError {
    /// The provided string parameter had an interior null byte in it.
    Nul(NulError),
    /// Libxdo failed, returning an error code.
    Ffi(i32),
}

impl fmt::Display for OpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OpError::Nul(ref err) => {
                write!(f, "Xdo operation failed: Nul byte in argument: {}", err)
            }
            OpError::Ffi(code) => write!(f, "Xdo operation failed. Error code {}.", code),
        }
    }
}

impl Error for OpError {
    fn description(&self) -> &str {
        match *self {
            OpError::Nul(_) => "xdo operation failure: Nul byte in argument",
            OpError::Ffi(_) => "xdo operation failure: Ffi error",
        }
    }
    fn cause(&self) -> Option<&Error> {
        match *self {
            OpError::Nul(ref err) => Some(err),
            OpError::Ffi(_) => None,
        }
    }
}

impl From<NulError> for OpError {
    fn from(err: NulError) -> Self {
        OpError::Nul(err)
    }
}

/// Result of an `XDo` operation.
pub type OpResult = Result<(), OpError>;

macro_rules! xdo (
    ($fncall: expr) => {
        unsafe {
            match $fncall {
                0 => Ok(()),
                code => Err(OpError::Ffi(code))
            }
        }
    }
);

impl XDo {
    /// Creates a new `XDo` instance.
    ///
    /// # Parameters
    ///
    /// display - An optional string display name, such as `":0"`. If `None`, uses `$DISPLAY`.
    ///
    /// # Returns
    ///
    /// Returns a new `XDo` instance, or a `CreationError` on error.
    pub fn new(display: Option<&str>) -> Result<XDo, CreationError> {
        let c_string;
        let display = match display {
            Some(display) => {
                c_string = CString::new(display)?;
                c_string.as_ptr()
            }
            None => ::std::ptr::null(),
        };
        let handle = unsafe { sys::xdo_new(display) };
        if handle.is_null() {
            return Err(CreationError::Ffi);
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
        let string = CString::new(text)?;
        xdo!(sys::xdo_enter_text_window(self.handle,
                                        sys::CURRENTWINDOW,
                                        string.as_ptr(),
                                        delay_microsecs))
    }
    /// Does the specified key sequence.
    pub fn send_keysequence(&self, sequence: &str, delay_microsecs: u32) -> OpResult {
        let string = CString::new(sequence)?;
        xdo!(sys::xdo_send_keysequence_window(self.handle,
                                              sys::CURRENTWINDOW,
                                              string.as_ptr(),
                                              delay_microsecs))
    }
    /// Releases the specified key sequence.
    pub fn send_keysequence_up(&self, sequence: &str, delay_microsecs: u32) -> OpResult {
        let string = CString::new(sequence)?;
        xdo!(sys::xdo_send_keysequence_window_up(self.handle,
                                                 sys::CURRENTWINDOW,
                                                 string.as_ptr(),
                                                 delay_microsecs))
    }
    /// Presses the specified key sequence down.
    pub fn send_keysequence_down(&self, sequence: &str, delay_microsecs: u32) -> OpResult {
        let string = CString::new(sequence)?;
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
