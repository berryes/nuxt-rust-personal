pub fn init( conn: sqlite::Connection )  {

    let query = "
    CREATE TABLE posts (
        id TEXT,
        name TEXT,
        markdown TEXT,
        created DATE,
        edited DATE);";


    conn.execute(query).unwrap();

}