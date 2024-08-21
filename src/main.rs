mod bank_account;
mod library_book;

use bank_account::run_banck_account;
use library_book::run_library_book;

fn main() {
    run_banck_account();

    run_library_book();
}
