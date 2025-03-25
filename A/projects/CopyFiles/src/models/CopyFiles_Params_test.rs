use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CopyFiles_Params_test {
    pub sourceFolderA: String,
    pub sourceFolderB: String,
    pub destinationFolder: String,
    pub copyFiles_flag: bool,
}
