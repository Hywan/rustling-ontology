extern crate unicode_normalization;
extern crate unicode_categories;
extern crate rustling;
extern crate rustling_ontology_moment as moment;
extern crate rustling_ontology_values as values;
extern crate regex;
use std::result;

#[macro_use]
mod macros;
mod preprocessing;

pub mod de;
pub mod en;
pub mod es;
pub mod fr;
pub mod ko;

pub use preprocessing::PreprocessingOptions;

macro_rules! lang_enum {
    ([$($lang:ident),*]) => {
        /// Enumerates all language supported for the general purpose ontology.
        #[derive(Copy,Clone,Debug)]
        pub enum Lang {
            $( $lang, )*
        }

        impl Lang {
            pub fn all() -> Vec<Lang> {
                vec![
                    $( Lang::$lang, )*
                ]
            }
        }
    }
}

lang_enum!([DE, EN, ES, FR, KO]);

impl std::str::FromStr for Lang {
    type Err = String;
    fn from_str(it: &str) -> result::Result<Lang, Self::Err> {
        match &*it.to_lowercase() {
            "en" => Ok(Lang::EN),
            "fr" => Ok(Lang::FR),
            "es" => Ok(Lang::ES),
            "ko" => Ok(Lang::KO),
            "de" => Ok(Lang::DE),
            _ => Err(format!("Unknown language {}", it)),
        }
    }
}

impl ::std::string::ToString for Lang {
    fn to_string(&self) -> String {
        match self {
            &Lang::EN => "en".to_string(),
            &Lang::FR => "fr".to_string(),
            &Lang::ES => "es".to_string(),
            &Lang::KO => "ko".to_string(),
            &Lang::DE => "de".to_string(),
        }
    }
}

macro_rules! lang {
    ($lang:ident, $config:ident, $boundaries_checker:ident, [$($rule:ident),*], [$($dim:ident),*], [$($option:ident),*]) => {
        pub mod $config {
            use values;
            use $lang;
            use preprocessing;
            
            pub fn rule_set() -> ::rustling::RustlingResult<::rustling::RuleSet<values::Dimension, preprocessing::PreprocessingOptions>> {
                let mut b = ::rustling::RuleSetBuilder::new(::rustling::BoundariesChecker::$boundaries_checker);
                $( $lang::$rule(&mut b)?; )*
                let mut options = vec![];
                $( options.push(preprocessing::PreprocessingOption::$option); )*
                Ok(b.build_with(preprocessing::PreprocessingOptions::new(options)))
            }

            pub fn dims() -> Vec<values::DimensionKind> {
                let mut dims = vec![];
                $( dims.push(values::DimensionKind::$dim); )*
                dims
            }
        }
    }
}

/// Obtain rules for a given language.
pub fn rules(lang: Lang) -> ::rustling::RustlingResult<::rustling::RuleSet<values::Dimension, ::preprocessing::PreprocessingOptions>> {
    match lang {
        Lang::DE => de_config::rule_set(),
        Lang::EN => en_config::rule_set(),
        Lang::ES => es_config::rule_set(),
        Lang::FR => fr_config::rule_set(),
        Lang::KO => ko_config::rule_set(),
    }
}

/// Obtain dimensions for a given language.
pub fn dims(lang: Lang) -> Vec<values::DimensionKind> {
    match lang {
        Lang::DE => de_config::dims(),
        Lang::EN => en_config::dims(),
        Lang::ES => es_config::dims(),
        Lang::FR => fr_config::dims(),
        Lang::KO => ko_config::dims(),
    }
}

lang!(de, de_config, ComposedWordOrDetailed, [rules_numbers, rules_time, rules_cycle, rules_duration, rules_temperature, rules_finance], 
          [Number, Ordinal, Time, Duration, Temperature, AmountOfMoney],
          [RemoveDiacritics, Lowercase]);
lang!(en, en_config, Detailed, [rules_numbers, rules_time, rules_cycle, rules_duration, rules_temperature, rules_finance], 
          [Number, Ordinal, Time, Duration, Temperature, AmountOfMoney],
          [RemoveDiacritics, Lowercase]);
lang!(es, es_config, Detailed, [rules_numbers, rules_temperature, rules_cycle, rules_duration, rules_time],
          [Number, Ordinal, Time, Duration, Temperature],
          [RemoveDiacritics, Lowercase]);
lang!(fr, fr_config, Detailed, [rules_numbers, rules_time, rules_temperature, rules_cycle, rules_duration],
          [Number, Ordinal, Time, Duration, Temperature],
          [RemoveDiacritics, Lowercase]);
lang!(ko, ko_config, Detailed, [rules_numbers, rule_time, rule_temperature, rules_finance, rules_cycle, rules_duration], 
          [Number, Ordinal, Time, Duration, Temperature, AmountOfMoney],
          [RemoveDiacritics, Lowercase]);
