use core::cell::RefCell;
use core::fmt::Debug;
use std::{
    fmt::Display,
    io::{stdin, BufRead},
    rc::Rc,
};

struct File {
    name: String,
    parent: Rc<Directory>,
    size: usize,
}

impl Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("File")
            .field("name", &self.name)
            .field("parent", &self.parent.name)
            .field("size", &self.size)
            .finish()
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}\n", self.size, self.name))
    }
}

struct Directory {
    name: String,
    parent: Option<Rc<Directory>>,
    files: RefCell<Vec<File>>,
    directories: RefCell<Vec<Rc<Directory>>>,
}

impl Debug for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Directory")
            .field("name", &self.name)
            .field("parent", &self.parent_name())
            .field("files", &self.files)
            .field("directories", &self.directories)
            .finish()
    }
}

impl Directory {
    fn parent_name(&self) -> String {
        match &self.parent {
            Some(d) => d.name.to_owned(),
            None => "/".to_owned(),
        }
    }

    fn format_output(&self, indent: usize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indentstr = std::iter::repeat(" ").take(indent).collect::<String>();
        f.write_fmt(format_args!("{}{}/\n", indentstr, self.name))?;
        for dir in self.directories.borrow().iter() {
            dir.format_output(indent + 2, f)?;
        }
        for file in self.files.borrow().iter() {
            f.write_fmt(format_args!("{}  ", indentstr))?;
            Display::fmt(file, f)?
        }

        Ok(())
    }

    fn all_dirs(&self) -> Vec<Rc<Directory>> {
        let mut dirs: Vec<Rc<Directory>> = Vec::new();
        self._all_dir_impl(&mut dirs);
        dirs
    }

    fn _all_dir_impl(&self, dirs: &mut Vec<Rc<Directory>>) {
        for d in self.directories.borrow().iter() {
            dirs.push(d.clone());
            d._all_dir_impl(dirs);
        }
    }

    fn size(&self) -> usize {
        self.files.borrow().iter().map(|f| f.size).sum::<usize>()
            + self
                .directories
                .borrow()
                .iter()
                .map(|d| d.size())
                .sum::<usize>()
    }
}

impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format_output(0, f)
    }
}

fn command_processor<'a>(
    input: &mut Box<impl Iterator<Item = String>>,
    current_dir: &Rc<Directory>,
    top_dir: &Rc<Directory>,
) -> Rc<Directory> {
    let mut next_input = input.next();
    let mut my_current_dir = current_dir.clone();
    while let Some(line) = next_input {
        (next_input, my_current_dir) = handle_command_line(
            &line.split_whitespace().collect::<Vec<_>>(),
            input,
            &my_current_dir,
            top_dir,
        );
    }
    my_current_dir.clone()
}

fn handle_command_line<'a>(
    tokens: &[&str],
    input: &mut Box<impl Iterator<Item = String>>,
    current_dir: &Rc<Directory>,
    top_dir: &Rc<Directory>,
) -> (Option<String>, Rc<Directory>) {
    if tokens.len() < 2 || tokens[0] != "$" {
        panic!("Invalid input: {:?}", tokens);
    }
    let command = tokens[1];
    match command {
        "cd" => (input.next(), handle_cd(tokens[2], current_dir, top_dir)),
        "ls" => (handle_ls(input, current_dir), Rc::clone(&current_dir)),
        _ => panic!("Invalid input: {:?}", tokens),
    }
}

fn handle_cd<'a>(
    name: &str,
    current_dir: &Rc<Directory>,
    top_dir: &Rc<Directory>,
) -> Rc<Directory> {
    match name {
        "/" => top_dir.clone(),
        ".." => match &current_dir.parent {
            Some(dir) => dir.clone(),
            None => current_dir.clone(),
        },
        _ => Rc::clone(
            current_dir
                .directories
                .borrow()
                .iter()
                .filter(|d| d.name == name)
                .next()
                .unwrap(),
        ),
    }
}

fn handle_ls<'a>(
    mut input: impl Iterator<Item = String>,
    current_dir: &Rc<Directory>,
) -> Option<String> {
    while let Some(line) = input.next() {
        let tokens: Vec<_> = line.split_whitespace().collect();
        if tokens.len() < 2 {
            panic!("Invalid input: {}", line);
        }
        match tokens[0] {
            "$" => return Some(line),
            "dir" => current_dir
                .directories
                .borrow_mut()
                .push(Rc::new(Directory {
                    name: tokens[1].to_string(),
                    parent: Some(current_dir.clone()),
                    files: RefCell::new(Vec::new()),
                    directories: RefCell::new(Vec::new()),
                })),
            _ => current_dir.files.borrow_mut().push(File {
                name: tokens[1].to_string(),
                parent: current_dir.clone(),
                size: tokens[0].parse::<usize>().unwrap(),
            }),
        };
    }
    None
}

fn main() {
    let input = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty());

    let top_dir = Rc::new(Directory {
        name: "".to_string(),
        parent: None,
        files: RefCell::new(Vec::new()),
        directories: RefCell::new(Vec::new()),
    });
    command_processor(&mut Box::new(input), &top_dir, &top_dir);

    println!("Directory structure:\n{}", top_dir);

    let all_subdirs = top_dir.all_dirs();
    let all_dirs: Vec<_> = std::iter::once(&top_dir)
        .chain(all_subdirs.iter())
        .collect();

    let mut directory_sizes: Vec<_> = all_dirs.iter().map(|d| d.size()).collect();

    println!(
        "Sum of all dirs with size at most 100000: {}",
        directory_sizes
            .iter()
            .map(|d| {
                match *d {
                    s if s <= 100000 => s,
                    _ => 0,
                }
            })
            .sum::<usize>()
    );

    let total_size = directory_sizes[0];
    println!("Total size of files: {}", total_size);
    let space_free = 70000000 - total_size;
    let space_needed_for_update = 30000000 - space_free;
    println!("Space needed for update: {}", space_needed_for_update);

    directory_sizes.sort();

    println!(
        "Size of smallest directory that can be deleted: {}",
        directory_sizes
            .iter()
            .filter(|&&s| s > space_needed_for_update)
            .next()
            .unwrap()
    );
}
