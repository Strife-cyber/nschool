pub trait Repository<T> {
    fn create(&self, item: &T) -> rusqlite::Result<()>;
    fn get_all(&self) -> rusqlite::Result<Vec<T>>;
    fn get(&self, id: &str) -> rusqlite::Result<Option<T>>;
    fn update(&self, item: &T) -> rusqlite::Result<()>;
    fn delete(&self, id: &str) -> rusqlite::Result<()>;

    fn filter(&self, filter: &str, params: &[&dyn rusqlite::ToSql]) -> rusqlite::Result<Vec<T>>;
}
