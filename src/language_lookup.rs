use std::collections::HashMap;

pub fn language_data() -> HashMap<String,String>{

    let mut folder_language = HashMap::new();

    // folder_language.insert("/workspace/Sample/Marathi".to_string(), "/workspace/Sample/Marathi".to_string());
    // folder_language.insert("/workspace/Sample/English".to_string(), "/workspace/Sample/English".to_string());
    // folder_language.insert("/workspace/Sample/Korean".to_string(), "/workspace/Sample/Korean".to_string());

    folder_language.insert("Marathi".to_string(), "Marathi".to_string());
    folder_language.insert("Korean".to_string(), "Korean".to_string());
    folder_language.insert("English".to_string(), "English".to_string());
    folder_language.insert("Hindi".to_string(), "Hindi".to_string());

    return folder_language;
}