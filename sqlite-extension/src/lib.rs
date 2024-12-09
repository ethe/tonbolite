mod table;

use core::ffi::{c_char, c_int};

use rusqlite::{ffi, to_sqlite_error, vtab, Connection};

use crate::table::TonboTable;

/// # Safety
///
/// db must be passed from SQLite sqlite3_auto_extension.
#[no_mangle]
pub unsafe extern "C" fn create_rs_extension(
    db: *mut ffi::sqlite3,
    pz_err_msg: *mut *mut c_char,
    _p_api: *mut ffi::sqlite3_api_routines,
) -> c_int {
    match Connection::from_handle(db).and_then(|conn| {
        let module = vtab::update_module::<TonboTable>();
        conn.create_module::<TonboTable>("tonbo", module, None)
    }) {
        Ok(_) => ffi::SQLITE_OK,
        Err(e) => to_sqlite_error(&e, pz_err_msg),
    }
}
