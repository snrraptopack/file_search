use std::collections::{HashMap, HashSet};
use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::{Lines, BufReader, BufRead};


struct Config<'a> {
    file_path:&'a String,
    search_query:&'a [String]
}

impl Config<'_>{
    fn new(args:&[String])->Result<Config,String>{
        if args.len() < 3 {
            Err("Invalid command provided ie cargo run -- file_name query".to_string())
        }else{
            let search_query = &args[2..];
            let file_path = &args[1];
            Ok(Config{file_path,search_query})
        }
    }
}

fn open_and_read_file(file_input:&str)-> Result<Lines<BufReader<File>>,Box<dyn Error>>{
    let file = File::open(file_input)?;
    Ok(BufReader::new(file).lines())
}

fn index_file_lines(lines:Lines<BufReader<File>>, lines_storage: &mut HashMap<i32,String>){
    let mut line_counter = 1;
    for each_line in lines.map_while(Result::ok){
        lines_storage.insert(line_counter,each_line);
        line_counter += 1;
    }
}

fn tokenize_words(lines_storage: & HashMap<i32,String>, token_storage : &mut HashMap<String,HashSet<i32>>){

    for (key,lines) in lines_storage{
        let chucked_words:Vec<_> = lines.split(" ")
           .collect();

        for word in &chucked_words{
            //check if it doesn't exist then it insert it
            let Some(line_indexes) = token_storage.get(&word.to_string()) else{
                token_storage.entry(word.to_string())
                    .or_insert_with(HashSet::new)
                    .insert(*key);
                continue;
            };

            // when it exists we clone and add the new data then replace it
            let mut prev_indexes = line_indexes.clone();
            prev_indexes.insert(*key);

            token_storage.insert(word.to_string(),prev_indexes);
        }
    }
}

fn get_query_match(
    user_input: &str,
    line_storage:& HashMap<i32,String>,
    token_storage:& HashMap<String,HashSet<i32>>,
    retrievable_lines: &mut HashMap<i32, String>
){

    let Some(query_match) = token_storage.get(user_input) else{
        return;
    };

    for line_number in query_match{
        let Some(line) = line_storage.get(line_number) else {
            return;
        };
        
        if let None = retrievable_lines.get(line_number){
            retrievable_lines.insert(*line_number, line.clone());
        }
        
    }

}



fn main(){

    let user_command: Vec<_> = args().collect();
    let processed_command = Config::new(&user_command);
    let mut line_storage: HashMap<i32,String> = HashMap::new();
    let mut token_storage: HashMap<String,HashSet<i32>> = HashMap::new();
    let mut retrievable_lines: HashMap<i32,String> = HashMap::new();

    let Ok(processed_command) = processed_command else{
        println!("{:?}",processed_command.err());
        return;
    };



   

    if let Ok(lines) = open_and_read_file(&processed_command.file_path){
       index_file_lines(lines,&mut line_storage);
       println!("indexing done>>");
    }
    tokenize_words(&line_storage, &mut token_storage);
    
    for query in processed_command.search_query{
        get_query_match(query,&line_storage,&token_storage,& mut retrievable_lines);
    }
    
    if retrievable_lines.is_empty(){
        println!("no result found....");
        return;
    }
    
    for (line_number,text) in retrievable_lines{
        println!("matching found @ {}  : {}",line_number,text)
    }
    
}