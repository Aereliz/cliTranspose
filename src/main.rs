use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::Write;

fn main() {

    // Reading CLI args(Filename)
    let args: Vec<String> = env::args().collect();
    // setting filename to file
    let file = &args[1];

    println!("Searching for file {}", file);

    // if file exists at specified location
    if let Ok(contents) = fs::read_to_string(file) {
        println!("Content:\n{}", contents);
        // cts = cout of rows
        let mut cts:usize = 0;
        // Init Vector of parsed numbers
        let mut parsed:Vec<isize>=Vec::new();
        // Parsing file line by line
        for line in contents.lines() {
            let mut a:Vec<&str> = line.split_whitespace().collect();
            cts+=1;
            while let Some(strvalue) = a.pop() {
                parsed.push(strvalue.parse().unwrap());
            }
            println!("{:#?}",a);
        }

        // Creating dir for output
        match fs::create_dir_all("out") {
            Err(why) => panic!("couldn't create folder out {}" , why.to_string()),
            Ok(_) => println!("successfully created folder out"),
        }

        let path = Path::new("out\\output");
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why.to_string()),
            Ok(file) => file,
        };

        let mut output:String = String::new();
        // count of output rows
        let rows = parsed.len()/cts as usize;
        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        for i in 0..rows{
            let mut j = rows - i - 1;
            while j<parsed.len(){
                output.push_str(parsed[j].to_string().as_ref());
                output.push_str(" ");
                j+=rows;
            }
            output.push_str("\n");
        }
        match file.write_all(output.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why.to_string()),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    } else {
        println!("File not exists!");
    }
}
