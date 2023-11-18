use std::io::ErrorKind;

use crate::failures::{CustomErrorMessage, CustomReisIOFailure};

#[derive(Debug)]
pub struct ErrorHandler;

impl ErrorHandler {
    pub fn handle_io_error(error: std::io::Error) -> CustomReisIOFailure {
        match error.kind() {
            ErrorKind::InvalidData => {
                CustomReisIOFailure::CorruptedDatabaseFailure(CustomErrorMessage::new(
                    String::from(
                        "Invalid data was read from your database! It seems it may be corrupt, or an invalid value was written to it externally!"
                    ), 
                    error
                ))
            },
            ErrorKind::InvalidInput => {
                CustomReisIOFailure::InvalidInputFailure(CustomErrorMessage::new(
                    String::from(
                        "Invalid parameters were passed to the operation!"
                    ),
                    error
                ))
            },
            ErrorKind::NotFound => {
                CustomReisIOFailure::DatabaseNotFoundFailure(CustomErrorMessage::new(
                    String::from(
                        "Database has not been created or database could not have been found!",
                    ),
                    error,
                ))
            }
            ErrorKind::PermissionDenied => {
                CustomReisIOFailure::PermissionDeniedForDatabase(CustomErrorMessage::new(
                    String::from(
                        "Reisbase doesn't have permission to get the specified acess into the database!",
                    ),
                    error,
                ))
            }
            ErrorKind::Unsupported => {
                CustomReisIOFailure::InvalidPlatformOperationFailure(CustomErrorMessage::new(
                    String::from(
                        "An operation ocurred which is invalid in this platform!"
                    ),
                    error
                ))
            }
            // ErrorKind::ReadOnlyFilesystem => {
            //     CustomReisFailure::PermissionDeniedForDatabase(CustomErrorMessage::new(
            //         String::from(
            //             "Your file system is read only, so we can't create our database on it!",
            //         ),
            //         error,
            //     ))
            // }
            // ErrorKind::FileTooLarge => {
            //     CustomReisFailure::DatabaseTooLargeError(CustomErrorMessage::new(
            //         String::from(
            //             "The database has a file size larger than what is supported!"
            //     ),
            //         error
            //     ))
            // }
            // ErrorKind::InvalidFilename => { 
            //     CustomReisFailure::InvalidDatabaseNameFailure(CustomErrorMessage::new(
            //         String::from(
            //             "The database filename exceeded the filename length limit."
            //         ),
            //         error
            //     ))
            // },
            _ => CustomReisIOFailure::DefaultReisFailure(CustomErrorMessage::new(
                String::from(
                    "An unexpected error ocurred! Please report this error to Rafinha!"
                ),
                error
            )),
        }
    }
}
