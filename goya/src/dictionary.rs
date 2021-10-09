use super::char_class::CharDefinition;
use super::id::WordIdentifier;
use super::morpheme::Morpheme;

pub trait Dictionary {
    fn get(&self, wid: &WordIdentifier) -> Option<&Morpheme> {
        match wid {
            WordIdentifier::Known(wid, _) => self.get_known_morpheme(wid),
            WordIdentifier::Unknown(wid, _) => self.get_unknown_morpheme(wid),
        }
    }
    fn get_known_morpheme(&self, wid: &usize) -> Option<&Morpheme>;
    fn get_unknown_morpheme(&self, wid: &usize) -> Option<&Morpheme>;
    fn resolve_homonyms(&self, wid: &usize) -> Option<&Vec<usize>>;
    fn take_unknown_chars_seq(&self, def: &CharDefinition, text: &str, start: &usize) -> String;
    fn classify_char(&self, c: &char) -> &CharDefinition;
    fn get_unknown_morphemes_by_class(&self, class: &str) -> Vec<(usize, &Morpheme)>;
    fn transition_cost(&self, left: &usize, right: &usize) -> Option<&i16>;
    fn occurrence_cost(&self, wid: &usize) -> Option<i16>;
}
