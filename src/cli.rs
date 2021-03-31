use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "s3-backup", about = "A simple S3 backup CLI and automation tool")]
pub enum Cli {

    /// Add a new backup
    Add {

        /// The name of the backup
        name: String,

        /// All the files and directory for this backup
        #[structopt(parse(from_os_str))]
        files: Vec<PathBuf>,

        /// The profile to use for this backup
        #[structopt(
        short = "p",
        long = "profile",
        default_value = "default",
        )]
        profile: String,

        /// The backup interval in hours
        #[structopt(short = "i",
        long = "interval",
        default_value = "24",
        )]
        backup_interval: i8,
    },

    /// Remove a backup
    #[structopt(alias = "rm")]
    Remove {

        /// The name of the backup to remove
        name: String,

        /// Remove the saved backups
        #[structopt(
        short = "a",
        long = "all",
        )]
        all: bool,
    },

    /// List all the backups
    #[structopt(alias = "ls")]
    List {

        /// List the provided backup saves
        name: Option<String>,
    },

    Profile(Profile),
}

#[derive(StructOpt, Debug)]
pub enum Profile {

    // Add a profile
    #[structopt(alias = "rm")]
    Add {

        /// The name of the profile
        name: String,

        /// The name the bucket where to save the backups of this profile
        bucket_name: String,

        /// The access key of the cloud provider
        access_key: String,

        /// The secret key of the cloud provider
        secret_key: String,

        /// The cloud provider to use
        #[structopt(
        short = "p",
        long = "provider",
        default_value = "aws",
        )]
        provider: String,
    },
}
