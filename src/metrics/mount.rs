extern crate systemstat;

use serde::{Deserialize, Serialize};
use systemstat::{Platform, System};

#[derive(Debug, Serialize, Deserialize)]
pub struct Mount {
    pub files: usize,
    pub files_total: usize,
    pub files_avail: usize,
    pub free: u64,
    pub avail: u64,
    pub total: u64,
    pub name_max: usize,
    pub fs_type: String,
    pub fs_mounted_from: String,
    pub fs_mounted_on: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileSystem {
    pub mounts: Vec<Mount>,
}

/// read_metric returns a list of mounted filesystems
pub fn read_metric(sys: &System) -> Result<String, std::io::Error> {
    match sys.mounts() {
        Ok(mounts) => {
            // file.write_all("\nMounts:\n".to_string().as_bytes()).unwrap();
            let mut mts = vec![];
            for mount in mounts.iter() {
                let mt = Mount {
                    files: mount.files,
                    files_total: mount.files_total,
                    files_avail: mount.files_avail,
                    free: mount.free.as_u64(),
                    avail: mount.avail.as_u64(),
                    total: mount.total.as_u64(),
                    name_max: mount.name_max,
                    fs_type: mount.fs_type.clone(),
                    fs_mounted_from: mount.fs_mounted_from.clone(),
                    fs_mounted_on: mount.fs_mounted_on.clone(),
                };
                mts.push(mt);
            }
            let out = serde_json::to_string(&mts).unwrap();
            Ok(out)
        }
        Err(err) => Err(err),
    }
}
