# Rust_ParentA

#### ParentA has members 
        [members = [ "projects/ApiServer",
            "projects/B",
            "projects/CopyFiles",
            "projects/DSA",
            "projects/E"
        , "projects/UtilsB"]]

## cargo management addming members/ referencing projects


#### install chocolatey to install c++ crates
- choco install pkgconfiglite -y
## Module Usages


UtilsB project - has a sub module(file helper) that gets imported into project "CopyMissing_files"s sub module and a unit  test is present
refer: md files of UtilsB project.

## test commands

[text](A/projects/CopyFiles/md_testing_readme.md)


# Project details:
CopyMissingFiles: Cli + file handling + logsv

ProjectA calls 
DSA - contains ds & algos
APiServer - API project with rocket + SqliteDB + auth




UtilsB - contains code to help

//comming
//async

//performance metrics

### LifeTime
[ownership_lifetime]](A/projects/DSA/src/c1_ownership_lifetime.rs)
[referencing_borrowing](A/projects/DSA/src/c2_Referencing_Borrowing.rs)
[Pointers_SmartPointers](A/projects/DSA/src/s13_Pointers_Box.rs)