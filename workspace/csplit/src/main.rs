use nix::unistd::ftruncate; // For ftruncate
use once_cell::sync::OnceCell;
use regex::Regex;
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::iterator::Signals;
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::os::unix::io::AsRawFd; // For fileno
use std::path::PathBuf;
use std::process;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;

pub const PATH_MAX: usize = 4096;

// Declare a static AtomicU32
static COUNTER: AtomicU32 = AtomicU32::new(0);
static PREFIX: OnceCell<String> = OnceCell::new();
static SUFFLEN: OnceCell<usize> = OnceCell::new();
static KFLAG: OnceCell<bool> = OnceCell::new();
static PROG_NAME: OnceCell<String> = OnceCell::new();
static INITIALIZED: AtomicBool = AtomicBool::new(false);

// Global state to simulate C global variables
struct GlobalState {
    prefix: String,
    sufflen: usize,
    sflag: bool,
    kflag: bool,
    lineno: i64,
    // reps: i64,
    nfiles: i64,
    // maxfiles: i64,
    currfile: PathBuf,
    infn: String,
    infile: Option<Box<BufReader<File>>>,
    overfile: Option<File>,
    truncofs: i64,
    doclean: bool,
}

// create global state for cleanup purposes

impl GlobalState {
    fn new() -> Self {
        Self {
            prefix: String::from("xx"),
            sufflen: 2,
            sflag: false,
            kflag: false,
            lineno: 0,
            // reps: 0,
            nfiles: 0,
            // maxfiles: 0,
            currfile: PathBuf::new(),
            infn: String::new(),
            infile: None,
            overfile: None,
            truncofs: 0,
            doclean: false,
        }
    }
}

impl Clone for GlobalState {
    fn clone(&self) -> Self {
        Self {
            prefix: self.prefix.clone(),
            sufflen: self.sufflen,
            sflag: self.sflag,
            kflag: self.kflag,
            lineno: self.lineno,
            nfiles: self.nfiles,
            currfile: self.currfile.clone(),
            infn: self.infn.clone(),
            infile: None,
            overfile: None,
            truncofs: self.truncofs,
            doclean: self.doclean,
        }
    }
}

fn clean_up_handler() -> io::Result<()> {
    if !INITIALIZED.load(Ordering::Acquire) {
        return Ok(());
    }

    if *KFLAG.get().unwrap() {
        return Ok(());
    }

    // remove tempfile.tmp if it exists
    let _ = fs::remove_file("tempfile.tmp");
    let counter = COUNTER.load(Ordering::Acquire);
    let pref = PREFIX.get().unwrap();
    let sufflen = SUFFLEN.get().unwrap();
    for i in 0..counter {
        let filename = format!("{}{:0width$}", pref, i, width = sufflen);
        let path = PathBuf::from(&filename);
        let _ = fs::remove_file(path); // Ignore errors in case file doesn't exist
    }
    Ok(())
}

#[allow(dead_code)]
fn cleanup(state: &GlobalState) -> io::Result<()> {
    if state.kflag {
        return Ok(());
    }

    // remove tempfile.tmp if it exists
    let _ = fs::remove_file("tempfile.tmp");
    for i in 0..state.nfiles {
        let filename = format!("{}{:0width$}", state.prefix, i, width = state.sufflen);
        let path = PathBuf::from(&filename);
        let _ = fs::remove_file(path); // Ignore errors in case file doesn't exist
    }
    Ok(())
}

fn usage() {
    eprintln!("usage: csplit [-ks] [-f prefix] [-n number] file args ...");
    process::exit(1);
}

fn usage_and_exit(err: &str) {
    if let Some(p) = PROG_NAME.get() {
        eprintln!("{}: {}", p, err);
    } else {
        eprintln!("{}", err);
    }
    eprintln!("usage: csplit [-ks] [-f prefix] [-n number] file args ...");
    process::exit(1);
}

