use std::{fs,io};
use std::path::Path;
use anyhow::{Result, Error};

use j4rs::{Instance, InvocationArg, Jvm, JvmBuilder};

fn main() -> anyhow::Result<()> {

    let vec = vec![(1, 'a'), (1, 'b'), (1, 'c'), (2, 'a'), (2, 'b')];
   let uniq_by_fst_comp = || {
    let mut seen = std::collections::HashSet::new();
    vec.iter().copied().filter(move |x| seen.insert(x.0))
    };
     println!("{:?}", uniq_by_fst_comp().collect::<Vec<_>>());
assert_eq!(uniq_by_fst_comp().last(), Some((2, 'a')));
assert_eq!(uniq_by_fst_comp().next_back(), Some((2, 'b')));

assert_eq!(
    uniq_by_fst_comp().fold(vec![], |mut v, x| {v.push(x); v}),
    vec![(1, 'a'), (2, 'a')]
);
assert_eq!(
    uniq_by_fst_comp().rfold(vec![], |mut v, x| {v.push(x); v}),
    vec![(2, 'b'), (1, 'c')]
);

    println!("Hello, world!");
    let dirname= "/Users/Raymond/Downloads/rust-test/hello/target".to_owned();
    let mut filenames = generate_filenames_vector(&dirname)?;
    println!("{:?}", filenames);
    let mut vec2= vec!["test".to_owned(),"test2".to_owned()];
    vec2.iter().for_each(|x| filenames.push(x.to_owned()));
    println!("{:?}", filenames);
    
    let chainiter = filenames.iter().chain(vec2.iter());
    chainiter.clone().for_each(|x| println!("{:?}", x));
    let vec3 = chainiter.collect::<Vec<_>>();
    println!("{:?}", vec3);


    
/* 
    let mut classpaths = vec![];
    for filename in filenames {
        let classpath = format!("{}/{}", dirname, filename).to_owned();
        let classpath_entry = j4rs::ClasspathEntry::new(&filename.clone());
        classpaths.push(classpath_entry);
    }    
    let jvm = JvmBuilder::new().classpath_entries(classpaths).build()?;

    
*/
    Ok(())
}

/* 
fn get_filenames(directory_path: &str) -> io::Result<Vec<String>> {
    let entries = fs::read_dir(Path::new(directory_path))?;
    let filenames: Vec<String> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().into_string().unwrap())
        .collect();
    Ok(filenames)
}
    */

fn generate_filenames_vector(directory_path: &str) -> Result<Vec<String>, Error> {
    let paths = fs::read_dir(directory_path)?;
    let mut filenames = Vec::new();
    for path in paths {
        let path = path?;
        if path.path().is_file() {
            if let Some(filename) = path.path().file_name() {
                if let Some(filename_str) = filename.to_str() {
                    filenames.push(filename_str.to_owned());
                }
            }
        }
    }
    Ok(filenames)
}