#[allow(dead_code)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub struct Lang<'a> {
    pub code: &'a str,
    pub label: &'a str,
}
// pub const LANGS: [&str; 3] = ["ar", "en", "ja"];
// pub const LANGS: [Lang; 109] = [
//     // Lang { code: "", label: "(Auto detect)" },
//     Lang {
//         code: "af",
//         label: "Afrikaans",
//     },
//     Lang {
//         code: "sq",
//         label: "Albanian",
//     },
//     Lang {
//         code: "am",
//         label: "Amharic",
//     },
//     Lang {
//         code: "ar",
//         label: "Arabic",
//     },
//     Lang {
//         code: "hy",
//         label: "Armenian",
//     },
//     Lang {
//         code: "az",
//         label: "Azerbaijani",
//     },
//     Lang {
//         code: "eu",
//         label: "Basque",
//     },
//     Lang {
//         code: "be",
//         label: "Belarusian",
//     },
//     Lang {
//         code: "bn",
//         label: "Bengali",
//     },
//     Lang {
//         code: "bs",
//         label: "Bosnian",
//     },
//     Lang {
//         code: "bg",
//         label: "Bulgarian",
//     },
//     Lang {
//         code: "ca",
//         label: "Catalan",
//     },
//     Lang {
//         code: "ceb",
//         label: "Cebuano",
//     },
//     Lang {
//         code: "zh-CN",
//         label: "Chinese (China)",
//     },
//     Lang {
//         code: "zh-TW",
//         label: "Chinese (Taiwan)",
//     },
//     Lang {
//         code: "co",
//         label: "Corsican",
//     },
//     Lang {
//         code: "hr",
//         label: "Croatian",
//     },
//     Lang {
//         code: "cs",
//         label: "Czech",
//     },
//     Lang {
//         code: "da",
//         label: "Danish",
//     },
//     Lang {
//         code: "nl",
//         label: "Dutch",
//     },
//     Lang {
//         code: "en",
//         label: "English",
//     },
//     Lang {
//         code: "eo",
//         label: "Esperanto",
//     },
//     Lang {
//         code: "et",
//         label: "Estonian",
//     },
//     Lang {
//         code: "fi",
//         label: "Finnish",
//     },
//     Lang {
//         code: "fr",
//         label: "French",
//     },
//     Lang {
//         code: "fy",
//         label: "Frisian",
//     },
//     Lang {
//         code: "gl",
//         label: "Galician",
//     },
//     Lang {
//         code: "ka",
//         label: "Georgian",
//     },
//     Lang {
//         code: "de",
//         label: "German",
//     },
//     Lang {
//         code: "el",
//         label: "Greek",
//     },
//     Lang {
//         code: "gu",
//         label: "Gujarati",
//     },
//     Lang {
//         code: "Creole",
//         label: "Haitian",
//     },
//     Lang {
//         code: "ha",
//         label: "Hausa",
//     },
//     Lang {
//         code: "haw",
//         label: "Hawaiian",
//     },
//     Lang {
//         code: "he",
//         label: "Hebrew",
//     },
//     Lang {
//         code: "hi",
//         label: "Hindi",
//     },
//     Lang {
//         code: "hmn",
//         label: "Hmong",
//     },
//     Lang {
//         code: "hu",
//         label: "Hungarian",
//     },
//     Lang {
//         code: "is",
//         label: "Icelandic",
//     },
//     Lang {
//         code: "ig",
//         label: "Igbo",
//     },
//     Lang {
//         code: "id",
//         label: "Indonesian",
//     },
//     Lang {
//         code: "ga",
//         label: "Irish",
//     },
//     Lang {
//         code: "it",
//         label: "Italian",
//     },
//     Lang {
//         code: "ja",
//         label: "Japanese",
//     },
//     Lang {
//         code: "jv",
//         label: "Javanese",
//     },
//     Lang {
//         code: "kn",
//         label: "Kannada",
//     },
//     Lang {
//         code: "kk",
//         label: "Kazakh",
//     },
//     Lang {
//         code: "km",
//         label: "Khmer",
//     },
//     Lang {
//         code: "rw",
//         label: "Kinyarwanda",
//     },
//     Lang {
//         code: "ko",
//         label: "Korean",
//     },
//     Lang {
//         code: "ku",
//         label: "Kurdish",
//     },
//     Lang {
//         code: "ky",
//         label: "Kyrgyz",
//     },
//     Lang {
//         code: "lo",
//         label: "Lao",
//     },
//     Lang {
//         code: "la",
//         label: "Latin",
//     },
//     Lang {
//         code: "lv",
//         label: "Latvian",
//     },
//     Lang {
//         code: "lt",
//         label: "Lithuanian",
//     },
//     Lang {
//         code: "lb",
//         label: "Luxembourgish",
//     },
//     Lang {
//         code: "mk",
//         label: "Macedonian",
//     },
//     Lang {
//         code: "mg",
//         label: "Malagasy",
//     },
//     Lang {
//         code: "ms",
//         label: "Malay",
//     },
//     Lang {
//         code: "ml",
//         label: "Malayalam",
//     },
//     Lang {
//         code: "mt",
//         label: "Maltese",
//     },
//     Lang {
//         code: "mi",
//         label: "Maori",
//     },
//     Lang {
//         code: "mr",
//         label: "Marathi",
//     },
//     Lang {
//         code: "mn",
//         label: "Mongolian",
//     },
//     Lang {
//         code: "my",
//         label: "Myanmar",
//     },
//     Lang {
//         code: "ne",
//         label: "Nepali",
//     },
//     Lang {
//         code: "no",
//         label: "Norwegian",
//     },
//     Lang {
//         code: "ny",
//         label: "Nyanja",
//     },
//     Lang {
//         code: "or",
//         label: "Odia",
//     },
//     Lang {
//         code: "ps",
//         label: "Pashto",
//     },
//     Lang {
//         code: "fa",
//         label: "Persian",
//     },
//     Lang {
//         code: "pl",
//         label: "Polish",
//     },
//     Lang {
//         code: "pt",
//         label: "Portuguese",
//     },
//     Lang {
//         code: "pa",
//         label: "Punjabi",
//     },
//     Lang {
//         code: "ro",
//         label: "Romanian",
//     },
//     Lang {
//         code: "ru",
//         label: "Russian",
//     },
//     Lang {
//         code: "sm",
//         label: "Samoan",
//     },
//     Lang {
//         code: "Gaelic",
//         label: "Scots",
//     },
//     Lang {
//         code: "sr",
//         label: "Serbian",
//     },
//     Lang {
//         code: "st",
//         label: "Sesotho",
//     },
//     Lang {
//         code: "sn",
//         label: "Shona",
//     },
//     Lang {
//         code: "sd",
//         label: "Sindhi",
//     },
//     Lang {
//         code: "si",
//         label: "Sinhala",
//     },
//     Lang {
//         code: "sk",
//         label: "Slovak",
//     },
//     Lang {
//         code: "sl",
//         label: "Slovenian",
//     },
//     Lang {
//         code: "so",
//         label: "Somali",
//     },
//     Lang {
//         code: "es",
//         label: "Spanish",
//     },
//     Lang {
//         code: "su",
//         label: "Sundanese",
//     },
//     Lang {
//         code: "sw",
//         label: "Swahili",
//     },
//     Lang {
//         code: "sv",
//         label: "Swedish",
//     },
//     Lang {
//         code: "tl",
//         label: "Tagalog",
//     },
//     Lang {
//         code: "tg",
//         label: "Tajik",
//     },
//     Lang {
//         code: "ta",
//         label: "Tamil",
//     },
//     Lang {
//         code: "tt",
//         label: "Tatar",
//     },
//     Lang {
//         code: "te",
//         label: "Telugu",
//     },
//     Lang {
//         code: "th",
//         label: "Thai",
//     },
//     Lang {
//         code: "tr",
//         label: "Turkish",
//     },
//     Lang {
//         code: "tk",
//         label: "Turkmen",
//     },
//     Lang {
//         code: "uk",
//         label: "Ukrainian",
//     },
//     Lang {
//         code: "ur",
//         label: "Urdu",
//     },
//     Lang {
//         code: "ug",
//         label: "Uyghur",
//     },
//     Lang {
//         code: "uz",
//         label: "Uzbek",
//     },
//     Lang {
//         code: "vi",
//         label: "Vietnamese",
//     },
//     Lang {
//         code: "cy",
//         label: "Welsh",
//     },
//     Lang {
//         code: "xh",
//         label: "Xhosa",
//     },
//     Lang {
//         code: "yi",
//         label: "Yiddish",
//     },
//     Lang {
//         code: "yo",
//         label: "Yoruba",
//     },
//     Lang {
//         code: "zu",
//         label: "Zulu",
//     },
// ];
pub const LANGS: [Lang; 24] = [
    // Lang { code: "", label: "(Auto detect)" },
    Lang {
        code: "bg",
        label: "Bulgarian",
    },
    Lang {
        code: "cs",
        label: "Czech",
    },
    Lang {
        code: "da",
        label: "Danish",
    },
    Lang {
        code: "de",
        label: "German",
    },
    Lang {
        code: "el",
        label: "Greek",
    },
    Lang {
        code: "en",
        label: "English",
    },
    Lang {
        code: "es",
        label: "Spanish",
    },
    Lang {
        code: "et",
        label: "Estonian",
    },
    Lang {
        code: "fi",
        label: "Finnish",
    },
    Lang {
        code: "fr",
        label: "French",
    },
    Lang {
        code: "hu",
        label: "Hungarian",
    },
    Lang {
        code: "it",
        label: "Italian",
    },
    Lang {
        code: "ja",
        label: "Japanese",
    },
    Lang {
        code: "lt",
        label: "Lithuanian",
    },
    Lang {
        code: "lv",
        label: "Latvian",
    },
    Lang {
        code: "nl",
        label: "Dutch",
    },
    Lang {
        code: "pl",
        label: "Polish",
    },
    Lang {
        code: "pt",
        label: "Portuguese",
    },
    Lang {
        code: "ro",
        label: "Romanian",
    },
    Lang {
        code: "ru",
        label: "Russian",
    },
    Lang {
        code: "sk",
        label: "Slovak",
    },
    Lang {
        code: "sl",
        label: "Slovenian",
    },
    Lang {
        code: "sv",
        label: "Swedish",
    },
    Lang {
        code: "zh",
        label: "Chinese",
    },
];
