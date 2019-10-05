use crate::Object;
use crate::Attribute;
use crate::Attributes;

use libloading as lib;

use std::ffi::c_void;

pub struct File {
    pub path : String,
    pub content : String
}

impl File {
    pub fn new(path: String, content: String) -> File {
        File {
            path,
            content
        }
    }
}

pub struct Files {
    pub files : Vec<File>
}

use std::io::Write;
use std::path::Path;

use std::fs;

impl Files {
    pub fn from(files : Vec<File>) -> Files {
        Files {
            files
        }
    }
    pub fn push(&mut self, file : File) {
        self.files.push(file);
    }
    fn save(&self) {
        for file in &self.files {
            let ligen_path = crate::get_path();
            let path = format!("{}/{}", ligen_path, file.path);
            let path = Path::new(&path);
            let dir = path.parent().unwrap();
            match fs::create_dir_all(&dir) { _ => () }
            let mut output = std::fs::File::create(&path).unwrap();
            output.write_all(file.content.as_bytes()).unwrap();
        }
    }
}

pub struct Generator {
    library: lib::Library,
    generator: *mut c_void
}

impl Generator {
    pub fn new(attribute: &Attribute) -> Result<Generator, String> {
        let (name, attributes) = match attribute {
            Attribute::Group(identifier, attributes) => (&identifier.name, attributes),
            Attribute::Named(_, _) => return Result::Err(format!("[1] Wrong attribute")),
            Attribute::Literal(_) => return Result::Err(format!("[2] Wrong attribute"))
        };

        eprintln!("Loading {}", name);

        let library = lib::Library::new(format!("{}/../ligen_{}", crate::get_path(), name));
        match library {
            Ok(library) => {
                let generator = unsafe {
                     let generator_new : lib::Symbol<extern fn(&Attributes) -> *mut c_void> = library.get(b"Generator_new").unwrap();
                     generator_new(&attributes)
                };
                Result::Ok(Generator{library, generator})
            },
            Err(_) => Result::Err(format!("Library file not found"))
        }
    }

    pub fn generate(&self, object: &Object) {
        let generate : lib::Symbol<extern fn(*mut c_void, &Object) -> *mut Files> = unsafe {
             self.library.get(b"Generator_generate").unwrap()
        };
        let files = unsafe {
            &*generate(self.generator, &object)
        };
        files.save();
    }
}
