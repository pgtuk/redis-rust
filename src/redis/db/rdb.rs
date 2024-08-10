use anyhow;

use super::Db;

pub(crate) struct RDB {
    path: String,
}

impl RDB {
    // TODO implement
    // read from rdb into memory
    // write rdb from memory

    fn new (path: String) -> RDB {
        RDB { path }
    }

    fn read(&self) -> Db {
        unimplemented!()
    }

    fn write(&self, db: &Db) -> anyhow::Result<()> {
        unimplemented!()
    }
}