use clap::{Arg, App};
use walkdir::WalkDir;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use magic::{Cookie, CookieFlags};
use regex::Regex;

fn main() {

    let n_workers = num_cpus::get(); // each logical core will run a worker
    let mut n_jobs = 0; // this will be incremented

    // Parse command arguments
    let matches = App::new("fast-magic")
        .author("https://github.com/bdlmt/fast-magic")
        .about("Concurrent file search, using libmagic and regex.")
        .arg(Arg::with_name("directory")
            .short("d")
            .long("directory")
            .takes_value(true)
            .help("Directory to walk"))
        .arg(Arg::with_name("regex")
            .short("r")
            .long("regex")
            .takes_value(true)
            .help("Regex pattern to match"))
        .get_matches();

    // Build a vector of grouped filepaths by walking the target directory.
    // Each group will be assigned to a worker.
    let mut filepath_groups = Vec::new();
    for _ in 0..n_workers {
        filepath_groups.push(vec![])
    }
    let dirpath = matches.value_of("directory").unwrap_or(".");
    for entry in WalkDir::new(dirpath).follow_links(true) {
        filepath_groups[n_jobs%n_workers].push(entry.expect("Failed to get WalkDir entry.").path().display().to_string());
        n_jobs += 1; // each filepath discovered will be a job
    }

    // Create a pool of threads to check each filepath using libmagic
    // Each thread will create a magic cookie to scan each filepath in the group handed to it
    let pool = ThreadPool::new(n_workers);
    let (tx, rx) = channel();
    for group in filepath_groups {
        let tx = tx.clone();
        let filepaths = group.clone();
        pool.execute(move|| {
            let cookie = Cookie::open(CookieFlags::default()).unwrap();
            let databases = vec!["/usr/share/misc/magic.mgc"];
            assert!(cookie.load(&databases).is_ok());
            for filepath in filepaths {
                let filetype = cookie.file(&filepath).expect("Failed to scan file using libmagic.");
                tx.send((filepath.clone(), filetype.clone())).expect("Failed to transmit via message channel from worker thread!");
            }
        });
    }

    // Construct a regex pattern matcher and use it to check each magic cookie result
    let re_pattern = matches.value_of("regex").unwrap_or("");
    let re = Regex::new(re_pattern).unwrap();
    for _ in 0..n_jobs {
        let (filepath, filetype) = rx.recv().expect("Failed to receive from message channel!");
        if re.is_match(&filetype) {
           println!("{}: {}", &filepath, &filetype);
        }
    }
}
