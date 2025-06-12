use actix_web::{App, HttpServer};
mod models;
mod routes;
mod helpers;

use crate::routes::{
    index, 
    get_bill, 
    update_bill_number,
    update_application_number,
    update_expense_type_number,
    make_reports, 
    post_bill,
    make_ear,
    update_expense_cash,
};
use tera::Tera;
use actix_files::Files;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Template parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        App::new()
            .service(Files::new(
                    "/f", 
                    "./public"
                    )
                // .show_files_listing()
                )
            .app_data(actix_web::web::Data::new(tera.clone()))
            .service(index)
            .service(get_bill)
            .service(post_bill)
            .service(update_bill_number)
            .service(update_application_number)
            .service(update_expense_type_number)
            .service(make_reports)
            .service(make_ear)
            .service(update_expense_cash)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
