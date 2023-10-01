use crate::{PyStatus, PyThreadState};
use std::ffi::c_int;

pub const PyInterpreterConfig_DEFAULT_GIL: c_int = PyInterpreterConfig_SHARED_GIL;
pub const PyInterpreterConfig_SHARED_GIL: c_int = 0;
pub const PyInterpreterConfig_OWN_GIL: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyInterpreterConfig {
    pub use_main_obmalloc: c_int,
    pub allow_fork: c_int,
    pub allow_exec: c_int,
    pub allow_threads: c_int,
    pub allow_daemon_threads: c_int,
    pub check_multi_interp_extensions: c_int,
    pub gil: c_int,
}

// "private" sub interpreter functions in cpython/pylifecycle.h accepted in PEP-684
extern "C" {
    pub fn Py_NewInterpreterFromConfig(
        tstate_p: *mut *mut PyThreadState,
        config: *const PyInterpreterConfig,
    ) -> PyStatus;
}
