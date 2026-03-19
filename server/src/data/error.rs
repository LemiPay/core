use diesel::result::Error as DieselError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error")]
    Diesel(#[from] DieselError),

    #[error("Connection pool error")]
    Pool(#[from] r2d2::Error),
}
