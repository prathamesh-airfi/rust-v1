#[allow(unused_variables)]
#[derive(Debug)]
struct File {
    name: String,
}

#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>,
}

impl Folder {
    fn new(name: String) -> Self {
        Self {
            name,
            contents: vec![],
        }
    }

    fn crete_file(&mut self, name: String) {
        self.contents.push(File { name });
    }

    fn delete_file(&mut self, index: usize) -> File {
        self.contents.remove(index)
    }

    fn get_file(&self, index: usize) -> Option<&File> {
        self.contents.get(index)
    }
}

fn main() {
    let mut music_folder = Folder::new("Music".to_string());

    music_folder.crete_file("in the name of love.mp3".to_string());
    music_folder.crete_file("ignite".to_string());

    music_folder.delete_file(0);

    println!("{:?}", music_folder);

    match music_folder.get_file(0) {
        Some(file) => println!("{file:#?}"),
        None => println!("There was no file"),
    }
}
