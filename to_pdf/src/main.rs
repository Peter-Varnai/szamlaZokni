use std::error::Error;
use headless_chrome::{Browser, LaunchOptions, types::PrintToPdfOptions};
use headless_chrome::protocol::cdp::Page;
use std::{
    path::Path, 
    fs,
};


fn browse_wikipedia(output_path: &str) -> Result<(), Box<dyn Error>> {
   let browser = Browser::new(LaunchOptions{
        headless: false,
        ..Default::default()
   })?;

   let pdf_options = PrintToPdfOptions {
       page_ranges: None,
       ignore_invalid_page_ranges: None,
       header_template: None,
       footer_template: None,
       transfer_mode: None,
       generate_document_outline: Some(false),
       landscape: Some(false),
       display_header_footer: Some(false),
       print_background: Some(true),
       scale: Some(1.0),
       paper_width: Some(210.0 / 25.4),  // 210mm in inches
       paper_height: Some(297.0 / 25.4), // 297mm in inches
       margin_top: Some(0.0),
       margin_bottom: Some(0.0),
       margin_left: Some(0.0),
       margin_right: Some(0.0),
       prefer_css_page_size: Some(true), // This respects CSS page sizes
       generate_tagged_pdf: Some(false),
   };


   let bmkos_html_path = &format!("file:///home/nixos/projects/szamlazokni/bills_to_db/bmkos_report.html");
   let ma7_html_path = &format!("file:///home/nixos/projects/szamlazokni/bills_to_db/ma7_report.html");
   let bezirk_html_path = &format!("file:///home/nixos/projects/szamlazokni/bills_to_db/bezirk_report.html");
   let ear_html_path = &format!("file:///home/nixos/projects/szamlazokni/bills_to_db/ear24.html");
   

   let tab = browser.new_tab()?;

   tab.navigate_to(bezirk_html_path)?;
   tab.wait_until_navigated()?;

   let pdf_data = tab.print_to_pdf(Some(pdf_options))?;
   fs::write(output_path, pdf_data)?;
   // println!("bmkos report converted");


   // let tab = browser.new_tab()?;
   //
   // tab.navigate_to(ma7_html_path)?;
   // tab.wait_until_navigated()?;
   //
   // let pdf_data = tab.print_to_pdf(Some(pdf_options))?;
   // fs::write(output_path, pdf_data)?;
   // println!("bmkos report converted");
   //
   //
   // let tab = browser.new_tab()?;
   //
   // tab.navigate_to(ma7_html_path)?;
   // tab.wait_until_navigated()?;
   //
   // let pdf_data = tab.print_to_pdf(Some(pdf_options))?;
   // fs::write(output_path, pdf_data)?;
   // println!("bmkos report converted");


   Ok(())
}


fn main() {
    browse_wikipedia("bezirk_report.pdf").expect("Failed to browse wikipedia...");
    println!("Hello, world!")
}
