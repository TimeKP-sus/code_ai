use rusqlite::{Connection, Result};

pub fn mo_csdl() -> Result<(), Box<dyn std::error::Error>> {
    let _conn: Connection = Connection::open("data.db")?;
    Ok(())
}
pub fn chay_lenh_sql(lenh: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let conn: Connection = Connection::open("data.db")?;
    let ket_qua: usize = conn.execute(lenh, [])?;
    Ok(ket_qua)
}
// pub fn truy_van_sql(lenh: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
//     let conn = Connection::open("data.db")?;
//     let mut stmt = conn.prepare(lenh)?;
//     let ket_qua_iter = stmt.query_map([], |row| {
//         let mut dong = Vec::new();
//         for i in 0..row.column_count() {
//             let gia_tri: String = row.get(i)?;
//             dong.push(gia_tri);
//         }
//         Ok(dong)
//     })?;

//     let mut ket_qua = Vec::new();
//     for dong_ket_qua in ket_qua_iter {
//         ket_qua.push(dong_ket_qua?);
//     }
//     Ok(ket_qua)
// }
