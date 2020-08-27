// Todo, należy upewnić się, że ma wystarczające uprawnienia do odczytu i usuwania
use std::collections::HashMap;
use std::process;

pub struct DuplicateFinder {
    number_of_checked_files: u64,
    number_of_files_which_has_duplicated_entries: u64,
    number_of_duplicated_files: u64,
    // files : Vec<HashMap<FileEntry, Vec<FileEntry>>>,
    files: HashMap<u64, Vec<FileEntry>>,
    // files : Vec<Vec<FileEntry>>,
    excluded_directories: Vec<String>,
    included_directories: Vec<String>,
}

impl DuplicateFinder {
    pub fn new() -> DuplicateFinder {
        DuplicateFinder {
            number_of_checked_files: 0,
            number_of_files_which_has_duplicated_entries: 0,
            number_of_duplicated_files: 0,
            files: Default::default(),
            excluded_directories: vec![],
            included_directories: vec![],
        }
    }
    // pub fn clear(&mut self) {
    //     self.number_of_checked_files = 0;
    //     self.number_of_files_which_has_duplicated_entries = 0;
    //     self.number_of_duplicated_files = 0;
    //     self.files.clear();
    //     self.excluded_directories.clear();
    //     self.included_directories.clear();
    // }
    // pub fn find_duplicates(&mut self) {}
    // pub fn save_to_file(&self) {}

    /// Setting include directories, panics when there is not directories available
    pub fn set_include_directory(&mut self, mut include_directory: String) {
        if include_directory.len() == 0 {
            println!("At least one directory must be provided")
        }

        include_directory = include_directory.replace("\"", "");
        let directories: Vec<String> = include_directory.split(",").map(String::from).collect();
        let mut checked_directories: Vec<String> = Vec::new();

        for directory in directories {
            if directory == "/" {
                println!("Using / is probably not good idea, you may go out of ram.");
            }
            if directory.contains("*") {
                println!("Include Directory ERROR: Wildcards are not supported, please don't use it.");
                process::exit(1);
            }
            if directory.starts_with("~") {
                println!("Include Directory ERROR: ~ in path isn't supported.");
                process::exit(1);
            }
            if !directory.starts_with("/") {
                println!("Include Directory ERROR: Relative path are not supported.");
                process::exit(1);
            }

            // directory must end with /, due to possiblity of incorrect assumption, that e.g. /home/rafal is top folder to /home/rafalinho
            if !directory.ends_with("/") {
                checked_directories.push(directory + "/");
            } else {
                checked_directories.push(directory);
            }
        }

        if checked_directories.len() == 0 {
            println!("Not found even one correct path to include.");
            process::exit(1);
        }

        self.included_directories = checked_directories;

        println!("Included directories - {:?}", self.included_directories);
    }

    pub fn set_exclude_directory(&mut self, mut exclude_directory: String) {
        if exclude_directory.len() == 0 {
            return;
        }

        exclude_directory = exclude_directory.replace("\"", "");
        let directories: Vec<String> = exclude_directory.split(",").map(String::from).collect();
        let mut checked_directories: Vec<String> = Vec::new();

        for directory in directories {
            if directory == "/" {
                println!("Exclude Directory ERROR: Excluding / is pointless, because it means that no files will be scanned.");
            }
            if directory.contains("*") {
                println!("Exclude Directory ERROR: Wildcards are not supported, please don't use it.");
                process::exit(1);
            }
            if directory.starts_with("~") {
                println!("Exclude Directory ERROR: ~ in path isn't supported.");
                process::exit(1);
            }
            if !directory.starts_with("/") {
                println!("Exclude Directory ERROR: Relative path are not supported.");
                process::exit(1);
            }

            // directory must end with /, due to possiblity of incorrect assumption, that e.g. /home/rafal is top folder to /home/rafalinho
            if !directory.ends_with("/") {
                checked_directories.push(directory + "/");
            } else {
                checked_directories.push(directory);
            }
        }

        self.excluded_directories = checked_directories;

        println!("Excluded directories - {:?}", &self.excluded_directories);
    }

    pub fn debug_print(&self) {
        println!("---------------DEBUG PRINT---------------");
        println!("Number of all checked files - {}", self.number_of_checked_files);
        println!(
            "Number of all files with duplicates - {}",
            self.number_of_files_which_has_duplicated_entries
        );
        println!("Number of duplicated files - {}", self.number_of_duplicated_files);
        println!("Files list - {}", self.files.len());
        println!("Excluded directories - {:?}", self.excluded_directories);
        println!("Included directories - {:?}", self.included_directories);
        println!("-----------------------------------------");
    }
    /// Remove unused entries when included or excluded overlaps with each other or are duplicated
    /// ```
    /// let df : DuplicateFinder = saf
    /// ```
    pub fn optimize_directories(&mut self) {
        let mut optimized_included: Vec<String> = Vec::<String>::new();
        let mut optimized_excluded: Vec<String> = Vec::<String>::new();
        // Remove duplicated entries like: "/", "/"

        self.excluded_directories.sort();
        self.included_directories.sort();

        self.excluded_directories.dedup();
        self.included_directories.dedup();

        // Optimize for duplicated included directories - "/", "/home". "/home/Pulpit" to "/"- TODO
        let mut is_inside: bool;
        for ed_checked in &self.excluded_directories {
            is_inside = false;
            for ed_help in &self.excluded_directories {
                if ed_checked == ed_help {
                    // We checking same element
                    continue;
                }
                if ed_checked.starts_with(ed_help) {
                    is_inside = true;
                    break;
                }
            }
            if is_inside == false {
                optimized_excluded.push(ed_checked.to_string());
            }
        }

        for id_checked in &self.included_directories {
            is_inside = false;
            for id_help in &self.included_directories {
                if id_checked == id_help {
                    // We checking same element
                    continue;
                }
                if id_checked.starts_with(id_help) {
                    is_inside = true;
                    break;
                }
            }
            if is_inside == false {
                optimized_included.push(id_checked.to_string());
            }
        }

        self.included_directories = optimized_included;
        optimized_included = Vec::<String>::new();
        self.excluded_directories = optimized_excluded;
        // optimized_excluded = Vec::<String>::new();

        // Remove include directories which are inside any exclude directory
        for ed in &self.excluded_directories {
            for id in &self.included_directories {
                if id.starts_with(ed) {
                    continue;
                }
                optimized_included.push(id.to_string());
            }
        }
        self.included_directories = optimized_included;
        // optimized_included = Vec::<String>::new();

        if self.included_directories.len() == 0 {
            println!("Optimize Directories ERROR: Excluded directories overlaps all included directories.");
            process::exit(1);
        }

        // Not needed, but better is to have sorted everything
        self.excluded_directories.sort();
        self.included_directories.sort();
    }
}

struct FileEntry {
    file_path: String,
    file_size: u64,
}