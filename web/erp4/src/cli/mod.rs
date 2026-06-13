use clap::{Parser, Subcommand};

pub mod customer;
pub mod export;
pub mod fmt;
pub mod import;
pub mod init;
pub mod invoice;
pub mod order;
pub mod product;
pub mod purchase_order;
pub mod report;
pub mod session;
pub mod supplier;
pub mod user;

pub use customer::CustomerCommand;
pub use export::ExportCommand;
pub use invoice::InvoiceCommand;
pub use order::OrderCommand;
pub use product::ProductCommand;
pub use purchase_order::PurchaseOrderCommand;
pub use report::ReportCommand;
pub use session::SessionCommand;
pub use supplier::SupplierCommand;
pub use user::UserCommand;

#[derive(Parser)]
#[command(
    name = "erp4",
    version,
    about = "ERP4 - Enterprise Resource Planning CLI"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Customer(CustomerCommand),
    Product(ProductCommand),
    Supplier(SupplierCommand),
    Order(OrderCommand),
    #[command(name = "purchase-order")]
    PurchaseOrder(PurchaseOrderCommand),
    Invoice(InvoiceCommand),
    Report(ReportCommand),
    User(UserCommand),
    Session(SessionCommand),
    Export(ExportCommand),
    Web {
        #[arg(long, default_value_t = 8080)]
        port: u16,
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        #[arg(long)]
        dev: bool,
    },
}
