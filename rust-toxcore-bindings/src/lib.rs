include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct Tox {
    tox: *mut TOX,
}

impl Tox {
    pub fn new() -> Self {
        let tox = unsafe { tox_new(ptr::null(), ptr::null()) };
        Tox { tox }
    }

    pub fn bootstrap(&self, addr: &str, port: u16, key: &[u8]) {
        unsafe {
            tox_bootstrap(self.tox, addr.as_ptr() as *const i8, port, key.as_ptr());
        }
    }

    // Additional safe abstractions...
}

impl Drop for Tox {
    fn drop(&mut self) {
        unsafe { tox_kill(self.tox) };
    }
}
