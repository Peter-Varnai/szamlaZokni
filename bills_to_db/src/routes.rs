use actix_web::{error::{ErrorInternalServerError, InternalError}, get, post, web::{self, Query}, Error, HttpResponse, Responder, Result as ActixResult};
use crate::{models::{BillQuery, Bill, BillNumberUpdate, EarBackup},
            helpers::{bills_from_db, bill_from_db, bills_per_category, 
                get_table_names, save_ear_state, expenses_from_db}};
use rusqlite::{params, Connection};
use tera::{Tera, Context};
use std::fs;


    // id INTEGER PRIMARY KEY AUTOINCREMENT,
    // date TEXT,
    // partner TEXT,
    // amount TEXT,
    // expense_type INTEGER,
    // bill INTEGER,
    // application INTEGER,
    // Bargeldabhebung BOOLEAN



#[get("/")]
pub async fn index(
    tera: actix_web::web::Data<Tera>
    ) -> impl Responder 
{

    let bills_conn = Connection::open("./public/bills.db").expect("ERROR CONNECTING TO DB");
    let expenses_conn = Connection::open("./public/discotec_accounting24.db").expect("ERROR CONNECTING TO DB");
    
    let backup_tables = get_table_names(&expenses_conn).unwrap();
  
    let bills = bills_from_db(&bills_conn).expect("error fetching bills from database");
    let expenses = expenses_from_db(&expenses_conn).expect("error fetching expenses"); 
    let mut context = Context::new();
    context.insert("bills", &bills);
    context.insert("expenses", &expenses);
    context.insert("backups", &backup_tables);

    let rendered = tera.render("index.html", &context).expect("error rendering template");
    HttpResponse::Ok().body(rendered)
}


#[get("/bill")]
pub async fn get_bill(
    no: web::Query<BillQuery>,
    tera: actix_web::web::Data<Tera>
    ) -> impl Responder 
{
   
    println!("Looking up bill no.: {}", no.no);
   
    let conn = Connection::open("./public/bills.db").expect("ERROR CONNECTING TO DB");
    let bill = bill_from_db(&conn, no.no).expect("error fetching bill from database");

    let mut context = Context::new();
    context.insert("action", "Edit");
    context.insert("bill", &bill);

    let rendered = tera.render("edit.html", &context).expect("error rendering template");

    HttpResponse::Ok().body(rendered)
}


#[post("/post_bill")]
pub async fn post_bill(
    form_data: web::Form<Bill>
) -> impl Responder 
{
    let no: u8 = form_data.id.clone();

    let conn = Connection::open("./public/bills.db").expect("ERROR CONNECTING TO DB");

    let params = params![
        &form_data.filename,
        &form_data.amount,
        &form_data.date,
        &form_data.id,
    ];
    

    conn.execute(
        "UPDATE bills24 SET 
                filename = ?1, 
                amount = ?2, 
                date = ?3
             WHERE no = ?4",
         params
    ).expect("error updating db");

    HttpResponse::Found().insert_header(("Location", format!("/bill?no={}", no + 1))).finish()
}


#[post("/setBillNumber")]
pub async fn update_bill_number(
    params: web::Json<BillNumberUpdate>
    ) -> impl Responder 
{
    println!("updating expense no: {} with bill no: {}", &params.expense_id, &params.new_number);
    let conn = Connection::open("./public/discotec_accounting24.db").expect("ERROR CONNECTING TO DB");
  
    conn.execute(
        "UPDATE ear24_main SET bill = ?1 WHERE id = ?2",
        [params.new_number, params.expense_id],
    ).expect("failed to update new bill number of ear24_main db");

    format!(
        "Updated bill number to {} for transaction ID {}",
        params.new_number, params.expense_id
    )
}


