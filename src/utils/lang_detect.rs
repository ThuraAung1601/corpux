// Ref: Kasitphoom Thowongs, "What Language is that?", Link: https://github.com/Kasitphoom/ELEMENTARY_SYSTEM_PROGRAMMING/blob/master/22.10.11/WhatLanguage_report_65011328.pdf
// Modifications of P' Phoom's program made for the project:
    // [1] Refactor as a utility function
    // [2] Added 50 more languages + 1 for Unknown case
    // [3] Solved panic cases for Unknown characters
    // [4] Calculated the total numbers as well as the percentage of characters for each language included
    // [5] Passed the information as a struct instead of printing out the results
    // [6] Added unit test to verify the language detection function

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Lang {
    English, Spanish, French, German, ChineseSimplified,
    Japanese, Russian, Arabic, Portuguese, Italian, Dutch, Swedish, Korean,
    Turkish, Greek, Hindi, Vietnamese, Finnish, Norwegian, Danish,
    Romanian, Polish, Hungarian, Hebrew, Thai, Czech,
    Serbian, Filipino, Ukrainian, Malay, Bengali,
    Swahili, Persian, Afrikaans, Bulgarian, Croatian, Slovenian, Estonian,
    Icelandic, Kurdish, Urdu, Tamil, PunjabiGurmukhi,
    Kannada, Amharic, Nepali, Yoruba, Zulu, Uzbek, Sinhala, Belarusian,
    Azerbaijani, Javanese, Burmese, AccentedLatin, Telugu, Malayalam,
    Unknown, Ascii,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LangType {
    lang: Lang,
    uniup: u64,
    unidown: u64,
    text: String,
}

#[derive(Debug)]
pub struct LangInfo {
    pub lang: String,
    pub total_character: usize,
    pub percentage: f64,
}

pub fn find_unicode(langtype: [LangType; 59], c: char) -> Lang {
    let uni = c as u64;

    for i in 0..58 {
        if uni <= langtype[i].uniup && uni >= langtype[i].unidown {
            return langtype[i].lang;
        }
    }
    
    Lang::Unknown
}

pub fn return_lang(lang: Lang) -> String {
    match lang {
        Lang::English => "English".to_string(),
        Lang::Spanish => "Spanish".to_string(),
        Lang::French => "French".to_string(),
        Lang::German => "German".to_string(),
        Lang::ChineseSimplified => "Chinese".to_string(),
        Lang::Japanese => "Japanese".to_string(),
        Lang::Russian => "Russian".to_string(),
        Lang::Arabic => "Arabic".to_string(),
        Lang::Portuguese => "Portuguese".to_string(),
        Lang::Italian => "Italian".to_string(),
        Lang::Dutch => "Dutch".to_string(),
        Lang::Swedish => "Swedish".to_string(),
        Lang::Korean => "Korean".to_string(),
        Lang::Turkish => "Turkish".to_string(),
        Lang::Greek => "Greek".to_string(),
        Lang::Hindi => "Hindi".to_string(),
        Lang::Vietnamese => "Vietnamese".to_string(),
        Lang::Finnish => "Finnish".to_string(),
        Lang::Norwegian => "Norwegian".to_string(),
        Lang::Danish => "Danish".to_string(),
        Lang::Romanian => "Romanian".to_string(),
        Lang::Polish => "Polish".to_string(),
        Lang::Hungarian => "Hungarian".to_string(),
        Lang::Hebrew => "Hebrew".to_string(),
        Lang::Thai => "Thai".to_string(),
        Lang::Czech => "Czech".to_string(),
        Lang::Serbian => "Serbian".to_string(),
        Lang::Filipino => "Filipino".to_string(),
        Lang::Ukrainian => "Ukrainian".to_string(),
        Lang::Malay => "Malay".to_string(),
        Lang::Bengali => "Bengali".to_string(),
        Lang::Swahili => "Swahili".to_string(),
        Lang::Persian => "Persian".to_string(),
        Lang::Afrikaans => "Afrikaans".to_string(),
        Lang::Bulgarian => "Bulgarian".to_string(),
        Lang::Croatian => "Croatian".to_string(),
        Lang::Slovenian => "Slovenian".to_string(),
        Lang::Estonian => "Estonian".to_string(),
        Lang::Icelandic => "Icelandic".to_string(),
        Lang::Kurdish => "Kurdish".to_string(),
        Lang::Urdu => "Urdu".to_string(),
        Lang::Tamil => "Tamil".to_string(),
        Lang::PunjabiGurmukhi => "Punjabi".to_string(),
        Lang::Kannada => "Kannada".to_string(),
        Lang::Amharic => "Amharic".to_string(),
        Lang::Nepali => "Nepali".to_string(),
        Lang::Yoruba => "Yoruba".to_string(),
        Lang::Zulu => "Zulu".to_string(),
        Lang::Uzbek => "Uzbek".to_string(),
        Lang::Sinhala => "Sinhala".to_string(),
        Lang::Belarusian => "Belarusian".to_string(),
        Lang::Azerbaijani => "Azerbaijani".to_string(),
        Lang::Javanese => "Javanese".to_string(),
        Lang::AccentedLatin => "Accented Latin".to_string(),
        Lang::Burmese => "Burmese".to_string(),
        Lang::Telugu => "Telugu".to_string(),
        Lang::Malayalam => "Malayalam".to_string(),
        Lang::Ascii => "ASCII".to_string(),
        Lang::Unknown => "Unknown".to_string(),
    }    
}

pub fn lang_detect(lines: Vec<String>) -> Vec<LangInfo> {
    // make an array of struct for each language
    let langtype = [
        LangType{lang: Lang::English, unidown: 0x0041, uniup: 0x007A, text: "English".to_string()},
        LangType{lang: Lang::Spanish, unidown: 0x00C0, uniup: 0x00FF, text: "Spanish".to_string()},
        LangType{lang: Lang::French, unidown: 0x00C0, uniup: 0x017F, text: "French".to_string()},
        LangType{lang: Lang::German, unidown: 0x00C4, uniup: 0x00FC, text: "German".to_string()},
        LangType{lang: Lang::ChineseSimplified, unidown: 0x4E00, uniup: 0x9FFF, text: "Chinese".to_string()},
        LangType{lang: Lang::Japanese, unidown: 0x3040, uniup: 0x309F, text: "Japanese".to_string()},
        LangType{lang: Lang::Russian, unidown: 0x0410, uniup: 0x044F, text: "Russian".to_string()},
        LangType{lang: Lang::Arabic, unidown: 0x0627, uniup: 0x064A, text: "Arabic".to_string()},
        LangType{lang: Lang::Portuguese, unidown: 0x00C0, uniup: 0x00FF, text: "Portuguese".to_string()},
        LangType{lang: Lang::Italian, unidown: 0x00C0, uniup: 0x017E, text: "Italian".to_string()},
        LangType{lang: Lang::Dutch, unidown: 0x00C0, uniup: 0x017E, text: "Dutch".to_string()},
        LangType{lang: Lang::Swedish, unidown: 0x00C0, uniup: 0x00E5, text: "Swedish".to_string()},
        LangType{lang: Lang::Korean, unidown: 0xAC00, uniup: 0xD7AF, text: "Korean".to_string()},
        LangType{lang: Lang::Turkish, unidown: 0x0041, uniup: 0x007A, text: "Turkish".to_string()},
        LangType{lang: Lang::Greek, unidown: 0x0391, uniup: 0x03C9, text: "Greek".to_string()},
        LangType{lang: Lang::Hindi, unidown: 0x0901, uniup: 0x097F, text: "Hindi".to_string()},
        LangType{lang: Lang::Vietnamese, unidown: 0x1E00, uniup: 0x1EFF, text: "Vietnamese".to_string()},
        LangType{lang: Lang::Finnish, unidown: 0x00C4, uniup: 0x00E4, text: "Finnish".to_string()},
        LangType{lang: Lang::Norwegian, unidown: 0x00C0, uniup: 0x00E5, text: "Norwegian".to_string()},
        LangType{lang: Lang::Danish, unidown: 0x00C0, uniup: 0x00E5, text: "Danish".to_string()},
        LangType{lang: Lang::Romanian, unidown: 0x0100, uniup: 0x021B, text: "Romanian".to_string()},
        LangType{lang: Lang::Polish, unidown: 0x0104, uniup: 0x017C, text: "Polish".to_string()},
        LangType{lang: Lang::Hungarian, unidown: 0x00C1, uniup: 0x0170, text: "Hungarian".to_string()},
        LangType{lang: Lang::Hebrew, unidown: 0x05D0, uniup: 0x05EA, text: "Hebrew".to_string()},
        LangType{lang: Lang::Thai, unidown: 0x0E01, uniup: 0x0E5B, text: "Thai".to_string()},
        LangType{lang: Lang::Czech, unidown: 0x0100, uniup: 0x01B6, text: "Czech".to_string()},
        LangType{lang: Lang::Serbian, unidown: 0x0410, uniup: 0x045F, text: "Serbian".to_string()},
        LangType{lang: Lang::Filipino, unidown: 0x0041, uniup: 0x007A, text: "Filipino".to_string()},
        LangType{lang: Lang::Ukrainian, unidown: 0x0410, uniup: 0x045F, text: "Ukrainian".to_string()},
        LangType{lang: Lang::Malay, unidown: 0x0041, uniup: 0x007A, text: "Malay".to_string()},
        LangType{lang: Lang::Bengali, unidown: 0x0981, uniup: 0x09FF, text: "Bengali".to_string()},
        LangType{lang: Lang::Swahili, unidown: 0x0061, uniup: 0x007A, text: "Swahili".to_string()},
        LangType{lang: Lang::Persian, unidown: 0x0621, uniup: 0x064A, text: "Persian".to_string()},
        LangType{lang: Lang::Afrikaans, unidown: 0x0041, uniup: 0x007A, text: "Afrikaans".to_string()},
        LangType{lang: Lang::Bulgarian, unidown: 0x0410, uniup: 0x044F, text: "Bulgarian".to_string()},
        LangType{lang: Lang::Croatian, unidown: 0x0100, uniup: 0x017E, text: "Croatian".to_string()},
        LangType{lang: Lang::Slovenian, unidown: 0x0100, uniup: 0x017E, text: "Slovenian".to_string()},
        LangType{lang: Lang::Estonian, unidown: 0x00C0, uniup: 0x017E, text: "Estonian".to_string()},
        LangType{lang: Lang::Icelandic, unidown: 0x0041, uniup: 0x00F6, text: "Icelandic".to_string()},
        LangType{lang: Lang::Kurdish, unidown: 0x0626, uniup: 0x06D5, text: "Kurdish".to_string()},
        LangType{lang: Lang::Urdu, unidown: 0x0601, uniup: 0x06D6, text: "Urdu".to_string()},
        LangType{lang: Lang::Tamil, unidown: 0x0B80, uniup: 0x0BFF, text: "Tamil".to_string()},
        LangType{lang: Lang::PunjabiGurmukhi, unidown: 0x0A01, uniup: 0x0A76, text: "Punjabi".to_string()},
        LangType{lang: Lang::Kannada, unidown: 0x0C80, uniup: 0x0CFF, text: "Kannada".to_string()},
        LangType{lang: Lang::Amharic, unidown: 0x1200, uniup: 0x137F, text: "Amharic".to_string()},
        LangType{lang: Lang::Nepali, unidown: 0x0901, uniup: 0x097F, text: "Nepali".to_string()},
        LangType{lang: Lang::Yoruba, unidown: 0x0180, uniup: 0x024F, text: "Yoruba".to_string()},
        LangType{lang: Lang::Zulu, unidown: 0x0180, uniup: 0x024F, text: "Zulu".to_string()},
        LangType{lang: Lang::Uzbek, unidown: 0x0400, uniup: 0x04F9, text: "Uzbek".to_string()},
        LangType{lang: Lang::Sinhala, unidown: 0x0D80, uniup: 0x0DFF, text: "Sinhala".to_string()},
        LangType{lang: Lang::Belarusian, unidown: 0x0410, uniup: 0x044F, text: "Belarusian".to_string()},
        LangType{lang: Lang::Azerbaijani, unidown: 0x018F, uniup: 0x019B, text: "Azerbaijani".to_string()},
        LangType{lang: Lang::Javanese, unidown: 0xA980, uniup: 0xA9DF, text: "Javanese".to_string()},
        LangType{lang: Lang::AccentedLatin, unidown: 0x00C0, uniup: 0x00FF, text: "Accented Latin".to_string()},
        LangType{lang: Lang::Burmese, unidown: 0x1000, uniup: 0x109F, text: "Burmese".to_string()},
        LangType{lang: Lang::Telugu, unidown: 0x0C00, uniup: 0x0C7F, text: "Telugu".to_string()},
        LangType{lang: Lang::Malayalam, unidown: 0x0D00, uniup: 0x0D7F, text: "Malayalam".to_string()},
        LangType{lang: Lang::Ascii, unidown: 0x0020, uniup: 0x007E, text: "ASCII".to_string()},
        LangType{lang: Lang::Unknown, unidown: 0x0000, uniup: 0x0000, text: "Unknown".to_string()},
    ];

    let mut total_words = 0;
    let mut language_counts: HashMap<Lang, HashMap<Lang, usize>> = HashMap::new();

    for line in lines {
        // Split the line into words
        let words: Vec<&str> = line.split_whitespace().collect();

        for word in words {
            total_words += 1;

            let mut word_language_counts: HashMap<Lang, usize> = HashMap::new();

            for c in word.chars() {
                let lang = find_unicode(langtype.clone(), c);
                let count = word_language_counts.entry(lang).or_insert(0);
                *count += 1;
            }

            for (lang, count) in word_language_counts.iter() {
                let language_count = language_counts.entry(*lang).or_insert(HashMap::new());
                let word_count = language_count.entry(*lang).or_insert(0);
                *word_count += count;
            }
        }
    }

    // let mut lang_info = String::new();
    let mut lang_info = Vec::new();
    for (lang, word_counts) in language_counts.iter() {
        let total_count: usize = word_counts.values().sum();
    
        // Calculate the unadjusted percentage
        let unadjusted_percentage = (total_count as f64 / total_words as f64) * 100.0;
    
        lang_info.push(LangInfo {
            lang: return_lang(*lang),
            total_character: total_count,
            percentage: unadjusted_percentage,
        });
    }
    
    // Calculate the total unadjusted percentage
    let total_unadjusted_percentage: f64 = lang_info.iter().map(|info| info.percentage).sum();
    
    // Adjust percentages to ensure they sum up to 100.00%
    for info in &mut lang_info {
        info.percentage = (info.percentage / total_unadjusted_percentage) * 100.0;
    }
    
    // If total_unadjusted_percentage is zero, adjust all percentages to be equal
    if total_unadjusted_percentage == 0.0 {
        let equal_percentage = 100.0 / lang_info.len() as f64;
        for info in &mut lang_info {
            info.percentage = equal_percentage;
        }
    }
    
    lang_info
}

#[test]
fn test_lang_detect() {
    let input_lines = vec![
        "This is a test sentence in English".to_string()
    ];

    let lang_info = lang_detect(input_lines);

    assert_eq!(lang_info.len(), 1);
    let english_info = &lang_info[0];

    assert_eq!(english_info.lang, "English");
    assert_eq!(english_info.total_character, 28);
}
