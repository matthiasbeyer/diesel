pub struct SqliteBlob<'conn> {
}

impl Drop for SqliteBlob<'_> {
    fn drop(&mut self) {
        todo!()
    }
}

impl SqliteBlob<'_> {
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }
}
