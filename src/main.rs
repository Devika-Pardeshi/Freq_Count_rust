use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use unicode_segmentation::UnicodeSegmentation;
extern crate unicode_segmentation;

mod language_lookup;
//use language_lookup::*;

#[derive(Debug, Clone)]
pub struct InputDetails{
    pub dir_path : String,

}

//re-iterate_folder
#[derive(Debug, Clone)]
pub struct AllResult{
    pub all_language_freq_count_map :HashMap<String,LanguageResult>,
}
//re-iterate_file
#[derive(Debug, Clone)]
pub struct LanguageResult{   
    pub frequency_count_result : Vec<FileFrequencyCountResult>,
}

//re--filepath //getfreq
#[derive(Debug, Clone)]
pub struct FileFrequencyCountResult{
    pub grapheme_freq_count_map : HashMap<String,u32>,
}

//iterate_folder
#[derive(Debug, Clone)]
pub struct AllFrequencyCountCalculator{
   pub input_detail :InputDetails,
}

//iterate_file
#[derive(Debug, Clone)]
pub struct FrequencyCountForLanguageCalculator{
   pub language_input_details : InputDetails,
}

//filepath //getfreq
#[derive(Debug, Clone)]
pub struct FileFreqCountCalculator{
    pub file_input_path_details : InputDetails,
}

impl LanguageResult{
fn get_all_merge_result(&self) -> HashMap<String,u32>{
    let mut result: HashMap<String,u32>=HashMap::new();
     
    for freq_count_result in self.frequency_count_result.iter(){
       for grapheme_result in freq_count_result.grapheme_freq_count_map.iter(){       
          *result.entry(grapheme_result.0.to_string().to_owned()).or_insert(0) += grapheme_result.1;
       }       
    }

    return result;
}

}

impl AllFrequencyCountCalculator{
    fn calculate(&self) -> AllResult{
        let result: HashMap<String,LanguageResult>=HashMap::new();
        let mut all_result = AllResult{all_language_freq_count_map:result};
        let dir_languages = language_lookup :: language_data();
        let dir_path = self.input_detail.dir_path.clone();

        for file in fs::read_dir(dir_path).unwrap() {
            let file_ph = format!("{}", file.as_ref().unwrap().path().display());
            let file_nm = format!("{}", file.as_ref().unwrap().file_name().to_str().unwrap());

            let m = file.as_ref().unwrap().metadata().unwrap();
                if m.is_dir()
                    {
                        if dir_languages.contains_key(&file_nm) {
                            let input_details = InputDetails{dir_path:file_ph};
                            let  lang_freq_count_calculator =  FrequencyCountForLanguageCalculator{language_input_details:input_details};
                            let lang_freq_count_result = lang_freq_count_calculator.calculate();
                            all_result.all_language_freq_count_map.insert(file_nm, lang_freq_count_result);

                    }else{
                        println!("folder not found")
                    }
                }
        }
      return all_result;
    }
}

impl FrequencyCountForLanguageCalculator {
    fn calculate(&self) -> LanguageResult{
      //  let mut files_frequency: HashMap<String, u32> = HashMap::new();
        
        let frequency_count_result_list = Vec::<FileFrequencyCountResult>::new();
        let mut lang_result = LanguageResult{frequency_count_result:frequency_count_result_list};
        let dir_path = self.language_input_details.dir_path.clone();

        for file in fs::read_dir(dir_path).unwrap() {
            let file_ph = format!("{}", file.as_ref().unwrap().path().display());
            let m = file.as_ref().unwrap().metadata().unwrap();

            if m.is_file() {
                let input_details = InputDetails{dir_path:file_ph};
                let file_freq_count_calculator = FileFreqCountCalculator{file_input_path_details:input_details};
                let file_freq_result = file_freq_count_calculator.calculate();

                lang_result.frequency_count_result.push(file_freq_result);
               
            } else {
                let fc = self.calculate();
                for file_freq_result in fc.frequency_count_result{
                    lang_result.frequency_count_result.push(file_freq_result);
                }

            }
        }
        return lang_result;
    }
}

impl FileFreqCountCalculator{
    fn calculate(&self) -> FileFrequencyCountResult{
        let file_path = self.file_input_path_details.dir_path.clone();

        let mut frequency: HashMap<String, u32> = HashMap::new();

        let f = BufReader::new(File::open(file_path).expect("open failed"));
        for line in f.lines() {
            let line_str = line.unwrap();
            let g = UnicodeSegmentation::graphemes(line_str.as_str(), true).collect::<Vec<&str>>();
            for s in g {
                 *frequency.entry(s.to_owned()).or_insert(0) += 1;
            }
        }

        let file_freq_count_result = FileFrequencyCountResult{grapheme_freq_count_map:frequency.clone()};
       
        return file_freq_count_result;
    }
}

fn main() {

   let input_details = InputDetails{dir_path:"/workspace/Sample".to_string()};
   let all_freq_count_calculator =  AllFrequencyCountCalculator{input_detail:input_details};
   let all_result  = all_freq_count_calculator.calculate();
   
   for lang_result in all_result.all_language_freq_count_map{
      let fc = lang_result.1.get_all_merge_result();
      println!("Lang : {:?}",lang_result.0);
      println!("Freq Count : {:?}",fc);
   }

}