#[post("/setExpenseTypeNumber")]
pub async fn update_expense_type_number(
    params: web::Json<BillNumberUpdate>
    ) -> impl Responder 
{
    println!("updating expense no: {} with expense type no: {}", &params.expense_id, &params.new_number);
    let conn = Connection::open("./public/discotec_accounting24.db").expect("ERROR CONNECTING TO DB");
  
    conn.execute(
        "UPDATE ear24_main SET expense_type = ?1 WHERE id = ?2",
        [params.new_number, params.expense_id],
    ).expect("failed to update new expense type number of ear24_main db");

    format!(
        "Updated bill number to {} for transaction ID {}",
        params.new_number, params.expense_id
    )
}


#[post("/setExpenseCash")]
pub async fn update_expense_cash(
    params: web::Json<BillNumberUpdate>
    ) -> impl Responder 
{
    println!("updating expense no: {} with cash status: {}", &params.expense_id, &params.new_number);
    let conn = Connection::open("./public/discotec_accounting24.db").expect("ERROR CONNECTING TO DB");
 
    conn.execute(
        "UPDATE ear24_main SET Bargeldabhebung = ?1 WHERE id = ?2",
        [params.new_number, params.expense_id],
    ).expect("failed to update new application number of ear24_main db");

    format!(
        "Updated bill number to {} for transaction ID {}",
        params.new_number, params.expense_id
    )
}


#[post("/setApplicationNumber")]
pub async fn update_application_number(
    params: web::Json<BillNumberUpdate>
    ) -> impl Responder 
{
    println!("updating expense no: {} with application no: {}", &params.expense_id, &params.new_number);
    let conn = Connection::open("./public/discotec_accounting24.db").expect("ERROR CONNECTING TO DB");
  
    conn.execute(
        "UPDATE ear24_main SET application = ?1 WHERE id = ?2",
        [params.new_number, params.expense_id],
    ).expect("failed to update new application number of transactions db");

    format!(
        "Updated bill number to {} for transaction ID {}",
        params.new_number, params.expense_id
    )
}


#[get("/make_ear")]
pub async fn make_ear (
    tera:web::Data<tera::Tera>
    ) -> impl Responder 
{
    let conn = Connection::open("./public/discotec_accounting24.db").expect("ERROR CONNECTING TO DB");
    
    let expenses = expenses_from_db(&conn).expect("error");
    
    let mut context = Context::new();
    context.insert("expenses", &expenses);
    let rendered = tera.render("ear.html", &context).expect("error");
    
    fs::write("ear24.html", &rendered).expect("error writing file");
    HttpResponse::Ok().body(rendered)
}


#[get("/make_reports")]
pub async fn make_reports (
    tera: web::Data<tera::Tera>,
    backup_params: Query<EarBackup>
    ) -> impl Responder 
{
    let backup_name = backup_params.backup_table_name
        .clone()
        .unwrap_or("".to_string());
    
    let make_backup = backup_params.make_backup
        .unwrap_or(false);

    let conn = Connection::open("./public/discotec_accounting24.db").expect("ERROR CONNECTING TO DB");
    let bills_conn = Connection::open("./public/bills.db").expect("ERROR CONNECTING TO DB");

    let raw_report = bills_per_category(&conn, &bills_conn).expect("Error returning raw report data");

    for report_data in raw_report.iter() {
        let mut context = Context::new();
        context.insert("report_data", &report_data.bills);
        let rendered = tera.render("report_page.html", &context).expect("error rendering template");

        let html_file = format!("{}_report.html", report_data.name);

        fs::write(&html_file, &rendered).expect("error writing html file");
    }

    if make_backup {
        match save_ear_state(&conn, backup_name) {
            Ok(_) => println!("EAR updated succesfully!"),
            Err(e) => println!("{} EAR returned an error", e),
        }
    }

    let mut context = Context::new();
    context.insert("report_data", &raw_report[1].bills);
    let rendered = tera.render("report_page.html", &context).expect("error rendering template");

    HttpResponse::Ok().body(rendered)
}
