use std::thread;
use std::time::Duration;
use reqwest;
use reqwest::Response;
use regex::{Regex, Captures};
use std::env;
use std::fs;
struct Patent {
    title: String, 
    date: String,
     number: i64,
    }

#[tokio::main]
async fn main() {
    let mut highest: i64 = 0;
    let mut lowest_patent_num: i64 = 0;  
    let farming_words1: String = fs::read_to_string
    ("/Users/judepackard-jones/Desktop/Programming/Rust/Patent-relational-mapper/Project assets/FarmingQueryWords1.txt")
    .expect("Error reading file 1");
    let farming_words_2: String = fs::read_to_string
    ("/Users/judepackard-jones/Desktop/Programming/Rust/Patent-relational-mapper/Project assets/FarmingQueryWords2.txt")
    .expect("Error reading file 2");
    let mut farming_words: &String;
    let mut loop_counter: i8 = 0;
    let mut patents: Vec<Patent> = Vec::new();
    let mut patent_temp_list: Vec<Patent> = Vec::new();
    loop {
        patent_temp_list.clear();
    match loop_counter {
        0 => {
            loop_counter = 1;
            farming_words = &farming_words1;
        }
        1 => {
            loop_counter = 0;
            farming_words = &farming_words_2;
        }
        _ => {
            panic!("Loop counter set to non-valid value.");
        }
    }
    let mut query = String::from(format!(r#"https://api.patentsview.org/patents/query?q={{"_and":[{{"_gt":{{"patent_number":"{lowest_patent_num}"}}}},{{"_text_any":{{"patent_title":"{farming_words}"}}}},{{"_text_any":{{"patent_abstract":"{farming_words}"}}}}]}}&f=["patent_title","patent_date","patent_number"]"#));
    let resp: Response = reqwest::get(&query).await.unwrap();
    let body = resp.text().await.unwrap();
    //println!("{}", body);
    thread::sleep(Duration::from_secs_f32(1.3));
    (patent_temp_list, highest) = format_patent(body);
    patents.append(&mut patent_temp_list);
    for pat in &patents {
        println!("***{}***{}***{}***", pat.title, pat.date, pat.number);
    }
    if loop_counter == 1 {
        if highest > lowest_patent_num {lowest_patent_num = highest;}
    }
    }

}


fn format_patent(patents: String) -> (Vec<Patent>, i64){
    let mut highest: i64 = 0;
    let mut parsed_patent: Vec<Patent> = Vec::new();
    let re_over = Regex::new(r#"(\{"patent_title"[^}]*\})"#).unwrap();
    let mut patent_captures = vec![];
    for (_, [pat]) in re_over.captures_iter(patents.as_str()).map(|c| c.extract()) {
        patent_captures.push(pat);
    }
    
    let re_each = Regex::new(r#""patent_title":"(.*?)","patent_date":"(.*?)","patent_number":"(.*?)""#).unwrap();
    let mut captures_in_vec: Vec<String> = Vec::new();
    for e in patent_captures{
        for capture in re_each.captures_iter(e) {
            // println!("*{}", e);
            captures_in_vec.clear();
            for i in 1..4{
         if let Some(value) = capture.get(i) {
            captures_in_vec.push(value.as_str().to_string());
            if i == 3 {
                let temp_val = value.as_str().to_string().parse::<i64>().unwrap();
                if temp_val > 0 {highest = temp_val}
            }
            }
        }
    }
    parsed_patent.push(Patent {
        title: captures_in_vec.get(0).unwrap_or(&"".to_string()).clone(),
        date: captures_in_vec.get(1).unwrap_or(&"".to_string()).clone(),
        number: captures_in_vec.get(2)
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(0), 
    });
    }
    println!("Highest is: {}", highest);
    (parsed_patent, highest)
}