fn print_error_and_exit(err: &str) {
    if let Some(p) = PROG_NAME.get() {
        eprintln!("{}: {}", p, err);
    } else {
        eprintln!("{}", err);
    }
    clean_up_handler().expect("Cleanup failed");
    process::exit(1);
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        usage();
    }

    PROG_NAME.set(args[0].clone()).unwrap();

    let mut state = GlobalState::new();

    let mut patterns = Vec::new();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-f" => {
                i += 1;
                state.prefix = args[i].clone();
            }
            "-k" => state.kflag = true,
            "-n" => {
                i += 1;
                state.sufflen = args[i].parse().unwrap_or_else(|_| {
                    print_error_and_exit(format!("{}: bad suffix length", args[i]).as_str());
                    0
                });
            }
            "-s" => state.sflag = true,
            _ => {
                if args[i].starts_with("-") {
                    if args[i].len() == 2 {
                        usage_and_exit(format!("{}: invalid option", args[i]).as_str());
                    }
                }
                break;
            }
        }
        i += 1;
    }

    KFLAG.set(state.kflag).unwrap();
    SUFFLEN.set(state.sufflen).unwrap();
    PREFIX.set(state.prefix.clone()).unwrap();
    INITIALIZED.store(true, Ordering::Release);

    if state.sufflen + state.prefix.len() >= PATH_MAX {
        print_error_and_exit("name too long");
    }

    if i == args.len() {
        usage();
    }

    state.infn = args[i].clone();
    i += 1;

    if state.infn == "-" {
        let stdin = io::stdin();
        let handle = stdin.lock();
        let filepath = "tempfile.tmp";

        let file = File::create(filepath)?;
        let mut writer = BufWriter::new(file);

        for line in handle.lines() {
            let line = line?;
            writeln!(writer, "{}", line)?;
        }

        state.infile = Some(Box::new(BufReader::new(File::open(filepath)?)));
    } else {
        let file_result = File::open(&state.infn);
        match file_result {
            Ok(f) => {
                state.infile = Some(Box::new(BufReader::new(f)));
            }
            Err(e) => {
                print_error_and_exit(format!("{}: {}", &state.infn, e.to_string()).as_str());
            }
        }

        // state.infile = Some(Box::new(BufReader::new(file)));
    }

    if !state.kflag {
        state.doclean = true;
        let mut signals = Signals::new(TERM_SIGNALS)?;
        let doclean_clone = Arc::new(AtomicBool::new(state.doclean));

        std::thread::spawn(move || {
            for _sig in signals.forever() {
                if doclean_clone.load(Ordering::Relaxed) {
                    clean_up_handler().expect("Cleanup failed");
                }
                process::exit(1);
            }
        });
    }

    // ensure 10^sufflen < LONG_MAX
    let mut maxfiles = 1;
    for i in 0..state.sufflen {
        if maxfiles > std::i64::MAX / 10 {
            print_error_and_exit(
                format!("{}: suffix length too long (limit {})", state.sufflen, i).as_str(),
            );
        }
        maxfiles *= 10;
    }

    let mut rep_count = 0;

    while i < args.len() {
        if args[i].starts_with("/") || args[i].starts_with("%") {
            let temp = args[i].clone();
            patterns.push(temp);
        } else if args[i].chars().all(char::is_numeric) {
            patterns.push(args[i].clone());
        } else if args[i].starts_with("{") {
            // read repition count
            let mut temp = args[i].clone();
            i += 1;
            while i < args.len() && !args[i].ends_with("}") {
                temp += &(" ".to_owned() + &args[i].clone());
                i += 1;
            }

            // remove first and last characters
            temp = temp[1..temp.len() - 1].to_string();

            if i == args.len() && !temp.chars().all(char::is_numeric) {
                print_error_and_exit(format!("{}: bad repetition count", temp).as_str());
            }

            rep_count = temp.parse::<i64>().unwrap_or_else(|_| {
                print_error_and_exit(format!("{}: bad repetition count", temp).as_str());
                0
            });
        } else {
            usage_and_exit(format!("{}: invalid option", args[i]).as_str());
        }
        i += 1;
    }

    // if !state.kflag {
    //     // Rust equivalent of cleanup on program exit would involve implementing Drop trait or explicitly cleaning up before exiting.
    //     state.doclean = true;
    // }

    // Actual file handling
    // let infile = state.infile.take().unwrap(); // Safe because we just set this
    // let reader = BufReader::new(infile);
    for expr in patterns {
        if expr.starts_with('/') || expr.starts_with('%') {
            do_rexp(&mut state, &expr)?;
        } else if expr.chars().all(char::is_numeric) {
            do_lineno(&mut state, &expr)?;
            // Implement do_lineno function logic here
        } else {
            print_error_and_exit(format!("{}: unrecognized pattern", expr).as_str());
        }
    }

    // print the rest of the file
    // Temporarily take the infile out of state to avoid multiple mutable borrows.
    if let Some(infile) = state.infile.take() {
        let mut ofp = newfile(&mut state)?;
        let mut ofp_writer = BufWriter::new(&mut ofp);
        let mut reader = BufReader::new(infile);
        let mut line = String::new();
        let mut cont = true;

        if rep_count > 0 {
            while reader.read_line(&mut line)? != 0 {
                for _ in 0..rep_count {
                    ofp_writer.write_all(line.as_bytes())?;
                    line.clear();
                    if reader.read_line(&mut line)? == 0 {
                        cont = false;
                        break;
                    }
                }
                ofp_writer.flush()?;
                drop(ofp_writer);
                if cont {
                    ofp = newfile(&mut state)?;
                }
                ofp_writer = BufWriter::new(&mut ofp);
            }
        } else {
            while reader.read_line(&mut line)? != 0 {
                ofp_writer.write_all(line.as_bytes())?;
                line.clear(); // Make sure to clear the line buffer after writing it out.
            }
            ofp_writer.flush()?;
        }

        // Flush the output writer to ensure all data is written out.

        // Once done, put the infile back into state
        // state.infile.replace(BufReader::new(reader.into_inner()));
    }
    Ok(())
}

