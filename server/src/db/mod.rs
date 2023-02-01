pub fn init( conn: sqlite::Connection )  {

    let query = "
    CREATE TABLE posts (name TEXT, markdown TEXT, created DATE, edited DATE);

    ";


    conn.execute(query).unwrap();

}