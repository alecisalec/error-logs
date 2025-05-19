
use std::{fs};




fn extract_errors(log: &str) -> Vec<String> {
      let vec_strings=log.split("\n");
      let mut ret_vec: Vec<String> = vec![];
      for i in vec_strings {
         if i.starts_with("ERROR"){
            println!("{}", i);
            ret_vec.push(i.to_string());
         }
         else{
            println!("didnt match");
         }
         
      }
      return ret_vec;
   }
 
fn main() {


   let text = fs::read_to_string("logs.txt").expect("failed to read file");

   let error_logs = extract_errors(&text.as_str());

   fs::write("errors.txt", error_logs.join("\n")).expect("there was an error");
   
   // let mut err_logs = vec![];

   // match fs::read_to_string("logs.txt") {
   //    Ok(text) =>{
   //       err_logs= extract_errors(text.as_str());
         
   //       }
   //    Err(err) =>{
   //       println!("failed to find: {}", err);
   //    }
   // }
   // println!("{:#?}", err_logs);
   



}