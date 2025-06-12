use rusqlite::{params, Connection, Result};
use crate::models::{BillToHtml, Bill, Expense, HasDate, Report};



fn sort_by_date_asc<T: HasDate>(items: &mut [T]) 
{
    items.sort_by(|a, b| {
        a.get_date().unwrap_or("0000-00-00")
         .cmp(b.get_date().unwrap_or("0000-00-00"))
    });
}


pub fn expenses_from_db(conn: &Connection) -> Result<Vec<Expense>, rusqlite::Error> 
{
    let mut stmt = conn.prepare(
        "SELECT * FROM ear24_main",
    )?;

    let mut rows = stmt.query([])?;
    let mut expenses = Vec::new();

    while let Some(row) = rows.next()? {
        expenses.push(Expense {
            id: row.get(0)?,
            date: row.get(1)?,
            partner: row.get(2)?,
            amount: {
                let raw: String = row.get(3)?;
                raw.replace(",", ".").parse::<f64>().unwrap_or(0.0)
            },
            expense_type: row.get(4)?,
            bill: row.get(5)?,
            application: row.get(6)?,
            cash: row.get(7)?,
        })
    }

    sort_by_date_asc(&mut expenses);

    Ok(expenses)
}


pub fn bill_from_db(conn: &Connection, no: u8) -> Result<Bill, rusqlite::Error> 
{
    let mut stmt = conn.prepare(
        "SELECT no, filename, amount, date, Bargeldabhebung FROM bills24 WHERE no = ?1",
    )?;

    let mut rows = stmt.query(params![no])?;

    if let Some(row) = rows.next()? {
        Ok(Bill {
            id: row.get(0)?,
            filename: row.get(1)?,
            amount: row.get(2)?,
            date: row.get(3)?,
            cash: row.get(4)?,
            // expense_type: row.get(4)?,
            // report: row.get(5)?,
        })
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}


pub fn bills_from_db(conn: &Connection) -> Result<Vec<Bill>, rusqlite::Error> 
{
    let mut stmt = conn.prepare(
        "SELECT no, filename, amount, date, Bargeldabhebung FROM bills24"
        )?;
    
    let mut rows = stmt.query([])?;
    let mut bills = Vec::new();

    while let Some(row) = rows.next()? {
        let amount: Option<f32> = row.get(2)?;
    
        bills.push(Bill {
            id: row.get(0)?,
            filename: row.get(1)?,
            amount,
            date: row.get(3)?,
            cash: row.get(4)?,
            // expense_type: None, 
            // report: None,
        })
    }

    sort_by_date_asc(&mut bills);
    Ok(bills)
}


pub fn save_ear_state(
    conn: &Connection, 
    backup_table_name: String
    ) -> Result<(), rusqlite::Error>
{
    let sql = format!(
        "CREATE TABLE {} AS SELECT id, expense_type, application FROM ear24_main", 
        backup_table_name);

    conn.execute(&sql, [])?;
    Ok(())
}


pub fn get_table_names(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table'")?;
    let tables = stmt.query_map([], |row| row.get(0))?.collect::<Result<Vec<String>>>()?;
    Ok(tables)
}


pub fn bills_per_category(
    conn: &Connection, 
    bills_conn: &Connection
    ) -> Result<Vec<Report>, rusqlite::Error> 
{
    let queries = [
        ("bmkos", "SELECT id, partner, amount, date, expense_type, bill, Bargeldabhebung FROM ear24_main WHERE application = 1"),
        ("ma7", "SELECT id, partner, amount, date, expense_type, bill, Bargeldabhebung FROM ear24_main WHERE application = 2"),
        ("bezirk", "SELECT id, partner, amount, date, expense_type, bill, Bargeldabhebung FROM ear24_main WHERE application = 3")
    ];

    let mut bills = Vec::new();

    for (category, query) in queries {
        let mut stmt = conn.prepare(query)?;
        let mut rows = stmt.query([])?;
        let mut bills_category = Vec::new();


        while let Some(row) = rows.next()? {
            bills_category.push(BillToHtml {
                expense_id: row.get(0)?,
                partner: row.get(1)?,
                amount: row.get(2)?,
                date: row.get(3)?,
                expense_type: row.get(4)?,
                filename: {
                    let raw = bill_from_db(bills_conn, row.get(5)?)?;
                    raw.filename
                },
                cash: row.get(4)?,
            });
        }

        let report = Report {
            name: category.to_string(),
            bills: bills_category,
        };

        bills.push(report);
    }

    Ok(bills)
}
