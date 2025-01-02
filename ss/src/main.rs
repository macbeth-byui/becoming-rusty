use std::collections::HashMap;
use std::fs::File;
use std::io::{self, stdin, BufReader, Read};
use serde::Deserialize;
use serde_json::Result;

// {
//     "books": [
//         {
//             "book": "1 Nephi",
//             "chapters": [
//                 {
//                     "chapter": 1,
//                     "reference": "1 Nephi 1",
//                     "verses": [
//                         {
//                             "reference": "1 Nephi 1:1",
//                             "text": "I, Nephi, having been born of goodly parents, therefore I was taught somewhat in all the learning of my father; and having seen many afflictions in the course of my days, nevertheless, having been highly favored of the Lord in all my days; yea, having had a great knowledge of the goodness and the mysteries of God, therefore I make a record of my proceedings in my days.",
//                             "verse": 1
//                         },

#[derive(Debug, Deserialize)]
struct Scriptures {
    pub books : Vec<Book>,
}

#[derive(Debug, Deserialize)]
struct Book {
    pub book : String,
    pub chapters : Vec<Chapter>,
}

#[derive(Debug, Deserialize)]
struct Chapter {
    pub chapter : u8,
    pub reference : String,
    pub verses : Vec<Verse>,
}

#[derive(Debug, Deserialize)]
struct Verse {
    pub reference : String,
    pub text : String,
    pub verse : u8,
}

fn load_json(filename : &str) -> io::Result<Scriptures> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    let scriptures : Scriptures = serde_json::from_str(buffer.as_str())?;
    Ok(scriptures)
}

fn analyze_scriptures(scriptures : &Scriptures) -> HashMap<String, u32> {
    let mut results = HashMap::new();
    for book in scriptures.books.iter() {
        for chapter in book.chapters.iter() {
            for verse in chapter.verses.iter() {
                let words: Vec<&str> = verse.text.split(
                    &[' ','.',',',':',';','?','!','(',')','—']).collect();
                for word in words {
                    if let Some(count) = results.get_mut(word.to_uppercase().as_str()) {
                        *count += 1;
                    } else {
                        results.insert(word.to_uppercase(), 1);
                    }
                }
            }
        }
    }
    results

}

fn search_scriptures(scriptures : &Scriptures, text : &str) -> Vec<String> {
    let mut results = Vec::new();
    for book in scriptures.books.iter() {
        for chapter in book.chapters.iter() {
            for verse in chapter.verses.iter() {
                if verse.text.contains(text) {
                    results.push(format!("[{}] - {}", verse.reference, verse.text));
                }
            }
        }
    }
    results
}

fn main() {
    let scriptures = load_json("bookofmormon.json").unwrap();
    // let mut search = String::new();
    // stdin().read_line(&mut search).expect("Error reading stdin");
    // let results = search_scriptures(&scriptures, &search[..search.len()-2]);
    // for result in results {
    //     println!("{}", result);
    // }
    let results = analyze_scriptures(&scriptures);
    println!("{:?}", results);
}
