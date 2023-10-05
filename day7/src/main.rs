use std::error::Error;
use std::iter::Peekable;

fn directory_from_str(s: &str) -> Result<Entry, String> {
    let mut chunks = s.split_whitespace();
    match (chunks.next(), chunks.next(), chunks.next()) {
        (Some("dir"), Some(name), None) => Ok(Entry::Directory { name: name.into() }),
        _ => Err(format!("Couldn't parse DirectoryEntry from: {:?}", s)),
    }
}

fn file_from_str(s: &str) -> Result<Entry, String> {
    fn report_error(s: &str) -> String {
        format!("Couldn't parse FileEntry from: {:?}", s)
    }
    let mut chunks = s.split_whitespace();
    match (chunks.next(), chunks.next(), chunks.next()) {
        (Some(size), Some(name), None) => {
            let size: usize = size.parse::<usize>().map_err(|_| report_error(s))?;
            Ok(Entry::File {
                size,
                name: name.into(),
            })
        }
        _ => Err(report_error(s)),
    }
}

#[derive(Debug, PartialEq)]
enum Entry {
    Directory { name: String },
    File { size: usize, name: String },
}

#[derive(Debug, PartialEq)]
enum Command {
    Listing { entries: Vec<Entry> },
    ChangeDir { name: String },
}

struct CommandStream<I: Iterator> {
    inner: Peekable<I>,
}

impl<'a, I: Iterator> CommandStream<I>
where
    I: Iterator<Item = &'a str>,
{
    fn new(iter: I) -> Self {
        CommandStream {
            inner: iter.peekable(),
        }
    }
}

impl<'a, I: Iterator<Item = &'a str>> Iterator for CommandStream<I> {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(line) if line.starts_with("$ ") => {
                let chunks: Vec<&str> = line.split_whitespace().collect();
                match &chunks[..] {
                    [_, "cd", name] => Some(Command::ChangeDir {
                        name: name.to_string(),
                    }),
                    [_, "ls"] => {
                        let mut entries = Vec::new();
                        while let Some(&line) = self.inner.peek() {
                            if let Ok(dir_entry) = directory_from_str(line) {
                                entries.push(dir_entry);
                                self.inner.next();
                                continue;
                            }

                            if let Ok(file_entry) = file_from_str(line) {
                                entries.push(file_entry);
                                self.inner.next();
                                continue;
                            }

                            break;
                        }
                        Some(Command::Listing { entries })
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Content {
    Directory { name: String, index: usize },
    File { size: usize, name: String },
}

#[derive(Debug, PartialEq)]
struct FileSystem {
    breadcrumb: Vec<usize>,
    contents: Vec<Vec<Content>>,
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            breadcrumb: vec![0],
            contents: vec![vec![]],
        }
    }

    fn list_contents(&self, index: usize) -> &[Content] {
        &self.contents[index][..]
    }

    fn move_up(&mut self) {
        self.breadcrumb.pop();
        if self.breadcrumb.is_empty() {
            self.breadcrumb.push(0);
        }
    }

    fn move_in(&mut self, name: &str) {
        let slot = self
            .contents
            .get(self.breadcrumb.last().copied().expect("empty"))
            .expect("out of range");
        let index = slot
            .iter()
            .find_map(|elt| match elt {
                Content::File { .. } => None,
                Content::Directory {
                    name: dir_name,
                    index,
                } => {
                    if dir_name == name {
                        Some(index)
                    } else {
                        None
                    }
                }
            })
            .expect("found directory");
        self.breadcrumb.push(*index);
    }

    fn move_root(&mut self) {
        self.breadcrumb.clear();
        self.breadcrumb.push(0);
    }

    fn add_entry_as_content(&mut self, entry: &Entry) {
        let next_index = self.contents.len();
        let slot = self
            .contents
            .get_mut(self.breadcrumb.last().copied().expect("empty"))
            .expect("out of range");
        match entry {
            Entry::File { name, size } => {
                slot.push(Content::File {
                    name: name.clone(),
                    size: *size,
                });
            }
            Entry::Directory { name } => {
                slot.push(Content::Directory {
                    name: name.clone(),
                    index: next_index,
                });
                self.contents.push(vec![]);
            }
        }
    }
}

fn total_size(fs: &FileSystem, index: usize) -> usize {
    let mut result = 0;
    for content in fs.list_contents(index) {
        result += match content {
            Content::Directory { index, .. } => total_size(fs, *index),
            Content::File { size, .. } => *size,
        };
    }
    result
}

fn dispatch(fs: &mut FileSystem, command: Command) {
    match command {
        Command::Listing { entries } => {
            for entry in entries {
                fs.add_entry_as_content(&entry);
            }
        }
        Command::ChangeDir { name } => {
            if name == "/" {
                fs.move_root();
            } else if name == ".." {
                fs.move_up();
            } else {
                fs.move_in(&name);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("adventofcode.com_2022_day_7_input.txt")?;

    let mut fs = FileSystem::new();
    for command in CommandStream::new(input.lines()) {
        dispatch(&mut fs, command);
    }

    // part 1
    println!("part 1: {}",
	     (0..(fs.contents.len())).filter_map(|index| {
					let size = total_size(&fs, index);
					if size <= 100000 {
					    Some(size)
					} else {
					    None
					}
	     }).sum::<usize>());

    // part 2
    let unused = 70000000 - total_size(&fs, 0);
    let target = 30000000 - unused;
    println!("part 2: {:?}",
	     (0..(fs.contents.len())).filter_map(|index| {
		 let size = total_size(&fs, index);
		 if size >= target {
		     Some(size)
		 } else {
		     None
		 }
	     }).min());
    
    Ok(())
}