fn newfile(state: &mut GlobalState) -> io::Result<File> {
    let filename = format!(
        "{}{:0width$}",
        state.prefix,
        state.nfiles,
        width = state.sufflen
    );
    let path = PathBuf::from(&filename);

    if path.as_os_str().len() as usize >= PATH_MAX {
        print_error_and_exit(format!("{}: filename too long", filename).as_str());
    }

    // Use OpenOptions to create the file with both read and write permissions
    let file = OpenOptions::new()
        .create(true) // Create the file if it does not exist
        .read(true) // Open the file with read permission
        .write(true) // Open the file with write permission
        .open(&path)?;

    state.currfile = path;
    state.nfiles += 1;
    COUNTER.fetch_add(1, Ordering::AcqRel);

    Ok(file)
}

fn toomuch(state: &mut GlobalState, ofp: &mut File, mut n: i64) -> io::Result<()> {
    // Truncate and drop the overflow file if it exists
    if let Some(overfile) = state.overfile.take() {
        overfile.sync_all()?;
        let fd = overfile.as_raw_fd();
        ftruncate(fd, state.truncofs as i64)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    }

    if n == 0 {
        return Ok(());
    }

    let mut buf = [0; 4096];
    #[allow(unused_assignments)]
    let mut nread = 0;
    let mut count_back = 0;
    loop {
        let current_pos = ofp.stream_position()?;
        if current_pos < buf.len() as u64 {
            ofp.rewind()?;
        } else {
            ofp.seek(SeekFrom::Current(-(buf.len() as i64)))?;
        }
        nread = ofp.read(&mut buf)?;

        if nread == 0 {
            break;
        }

        for i in (0..nread).rev() {
            if buf[i] == b'\n' {
                n -= 1;
                if n == 0 {
                    break;
                }
            } else {
                count_back += 1;
            }
        }

        if current_pos == 0 || n == 0 {
            break;
        }
    }

    let curr_pos = ofp.stream_position()?;

    if nread > 0 {
        if count_back as usize >= curr_pos as usize {
            ofp.seek(SeekFrom::Start(0))?;
        } else {
            ofp.seek(SeekFrom::End(-(count_back + 1 as i64)))?;
        }

        let new_len = ofp.stream_position()?;
        ofp.set_len(new_len)?;

        if let Some(ref mut infile) = state.infile {
            let infile_pos = infile.stream_position()?;
            if infile_pos < count_back as u64 {
                infile.seek(SeekFrom::Start(0))?;
            } else {
                infile.seek(SeekFrom::Current(-(count_back + 1 as i64)))?;
            }
        }
    }

    // Prepare for next operations
    let new_pos = ofp.stream_position()?;
    state.overfile = Some(ofp.try_clone()?); // Clone ofp to overfile
    state.truncofs = new_pos as i64;

    Ok(())
}

