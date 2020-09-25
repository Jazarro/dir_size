use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let path_string = "/";
    let path = Path::new(&path_string);
    let tree = scan_path(path);
    println!("{:?}", tree);
}

#[derive(Debug)]
struct Dir {
    name: OsString,
    local_size: u64,
    size: u64,
    children: Vec<Dir>,
}

impl Dir {
    fn print_maybe(&self) {
        if self.size > 1024 * 1024 * 1024 * 16 {
            println!(
                "{:?}\n\t\tlocal_size: {:?}\n\t\tsize:       {:?}",
                self.name,
                format_size(self.local_size),
                format_size(self.size),
            );
        }
    }
}

fn format_size(in_bytes: u64) -> String {
    let k = in_bytes / 1024;
    if k == 0 {
        return format!("{} bytes", in_bytes);
    }
    let m = k / 1024;
    if m == 0 {
        return format!("{} KB", k);
    }
    let g = m / 1024;
    if g == 0 {
        return format!("{} MB", m);
    }
    let t = g / 1024;
    if t == 0 {
        return format!("{} GB", g);
    }
    return format!("{} TB", t);
}

fn scan_path(parent: &Path) -> io::Result<Dir> {
    let children = get_child_dirs(parent);
    let local_size = get_local_size(parent);
    let size = local_size + children.iter().map(|dir| dir.size).sum::<u64>();
    Ok(Dir {
        name: parent
            .file_name()
            .expect(&format!("Path {:?} ended in .. apparently", parent))
            .to_os_string(),
        local_size: local_size,
        size: size,
        children: children,
    })
}

fn get_child_dirs(parent: &Path) -> Vec<Dir> {
    fs::read_dir(parent)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_dir())
        .map(|path| scan_path(&path).unwrap())
        .inspect(|dir| dir.print_maybe())
        .collect()
}

fn get_local_size(dir: &Path) -> u64 {
    fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file())
        .map(|path| path.metadata().unwrap().len())
        .sum()
}
