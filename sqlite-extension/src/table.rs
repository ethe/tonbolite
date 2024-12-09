use core::{ffi::c_int, marker::PhantomData};

use rusqlite::{
    ffi::{sqlite3_vtab, sqlite3_vtab_cursor},
    types::ValueRef,
    vtab::{
        Context, CreateVTab, IndexInfo, UpdateVTab, VTab, VTabConnection, VTabCursor, VTabKind,
        Values,
    },
};

#[derive(Default)]
#[repr(C)]
pub struct TonboCursor<'vtab> {
    base: sqlite3_vtab_cursor,
    row_id: i64,
    _marker: PhantomData<&'vtab ()>,
}

unsafe impl VTabCursor for TonboCursor<'_> {
    fn filter(
        &mut self,
        _idx_num: c_int,
        _idx_str: Option<&str>,
        _args: &Values<'_>,
    ) -> rusqlite::Result<()> {
        self.row_id = 1;
        Ok(())
    }

    fn next(&mut self) -> rusqlite::Result<()> {
        self.row_id += 1;
        Ok(())
    }

    fn eof(&self) -> bool {
        self.row_id > 1
    }

    fn column(&self, ctx: &mut Context, _i: c_int) -> rusqlite::Result<()> {
        ctx.set_result(&self.row_id)
    }

    fn rowid(&self) -> rusqlite::Result<i64> {
        Ok(self.row_id)
    }
}

#[repr(C)]
pub struct TonboTable {
    base: sqlite3_vtab,
}

unsafe impl<'vtab> VTab<'vtab> for TonboTable {
    type Aux = ();

    type Cursor = TonboCursor<'vtab>;

    fn connect(
        _db: &mut VTabConnection,
        _aux: Option<&Self::Aux>,
        args: &[&[u8]],
    ) -> rusqlite::Result<(String, Self)> {
        let vtab = Self {
            base: sqlite3_vtab::default(),
        };
        let args = args
            .iter()
            .map(|a| String::from_utf8_lossy(a).to_string())
            .collect::<Vec<_>>();
        Ok((
            format!("CREATE TABLE {}({})", args[2], args[3..].join(",")),
            vtab,
        ))
    }

    fn best_index(&self, info: &mut IndexInfo) -> rusqlite::Result<()> {
        info.set_estimated_cost(1.);
        Ok(())
    }

    fn open(&'vtab mut self) -> rusqlite::Result<Self::Cursor> {
        Ok(TonboCursor::default())
    }
}

impl CreateVTab<'_> for TonboTable {
    const KIND: VTabKind = VTabKind::Default;
}

impl UpdateVTab<'_> for TonboTable {
    fn delete(&mut self, _arg: ValueRef<'_>) -> rusqlite::Result<()> {
        Ok(())
    }

    fn insert(&mut self, args: &Values<'_>) -> rusqlite::Result<i64> {
        Ok(args.len() as i64)
    }

    fn update(&mut self, _args: &Values<'_>) -> rusqlite::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rusqlite::{version_number, vtab, Connection};

    use super::*;

    #[test]
    fn its_ok() {
        let module = vtab::update_module::<TonboTable>();

        let db = Connection::open_in_memory().unwrap();

        db.create_module::<TonboTable>("tonbo", module, Some(()))
            .unwrap();

        let version = version_number();
        if version < 3_009_000 {
            return;
        }

        db.execute(
            "CREATE VIRTUAL TABLE test USING tonbo(foo INT PRIMARY KEY NOT NULL, bar TEXT)",
            [],
        )
        .unwrap();

        let mut s = db.prepare("SELECT * FROM test WHERE bar = 1").unwrap();

        let dummy = s.query_row([], |row| row.get::<_, i32>(0)).unwrap();
        assert_eq!(1, dummy);
    }
}