fn do_rexp(state: &mut GlobalState, expr: &str) -> io::Result<()> {
    let mut offset: i64 = 0;
    let mut re = &expr[1..]; // Extract regex pattern
    if !re.ends_with('/') && !re.ends_with('%') {
        if let Some(idx) = expr.find(|c| c == '+' || c == '-') {
            let offset_str = &expr[idx + 1..];
            offset = offset_str.parse::<i64>().unwrap_or_else(|_| {
                print_error_and_exit(format!("{}: bad offset", expr).as_str());
                0
            });
            if let Some(_idx) = expr.find(|c| c == '-') {
                offset = -offset;
            }
            re = &expr[1..idx - 1];
            // check character after re
            let last_char = &expr[idx - 1..idx];
            if !last_char.ends_with('/') && !last_char.ends_with('%') {
                print_error_and_exit(format!("{}: bad offset", expr).as_str());
            }
        } else {
            print_error_and_exit(format!("{}: bad offset", expr).as_str());
        }
    } else {
        re = &expr[1..expr.len() - 1];
    }

    let regex = Regex::new(re).unwrap_or_else(|_| {
        print_error_and_exit(format!("{}: bad regular expression", re).as_str());
        Regex::new("").unwrap()
    });

    let mut ofp = if expr.starts_with('/') {
        // Save results to a file
        newfile(state)?
    } else {
        let result = File::create("tempfile.tmp");
        match result {
            Ok(file) => file,
            Err(_) => {
                print_error_and_exit("tempfile.tmp: file creation failed");
                File::create("tempfile.tmp")?
            }
        }
    };

    let mut ofp_writer = BufWriter::new(&ofp);

    let mut matched = false;

    if let Some(ref mut infile) = state.infile {
        let mut line = String::new();

        loop {
            if infile.read_line(&mut line)? == 0 {
                state.infile = None;
                break; // EOF reached
            }

            if regex.is_match(&line) {
                // Matches the line, re-add the line to the infile
                if offset >= 0 {
                    if offset == 0 {
                        infile.seek(SeekFrom::Current(-(line.len() as i64)))?;
                        line.clear();
                        matched = true;
                        break;
                    }
                    ofp_writer.write_all(line.as_bytes())?;
                    line.clear();
                    offset -= 1;
                    for _ in 0..offset.abs() {
                        if infile.read_line(&mut line)? == 0 {
                            state.infile = None;
                            break;
                        }
                        state.lineno += 1;
                        ofp_writer.write_all(line.as_bytes())?;
                        line.clear();
                    }
                } else {
                    infile.seek(SeekFrom::Current(-(line.len() as i64)))?;
                    drop(ofp_writer);
                    toomuch(state, &mut ofp, offset.abs() + 1)?;
                    state.lineno += offset;
                }
                matched = true;
                break;
            }

            ofp_writer.write_all(line.as_bytes())?;
            state.lineno += 1;
            line.clear();
        }
    }

    if !matched {
        print_error_and_exit(format!("{}: no match", expr).as_str());
    }

    Ok(())
}

fn do_lineno(state: &mut GlobalState, expr: &str) -> io::Result<()> {
    let tgtline: i64 = expr.parse().unwrap_or_else(|_| {
        print_error_and_exit(format!("{}: bad line number", expr).as_str());
        1
    });

    if tgtline <= 0 {
        let err_str = format!("{}: bad line number", expr);
        print_error_and_exit(err_str.as_str());
    }

    if tgtline <= state.lineno {
        let err_str = format!("{}: can't go backwards", expr);
        print_error_and_exit(err_str.as_str());
    }

    let ofp = newfile(state)?;
    let mut ofp_writer = BufWriter::new(&ofp);
    if let Some(ref mut infile) = state.infile {
        let mut line = String::new();

        while state.lineno < tgtline - 1 {
            if infile.read_line(&mut line)? == 0 {
                print_error_and_exit(format!("{}: out of range", expr).as_str());
            }
            ofp_writer.write_all(line.as_bytes())?;
            state.lineno += 1;
            line.clear();
        }
    }

    Ok(())
}
