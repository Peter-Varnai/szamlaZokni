use serde::{Deserialize, Serialize};


pub trait HasDate {
    fn get_date(&self) -> Option<&str>;
}


impl HasDate for Expense {
    fn get_date(&self) -> Option<&str> {
        self.date.as_deref()
    }
}

impl HasDate for Bill {
    fn get_date(&self) -> Option<&str> {
        self.date.as_deref()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bill {
    pub id: u8,
    pub filename: String,
    pub amount: Option<f32>,
    pub date: Option<String>,
    pub cash: Option<bool>,
    // pub expense_type: Option<u8>,
    // pub report: Option<u8>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Expense {
    pub id: u16,
    pub date: Option<String>,
    pub partner: String,
    pub amount: f64,
    pub bill: u16,
    pub expense_type: u16,
    pub application: Option<u8>,
    pub cash: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct BillQuery {
    pub no: u8,
}


#[derive(Serialize, Deserialize)]
pub struct BillToHtml {
    pub expense_id: u16,
    pub partner: String,
    pub amount: String,
    pub date: Option<String>,
    pub expense_type: i8,
    pub filename: String,
    pub cash: Option<bool>,
}


#[derive(Deserialize)]
pub struct BillNumberUpdate {
    pub expense_id: i32,
    pub new_number: i32,
}


#[derive(Deserialize, Serialize)]
pub struct Report {
    pub name: String,
    pub bills: Vec<BillToHtml>,
}

#[derive(Debug, Deserialize)]
pub struct EarBackup {
    pub backup_table_name: Option<String>,
    pub make_backup: Option<bool>
}
