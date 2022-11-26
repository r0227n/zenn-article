use std::env;
use std::fs::File;
use core::str::FromStr;
use std::io::{prelude::*, BufReader, Error};
use std::collections::HashMap; 
use serde::{Serialize, Deserialize};

#[derive(Debug)]
#[allow(dead_code)]
pub struct CommitInfo {
    date: String,
    files: Vec<CommitFile>,
    current_directory: String,
}

#[derive(Debug)]
#[allow(dead_code)]
struct CommitFile {
    status: CommitStatus,
    path: String,
}

#[derive(Debug)]
enum CommitStatus {
    M, // modified
    T, // file type changed (regular file, symbolic link or submodule)
    A, // added
    D, // deleted
    R, // renamed
    C, // copied (if config option status.renames is set to "copies")
    U, // updated but unmerged
}

impl FromStr for CommitStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "M" => Ok(CommitStatus::M),
            "T" => Ok(CommitStatus::T),
            "A" => Ok(CommitStatus::A),
            "D" => Ok(CommitStatus::D),
            "R" => Ok(CommitStatus::R),
            "C" => Ok(CommitStatus::C),
            "U" => Ok(CommitStatus::U),
            _ => Err(()),
        }
    }
}

impl CommitInfo {
    fn new(date: String, files: Vec<CommitFile>, current_directory: String) -> CommitInfo {
        CommitInfo { date, files, current_directory }
    }

    fn env() -> Result<CommitInfo, &'static str> {
        let commit_info_path = env::var("COMMIT_INFO_PATH").expect("COMMIT_INFO_PATH not found");
        println!("commit_info_path: {}", commit_info_path);
        let input = File::open(commit_info_path).expect("Unable to open file");
        let reader = BufReader::new(input);

        let timestamp = env::var("TIMESTAMP").expect("COMMIT_INFO_PATH not found");
        let current_directory = env::var("WORKSPACE").expect("WORKSPACE not found");
        let mut files = Vec::<CommitFile>::new();
        
        for line in reader.lines() {
            let line = line.unwrap();

            let parse = Self::parse_commit_file(line).unwrap();
            println!("{:?}", parse);
            files.push(parse);
        }

        Ok(Self::new(timestamp, files, current_directory))
    }

    fn parse_commit_file(line: String) -> Result<CommitFile, &'static str> {
        let mut iter = line.split_whitespace();
        let status = iter.next();
        let file = iter.next();

        Ok(CommitFile {
            status: CommitStatus::from_str(status.unwrap()).unwrap(),
            path: file.unwrap().to_string(),
        })
    }
}

fn main() {
    let commit_info = CommitInfo::env().unwrap();
 
    for file in commit_info.files {
        let zenn = Zenn::new(commit_info.current_directory.clone(), file.path).unwrap();
        let index = Index::read(commit_info.current_directory.clone());

        let hoge = Index::write(index,zenn, commit_info.current_directory.clone());
        println!("{:?}",hoge);
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct Index {
    total: i32,
    update_at: String,
    articles: Vec<Zenn>,
}

impl Index {
    const FILE_NAME: &'static str = "index.json";

    fn read(read_directory: String) -> Index {
        if let Ok(file) = File::open(read_directory + "/" + Self::FILE_NAME) {
            let reader = BufReader::new(file);
            if let Ok(index) = serde_json::from_reader(reader) {
                return index;
            }
        }

        return Index {
            total: 0,
            update_at: String::from(""),
            articles: Vec::<Zenn>::new(),
        };
    }

    fn write(mut self, article: Zenn, write_directory: String) -> Result<File, Error> {
        let contains_updated = self.articles.iter().any(|x| x.path == article.path);
        let contains_path = self.articles.iter().any(|x| x.path == article.path);
        if contains_updated && contains_path {
            return Err(Error::new(std::io::ErrorKind::Other, "already exists"));
        }

        self.update_at = String::from("2021-01-01");
        self.articles.push(article);
        self.total = self.articles.len() as i32;

        let json = serde_json::to_string_pretty(&self)?;

        let mut file = File::create(write_directory + "/" + Self::FILE_NAME)?;
        file.write_all(json.as_bytes())?;
        Ok(file)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Zenn {
    title: String,
    emoji: String,
    r#type: String,
    topics: Vec<String>,
    published: bool,
    updated_at: String,
    path: String,
}

enum SplitMode {
    Ready,
    Start,
    End,
}

#[derive(Serialize, Deserialize, Debug)]
struct Mardkdown {
    key: String,
    value: String,
}

impl Mardkdown {
    // [Markdown] を新規に作成する
    fn new(row: String) -> Mardkdown {
        let words: Vec<&str> = row.split(":").collect::<Vec<&str>>();
        let key: String = words.first().unwrap().trim().to_string();
        let value: &str = words.last().unwrap().trim();

        Mardkdown {
            key: key.trim().to_string(),
            value: value.replace("\"", ""),
        }
    }
}

impl Zenn {
    // [HashMap]から[Zenn]を作成する
    fn from_map(map: HashMap<String, String>) -> Zenn {
        return Zenn {
            title: map.get("title").unwrap().to_string(),
            emoji: map.get("emoji").unwrap().to_string(),
            r#type: map.get("type").unwrap().to_string(),
            topics: map.get("topics").unwrap().trim_start_matches('[').trim_end_matches(']').split(',').map(|x| x.trim().to_string()).collect::<Vec<String>>(),
            published: map.get("published").unwrap().parse::<bool>().unwrap(),
            updated_at: map.get("update_at").unwrap().to_string(),
            path: map.get("path").unwrap().to_string(),
        };
    }

    fn new(directory: String, path: String) -> Result<Zenn, Error> {
        const SPLITE: &str = "---";

        let mut body = HashMap::new();
        body.insert(String::from("update_at"), String::from("2021-01-01"));
        body.insert(String::from("path"), path.clone());

        let input = File::open(directory + &path)?;
        let buffered = BufReader::new(input);
        let mut mode = SplitMode::Ready;

        for line in buffered.lines() {
            match mode {
                SplitMode::Ready => {
                    if SPLITE == line.as_ref().unwrap() {
                        mode = SplitMode::Start;
                        continue;
                    }
                }
                SplitMode::Start => {
                    if SPLITE == line.as_ref().unwrap() {
                        mode = SplitMode::End;
                        continue;
                    }
                }
                SplitMode::End => {
                    break;
                }
            }

            let content = Mardkdown::new(line.unwrap());
            body.entry(content.key).or_insert(content.value.trim().to_string());
        }

        Ok(Self::from_map(body))
    }
}
