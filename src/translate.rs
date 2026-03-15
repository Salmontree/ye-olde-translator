// this is a vibe coded MESS because i really don't feel like spending all that time making this when i don't care about this project at all, good luck reading through it lol

// this is a vibe coded MESS because i really don't feel like spending all that time making this when i don't care about this project at all, good luck reading through it lol

// this is a vibe coded MESS because i really don't feel like spending all that time making this when i don't care about this project at all, good luck reading through it lol

// this is a vibe coded MESS because i really don't feel like spending all that time making this when i don't care about this project at all, good luck reading through it lol

// this is a vibe coded MESS because i really don't feel like spending all that time making this when i don't care about this project at all, good luck reading through it lol

// this is a vibe coded MESS because i really don't feel like spending all that time making this when i don't care about this project at all, good luck reading through it lol

// this is a vibe coded MESS because i really don't feel like spending all that time making this when i don't care about this project at all, good luck reading through it lol

// this is a vibe coded MESS because i really don't feel like spending all that time making this when i don't care about this project at all, good luck reading through it lol

// ─────────────────────────────────────────────────────────────────────────────
// 0. CONTRACTION EXPANDER
// ─────────────────────────────────────────────────────────────────────────────

/// Expands modern contractions to their full forms before any other processing.
/// This makes every later stage simpler because it only needs to handle
/// full words (e.g. "thou art" is easier to conjugate than "you're").
fn expand_contractions(input: &str) -> String {
    // Order matters: longer / more-specific patterns first.
    let expansions: &[(&str, &str)] = &[
        // ── you-contractions ──────────────────────────────────────────────
        ("you're",    "you are"),
        ("you've",    "you have"),
        ("you'll",    "you will"),
        ("you'd've",  "you would have"),
        ("you'd",     "you would"),
        // ── I-contractions ────────────────────────────────────────────────
        ("i'm",       "I am"),
        ("i've",      "I have"),
        ("i'll",      "I will"),
        ("i'd've",    "I would have"),
        ("i'd",       "I would"),
        // ── he/she/it ─────────────────────────────────────────────────────
        ("he's",      "he is"),
        ("she's",     "she is"),
        ("it's",      "it is"),
        ("he'd",      "he would"),
        ("she'd",     "she would"),
        ("he'll",     "he will"),
        ("she'll",    "she will"),
        // ── they/we ───────────────────────────────────────────────────────
        ("they're",   "they are"),
        ("they've",   "they have"),
        ("they'll",   "they will"),
        ("they'd",    "they would"),
        ("we're",     "we are"),
        ("we've",     "we have"),
        ("we'll",     "we will"),
        ("we'd",      "we would"),
        // ── that/there/here/who/what ──────────────────────────────────────
        ("that's",    "that is"),
        ("there's",   "there is"),
        ("here's",    "here is"),
        ("who's",     "who is"),
        ("who've",    "who have"),
        ("who'll",    "who will"),
        ("who'd",     "who would"),
        ("what's",    "what is"),
        ("what've",   "what have"),
        ("what'll",   "what will"),
        ("what'd",    "what would"),
        // ── negatives ─────────────────────────────────────────────────────
        ("aren't",    "are not"),
        ("isn't",     "is not"),
        ("wasn't",    "was not"),
        ("weren't",   "were not"),
        ("don't",     "do not"),
        ("doesn't",   "does not"),
        ("didn't",    "did not"),
        ("won't",     "will not"),
        ("wouldn't",  "would not"),
        ("couldn't",  "could not"),
        ("shouldn't", "should not"),
        ("mightn't",  "might not"),
        ("mustn't",   "must not"),
        ("needn't",   "need not"),
        ("haven't",   "have not"),
        ("hasn't",    "has not"),
        ("hadn't",    "had not"),
        ("can't",     "cannot"),
        ("shan't",    "shall not"),
    ];

    let mut result = input.to_string();

    for (contracted, expanded) in expansions {
        let lower = result.to_lowercase();
        let mut new_str = String::with_capacity(result.len());
        let mut last = 0usize;

        while let Some(rel_pos) = lower[last..].find(contracted) {
            let abs = last + rel_pos;

            // Whole-token boundary check
            let before_ok = abs == 0
                || !result[..abs]
                    .chars()
                    .last()
                    .map(|c| c.is_alphabetic() || c == '\'')
                    .unwrap_or(false);
            let after_ok = abs + contracted.len() >= result.len()
                || !result[abs + contracted.len()..]
                    .chars()
                    .next()
                    .map(|c| c.is_alphabetic() || c == '\'')
                    .unwrap_or(false);

            if before_ok && after_ok {
                new_str.push_str(&result[last..abs]);
                let is_cap = result[abs..]
                    .chars()
                    .next()
                    .map(|c| c.is_uppercase())
                    .unwrap_or(false);
                let cap = capitalise_first(expanded);
                new_str.push_str(if is_cap { &cap } else { expanded });
                last = abs + contracted.len();
            } else {
                new_str.push_str(&result[last..abs + 1]);
                last = abs + 1;
            }
        }

        new_str.push_str(&result[last..]);
        result = new_str;
    }

    result
}

// ─────────────────────────────────────────────────────────────────────────────
// 1. TOKENIZER
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Word(String),
    Punctuation(String),
    Whitespace(String),
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        if ch.is_alphabetic() {
            let mut word = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_alphabetic() {
                    word.push(c);
                    chars.next();
                } else if c == '\'' {
                    // Include apostrophe only if the *next* char is alphabetic
                    // (handles o'er, ne'er, 'tis etc. but not trailing ')
                    let mut tmp = chars.clone();
                    tmp.next();
                    if tmp.peek().map(|nc| nc.is_alphabetic()).unwrap_or(false) {
                        word.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            tokens.push(Token::Word(word));
        } else if ch.is_whitespace() {
            let mut ws = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_whitespace() {
                    ws.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(Token::Whitespace(ws));
        } else {
            tokens.push(Token::Punctuation(ch.to_string()));
            chars.next();
        }
    }

    tokens
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. POS TAGGER (lightweight, rule-based)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum Pos {
    SubjectPronoun, // "you" as subject (before verb)
    ObjectPronoun,  // "you" as object (after verb/preposition)
    PossessiveDet,  // "your" before a noun
    PossessivePred, // "yours" standalone
    Reflexive,      // "yourself"
    Verb,
    Auxiliary,
    Article, // a / an
    Other,
}

fn pos_tag(words: &[&str]) -> Vec<Pos> {
    let auxiliaries = [
        "do", "does", "did", "have", "has", "had", "will", "shall", "would", "could", "should",
        "might", "may", "must", "can", "dare", "need",
    ];

    let verb_lemmas = [
        "be", "is", "am", "are", "was", "were", "go", "goes", "went", "come", "comes", "came",
        "see", "sees", "saw", "know", "knows", "knew", "say", "says", "said", "get", "gets",
        "got", "make", "makes", "made", "take", "takes", "took", "think", "thinks", "thought",
        "speak", "speaks", "spoke", "walk", "walks", "run", "runs", "love", "loves", "give",
        "gives", "gave", "find", "finds", "found", "tell", "tells", "told", "call", "calls",
        "called", "keep", "keeps", "kept", "let", "lets", "put", "puts", "seem", "seems", "feel",
        "feels", "felt", "leave", "leaves", "left", "turn", "turns", "ask", "asks", "show",
        "shows", "try", "tries", "use", "uses", "want", "wants", "need", "needs", "hear",
        "hears", "heard", "hold", "holds", "held", "bring", "brings", "brought", "stand",
        "stands", "stood", "fall", "falls", "fell", "send", "sends", "sent", "write", "writes",
        "wrote", "meet", "meets", "met", "lie", "lies", "lay", "rise", "rises", "rose", "lose",
        "loses", "lost", "pay", "pays", "paid", "lead", "leads", "led", "grow", "grows", "grew",
        "open", "opens", "opened", "close", "closes", "closed", "live", "lives", "lived",
        "serve", "serves", "served", "remember", "forget", "forgets", "forgot", "hope", "hopes",
        "hoped", "wish", "wishes", "wished", "fear", "fears", "feared", "trust", "trusts",
        "trusted", "believe", "believes", "believed", "pray", "prays", "prayed",
    ];

    let prepositions = [
        "to", "for", "with", "at", "by", "from", "of", "on", "in", "about", "between", "among",
        "against", "without", "near", "thank", "thanking", "tell", "told", "give", "gave", "see",
        "saw", "meet", "ask", "help", "beside", "beneath", "beyond", "unto", "upon",
    ];

    let mut tags = vec![Pos::Other; words.len()];

    for (i, word) in words.iter().enumerate() {
        let lower = word.to_lowercase();
        let prev = if i > 0 {
            words[i - 1].to_lowercase()
        } else {
            String::new()
        };

        match lower.as_str() {
            "you" => {
                let is_after_prep_or_verb = prepositions.contains(&prev.as_str())
                    || auxiliaries.contains(&prev.as_str())
                    || verb_lemmas.contains(&prev.as_str())
                    || prev.ends_with("ing")
                    || prev.ends_with("ed");
                tags[i] = if is_after_prep_or_verb {
                    Pos::ObjectPronoun
                } else {
                    Pos::SubjectPronoun
                };
            }
            "your" => tags[i] = Pos::PossessiveDet,
            "yours" => tags[i] = Pos::PossessivePred,
            "yourself" | "yourselves" => tags[i] = Pos::Reflexive,
            "a" | "an" => tags[i] = Pos::Article,
            w if auxiliaries.contains(&w) => tags[i] = Pos::Auxiliary,
            w if verb_lemmas.contains(&w) => tags[i] = Pos::Verb,
            w if w.ends_with("ing") || w.ends_with("ed") => tags[i] = Pos::Verb,
            w if w.ends_with('s') && w.len() > 3 => {
                let noun_like_prev = matches!(
                    prev.as_str(),
                    "he" | "she" | "it" | "one" | "man" | "woman" | "god" | "king" | "queen"
                        | "lord" | "lady"
                );
                tags[i] = if noun_like_prev {
                    Pos::Verb
                } else {
                    Pos::Other
                };
            }
            _ => {}
        }
    }

    tags
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. PRONOUN SUBSTITUTION
// ─────────────────────────────────────────────────────────────────────────────

/// `next_word` is used to choose `thy` vs `thine` (thine before a vowel sound).
fn substitute_pronouns(word: &str, pos: &Pos, next_word: Option<&str>) -> Option<String> {
    let is_upper = word.chars().next().map(|c| c.is_uppercase()).unwrap_or(false);

    let replacement: Option<&str> = match pos {
        Pos::SubjectPronoun => Some("thou"),
        Pos::ObjectPronoun => Some("thee"),
        Pos::PossessiveDet => {
            let starts_vowel = next_word
                .and_then(|w| w.chars().next())
                .map(|c| "aeiouAEIOU".contains(c))
                .unwrap_or(false);
            Some(if starts_vowel { "thine" } else { "thy" })
        }
        Pos::PossessivePred => Some("thine"),
        Pos::Reflexive => Some("thyself"),
        _ => None,
    };

    replacement.map(|r| if is_upper { capitalise_first(r) } else { r.to_string() })
}

// ─────────────────────────────────────────────────────────────────────────────
// 4 & 5. VERB CONJUGATION + IRREGULAR VERBS
// ─────────────────────────────────────────────────────────────────────────────

fn conjugate_for_thou(verb: &str) -> String {
    let lower = verb.to_lowercase();

    let irregular_thou: &[(&str, &str)] = &[
        ("are",      "art"),
        ("were",     "wert"),
        ("have",     "hast"),
        ("has",      "hast"),
        ("do",       "dost"),
        ("does",     "dost"),
        ("did",      "didst"),
        ("will",     "wilt"),
        ("shall",    "shalt"),
        ("would",    "wouldst"),
        ("could",    "couldst"),
        ("should",   "shouldst"),
        ("might",    "mightst"),
        ("must",     "must"),
        ("may",      "mayst"),
        ("know",     "knowest"),
        ("go",       "goest"),
        ("say",      "sayest"),
        ("see",      "seest"),
        ("come",     "comest"),
        ("think",    "thinkest"),
        ("speak",    "speakest"),
        ("love",     "lovest"),
        ("make",     "makest"),
        ("take",     "takest"),
        ("give",     "givest"),
        ("live",     "livest"),
        ("fear",     "fearest"),
        ("hear",     "hearest"),
        ("hold",     "holdest"),
        ("bring",    "bringest"),
        ("stand",    "standest"),
        ("send",     "sendest"),
        ("write",    "writest"),
        ("lose",     "losest"),
        ("seek",     "seekest"),
        ("trust",    "trustest"),
        ("wish",     "wishest"),
        ("pray",     "prayest"),
        ("forget",   "forgettest"),
        ("remember", "rememberest"),
        ("believe",  "believest"),
        ("desire",   "desirest"),
        ("require",  "requirest"),
        ("is",       "art"),
        ("am",       "art"),
    ];

    for (modern, shakes) in irregular_thou {
        if lower == *modern {
            return preserve_case(verb, shakes);
        }
    }

    if lower.ends_with('e') {
        preserve_case(verb, &format!("{}st", lower))
    } else {
        preserve_case(verb, &format!("{}est", lower))
    }
}

fn conjugate_third_person(verb: &str) -> String {
    let lower = verb.to_lowercase();

    let irregular_3rd: &[(&str, &str)] = &[
        ("has",  "hath"),
        ("does", "doth"),
        ("says", "saith"),
        ("goes", "goeth"),
        ("is",   "is"),
    ];

    for (modern, shakes) in irregular_3rd {
        if lower == *modern {
            return preserve_case(verb, shakes);
        }
    }

    if lower.ends_with("es") {
        preserve_case(verb, &format!("{}eth", &lower[..lower.len() - 2]))
    } else if lower.ends_with('s') {
        preserve_case(verb, &format!("{}eth", &lower[..lower.len() - 1]))
    } else {
        verb.to_string()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 6. VOCABULARY SUBSTITUTION
// ─────────────────────────────────────────────────────────────────────────────

fn vocabulary_substitution(word: &str) -> Option<String> {
    let lower = word.to_lowercase();

    let vocab: &[(&str, &str)] = &[
        // non vibe-coded things lol
        ("ur",                   "thy"),
        ("mom",                  "mother"),
        ("dad",                  "father"),
        ("u're",                 "thou art"),
        ("I have",               "I has't"),
            
        ("I love thee",           "I loveth thee"),
        ("I like thee",           "I liketh thee"),
        ("I miss thee",           "I misseth thee"),
        ("I hate thee",           "I has't utmost misprise for thee"),
        ("I despise thee",        "I has't utmost misprise for thee"),

        // ── Affirmatives / negatives ─────────────────────────────────────
        ("yes",                  "aye"),
        ("yeah",                 "aye"),
        ("yep",                  "aye"),
        ("yup",                  "aye"),
        ("no",                   "nay"),
        ("nope",                 "nay"),
        ("nah",                  "nay"),
        ("ok",                   "very well"),
        ("okay",                 "very well"),
        ("alright",              "very well"),
        ("sure",                 "certes"),
        ("of course",            "forsooth"),
        ("absolutely",           "verily"),
        ("exactly",              "even so"),
        ("correct",              "thou hast it right"),
        ("wrong",                "nay, thou errest"),
        ("never",                "ne'er"),
        ("ever",                 "e'er"),
        ("not",                  "not"),

        // ── Greetings / farewells ────────────────────────────────────────
        ("hello",                "good morrow"),
        ("hi",                   "good morrow"),
        ("hey",                  "hark"),
        ("greetings",            "well met"),
        ("welcome",              "thou art most welcome"),
        ("goodbye",              "fare thee well"),
        ("farewell",             "fare thee well"),
        ("bye",                  "fare thee well"),
        ("see you",              "till we meet again"),
        ("see you later",        "fare thee well for now"),
        ("good morning",         "good morrow"),
        ("good afternoon",       "good day"),
        ("good evening",         "good e'en"),
        ("good night",           "good night"),
        ("how are you",          "how dost thou fare"),
        ("how do you do",        "how dost thou"),
        ("nice to meet you",     "well met"),
        ("pleased to meet you",  "right glad am I to meet thee"),

        // ── Politeness / requests ────────────────────────────────────────
        ("please",               "prithee"),
        ("thank you",            "grammercy"),
        ("thanks",               "grammercy"),
        ("thank",                "I thank"),
        ("excuse me",            "I prithee pardon"),
        ("pardon me",            "I prithee pardon"),
        ("sorry",                "I am most heartily sorry"),
        ("I apologize",          "I do humbly beg thy pardon"),
        ("forgive me",           "prithee forgive me"),
        ("help",                 "aid"),
        ("please help",          "prithee aid me"),
        ("I need help",          "I require thine aid"),
        ("can you help",         "canst thou aid me"),

        // ── Time adverbs ─────────────────────────────────────────────────
        ("before",               "ere"),
        ("after",                "after"),
        ("now",                  "presently"),
        ("right now",            "forthwith"),
        ("immediately",          "forthwith"),
        ("quickly",              "apace"),
        ("soon",                 "anon"),
        ("later",                "by and by"),
        ("eventually",           "in good time"),
        ("already",              "ere now"),
        ("still",                "yet"),
        ("again",                "once more"),
        ("always",               "evermore"),
        ("forever",              "for evermore"),
        ("often",                "oft"),
        ("sometimes",            "betimes"),
        ("rarely",               "seldom"),
        ("never",                "ne'er"),
        ("ever",                 "e'er"),
        ("once",                 "one time"),
        ("twice",                "two times"),
        ("today",                "this day"),
        ("yesterday",            "yester-day"),
        ("tomorrow",             "the morrow"),
        ("tonight",              "this night"),
        ("last night",           "yester-night"),
        ("this morning",         "this morrow"),
        ("this evening",         "this e'en"),
        ("meanwhile",            "in the meantime"),
        ("suddenly",             "of a sudden"),
        ("recently",             "of late"),
        ("long ago",             "in days of yore"),
        ("in the past",          "in days gone by"),
        ("in the future",        "in time to come"),
        ("at last",              "at long last"),
        ("finally",              "at long last"),
        ("early",                "betimes"),
        ("late",                 "tardy"),
        ("on time",              "in good time"),

        // ── Place adverbs ────────────────────────────────────────────────
        ("here",                 "hither"),
        ("there",                "thither"),
        ("where",                "whither"),
        ("everywhere",           "wheresoever"),
        ("nowhere",              "nowhere"),
        ("somewhere",            "somewhere"),
        ("away",                 "hence"),
        ("from here",            "hence"),
        ("from there",           "thence"),
        ("from where",           "whence"),
        ("nearby",               "nigh"),
        ("near",                 "nigh"),
        ("far",                  "afar"),
        ("far away",             "far hence"),
        ("inside",               "within"),
        ("outside",              "without"),
        ("above",                "above"),
        ("below",                "beneath"),
        ("under",                "beneath"),
        ("over",                 "o'er"),
        ("across",               "athwart"),
        ("through",              "through"),
        ("around",               "about"),
        ("behind",               "behind"),
        ("in front",             "before"),
        ("beside",               "beside"),
        ("beyond",               "beyond"),
        ("up",                   "aloft"),
        ("down",                 "below"),
        ("forward",              "onward"),
        ("backward",             "back"),
        ("upstairs",             "above stairs"),
        ("downstairs",           "below stairs"),
        ("home",                 "mine abode"),
        ("abroad",               "in foreign lands"),

        // ── Conjunctions / connectives ───────────────────────────────────
        ("because",              "for"),
        ("although",             "albeit"),
        ("though",               "albeit"),
        ("even though",          "howbeit"),
        ("however",              "howbeit"),
        ("nevertheless",         "natheless"),
        ("nonetheless",          "natheless"),
        ("while",                "whilst"),
        ("whereas",              "whereas"),
        ("until",                "till"),
        ("unless",               "save"),
        ("except",               "save"),
        ("if",                   "if"),
        ("whether",              "whether"),
        ("either",               "either"),
        ("neither",              "neither"),
        ("both",                 "both"),
        ("therefore",            "hence"),
        ("thus",                 "thus"),
        ("so",                   "thus"),
        ("then",                 "then"),
        ("also",                 "likewise"),
        ("furthermore",          "moreover"),
        ("moreover",             "moreover"),
        ("besides",              "withal"),
        ("meanwhile",            "in the meantime"),
        ("otherwise",            "else"),
        ("instead",              "in lieu"),
        ("indeed",               "forsooth"),
        ("actually",             "in sooth"),
        ("truly",                "verily"),
        ("certainly",            "certes"),
        ("surely",               "forsooth"),
        ("perhaps",              "perchance"),
        ("maybe",                "haply"),
        ("possibly",             "mayhaps"),
        ("probably",             "like enough"),
        ("supposedly",           "as they say"),
        ("apparently",           "it seemeth"),
        ("obviously",            "plainly"),
        ("clearly",              "plainly"),
        ("frankly",              "in plain speech"),
        ("honestly",             "in good sooth"),
        ("luckily",              "by good fortune"),
        ("unfortunately",        "alas"),
        ("thankfully",           "I give thanks"),

        // ── Degree adverbs ───────────────────────────────────────────────
        ("very",                 "most"),
        ("really",               "exceeding"),
        ("quite",                "passing"),
        ("rather",               "somewhat"),
        ("extremely",            "exceeding"),
        ("incredibly",           "wondrous"),
        ("tremendously",         "mightily"),
        ("terribly",             "most grievously"),
        ("awfully",              "most grievously"),
        ("horribly",             "most grievously"),
        ("wonderfully",          "wondrously"),
        ("beautifully",          "most beauteously"),
        ("perfectly",            "in all perfection"),
        ("completely",           "wholly"),
        ("totally",              "wholly"),
        ("entirely",             "wholly"),
        ("absolutely",           "utterly"),
        ("utterly",              "utterly"),
        ("somewhat",             "in some measure"),
        ("slightly",             "in small measure"),
        ("barely",               "scarce"),
        ("hardly",               "scarce"),
        ("scarcely",             "scarce"),
        ("almost",               "well nigh"),
        ("nearly",               "well nigh"),
        ("enough",               "sufficient"),
        ("too",                  "o'ermuch"),
        ("too much",             "o'ermuch"),
        ("more",                 "more"),
        ("less",                 "less"),
        ("most",                 "most"),
        ("least",                "least"),
        ("only",                 "only"),
        ("just",                 "but"),
        ("even",                 "even"),
        ("still",                "yet"),
        ("already",              "ere now"),
        ("yet",                  "yet"),

        // ── Questions ────────────────────────────────────────────────────
        ("what",                 "what"),
        ("why",                  "wherefore"),
        ("how",                  "how"),
        ("when",                 "when"),
        ("who",                  "who"),
        ("which",                "which"),
        ("whose",                "whose"),
        ("what is",              "what is"),
        ("why is",               "wherefore is"),
        ("how much",             "how much"),
        ("how many",             "how many"),
        ("what are you doing",   "what dost thou"),
        ("what do you want",     "what dost thou desire"),
        ("where are you going",  "whither dost thou go"),
        ("who are you",          "who art thou"),
        ("what is your name",    "what is thy name"),

        // ── Common nouns — people ────────────────────────────────────────
        ("person",               "soul"),
        ("people",               "souls"),
        ("man",                  "fellow"),
        ("men",                  "fellows"),
        ("woman",                "mistress"),
        ("women",                "mistresses"),
        ("boy",                  "lad"),
        ("girl",                 "lass"),
        ("child",                "child"),
        ("children",             "children"),
        ("baby",                 "babe"),
        ("infant",               "babe"),
        ("teenager",             "youth"),
        ("adult",                "grown soul"),
        ("elder",                "elder"),
        ("old man",              "aged fellow"),
        ("old woman",            "aged mistress"),
        ("friend",               "good fellow"),
        ("friends",              "good fellows"),
        ("enemy",                "foe"),
        ("enemies",              "foes"),
        ("companion",            "comrade"),
        ("companions",           "comrades"),
        ("stranger",             "unknown soul"),
        ("neighbor",             "neighbour"),
        ("neighbour",            "neighbour"),
        ("servant",              "servant"),
        ("master",               "master"),
        ("slave",                "bondsman"),
        ("soldier",              "man-at-arms"),
        ("warrior",              "warrior"),
        ("knight",               "knight"),
        ("king",                 "king"),
        ("queen",                "queen"),
        ("prince",               "prince"),
        ("princess",             "princess"),
        ("lord",                 "lord"),
        ("lady",                 "lady"),
        ("duke",                 "duke"),
        ("earl",                 "earl"),
        ("baron",                "baron"),
        ("noble",                "noble"),
        ("peasant",              "commoner"),
        ("merchant",             "merchant"),
        ("priest",               "friar"),
        ("doctor",               "physician"),
        ("lawyer",               "man of law"),
        ("judge",                "justice"),
        ("teacher",              "tutor"),
        ("student",              "pupil"),
        ("scholar",              "scholar"),
        ("poet",                 "poet"),
        ("actor",                "player"),
        ("musician",             "minstrel"),
        ("painter",              "limner"),
        ("writer",               "scrivener"),
        ("thief",                "cutpurse"),
        ("murderer",             "slayer"),
        ("villain",              "villain"),
        ("hero",                 "champion"),
        ("champion",             "champion"),
        ("ghost",                "spectre"),
        ("witch",                "witch"),
        ("wizard",               "sorcerer"),
        ("god",                  "God"),
        ("gods",                 "the gods"),
        ("devil",                "the devil"),
        ("angel",                "angel"),
        ("sir",                  "good sir"),
        ("mr",                   "good master"),
        ("mrs",                  "good mistress"),
        ("ms",                   "good mistress"),
        ("miss",                 "good mistress"),
        ("madam",                "good madam"),
        ("she",                  "that lady"),
        ("he",                   "that gent"),
        ("they",                 "those gents"),

        // ── Common nouns — body ──────────────────────────────────────────
        ("body",                 "form"),
        ("head",                 "head"),
        ("face",                 "visage"),
        ("eye",                  "eye"),
        ("eyes",                 "eyes"),
        ("ear",                  "ear"),
        ("ears",                 "ears"),
        ("nose",                 "nose"),
        ("mouth",                "mouth"),
        ("lip",                  "lip"),
        ("lips",                 "lips"),
        ("tongue",               "tongue"),
        ("teeth",                "teeth"),
        ("tooth",                "tooth"),
        ("neck",                 "neck"),
        ("throat",               "throat"),
        ("shoulder",             "shoulder"),
        ("arm",                  "arm"),
        ("hand",                 "hand"),
        ("hands",                "hands"),
        ("finger",               "finger"),
        ("fingers",              "fingers"),
        ("thumb",                "thumb"),
        ("fist",                 "fist"),
        ("chest",                "breast"),
        ("heart",                "heart"),
        ("stomach",              "belly"),
        ("back",                 "back"),
        ("leg",                  "leg"),
        ("foot",                 "foot"),
        ("feet",                 "feet"),
        ("toe",                  "toe"),
        ("skin",                 "skin"),
        ("hair",                 "hair"),
        ("blood",                "blood"),
        ("bone",                 "bone"),
        ("brain",                "wit"),
        ("mind",                 "wit"),
        ("soul",                 "soul"),
        ("spirit",               "spirit"),
        ("breath",               "breath"),
        ("voice",                "voice"),
        ("tear",                 "tear"),
        ("tears",                "tears"),
        ("shadow",               "shadow"),

        // ── Common nouns — nature ────────────────────────────────────────
        ("sun",                  "sun"),
        ("moon",                 "moon"),
        ("star",                 "star"),
        ("stars",                "stars"),
        ("sky",                  "sky"),
        ("heaven",               "heaven"),
        ("heavens",              "the heavens"),
        ("earth",                "earth"),
        ("ground",               "ground"),
        ("land",                 "land"),
        ("sea",                  "sea"),
        ("ocean",                "ocean"),
        ("river",                "river"),
        ("lake",                 "lake"),
        ("pond",                 "pond"),
        ("stream",               "brook"),
        ("mountain",             "mountain"),
        ("hill",                 "hill"),
        ("valley",               "vale"),
        ("forest",               "forest"),
        ("wood",                 "wood"),
        ("tree",                 "tree"),
        ("flower",               "flower"),
        ("grass",                "grass"),
        ("stone",                "stone"),
        ("rock",                 "rock"),
        ("fire",                 "fire"),
        ("flame",                "flame"),
        ("water",                "water"),
        ("air",                  "air"),
        ("wind",                 "wind"),
        ("storm",                "tempest"),
        ("rain",                 "rain"),
        ("snow",                 "snow"),
        ("ice",                  "ice"),
        ("thunder",              "thunder"),
        ("lightning",            "lightning"),
        ("cloud",                "cloud"),
        ("fog",                  "mist"),
        ("mist",                 "mist"),
        ("darkness",             "darkness"),
        ("light",                "light"),
        ("shadow",               "shadow"),
        ("dawn",                 "dawn"),
        ("dusk",                 "dusk"),
        ("night",                "night"),
        ("day",                  "day"),
        ("morning",              "morrow"),
        ("evening",              "e'en"),
        ("noon",                 "high noon"),
        ("midnight",             "the witching hour"),
        ("spring",               "spring"),
        ("summer",               "summer"),
        ("autumn",               "autumn"),
        ("fall",                 "autumn"),
        ("winter",               "winter"),
        ("season",               "season"),
        ("year",                 "year"),
        ("month",                "month"),
        ("week",                 "se'nnight"),
        ("hour",                 "hour"),
        ("minute",               "moment"),
        ("second",               "instant"),
        ("moment",               "moment"),
        ("time",                 "time"),
        ("age",                  "age"),
        ("era",                  "age"),
        ("century",              "century"),

        // ── Common nouns — abstract ──────────────────────────────────────
        ("life",                 "life"),
        ("death",                "death"),
        ("love",                 "love"),
        ("hate",                 "hatred"),
        ("hope",                 "hope"),
        ("fear",                 "dread"),
        ("joy",                  "joy"),
        ("happiness",            "mirth"),
        ("sadness",              "woe"),
        ("anger",                "wrath"),
        ("sorrow",               "sorrow"),
        ("grief",                "grief"),
        ("pain",                 "pain"),
        ("suffering",            "suffering"),
        ("peace",                "peace"),
        ("war",                  "war"),
        ("battle",               "battle"),
        ("victory",              "triumph"),
        ("defeat",               "defeat"),
        ("glory",                "glory"),
        ("shame",                "shame"),
        ("honour",               "honour"),
        ("honor",                "honour"),
        ("pride",                "pride"),
        ("humility",             "humility"),
        ("courage",              "valour"),
        ("cowardice",            "cravenness"),
        ("wisdom",               "wisdom"),
        ("foolishness",          "folly"),
        ("truth",                "truth"),
        ("lie",                  "falsehood"),
        ("lies",                 "falsehoods"),
        ("secret",               "secret"),
        ("mystery",              "mystery"),
        ("fate",                 "fate"),
        ("destiny",              "destiny"),
        ("fortune",              "fortune"),
        ("luck",                 "fortune"),
        ("chance",               "happenstance"),
        ("danger",               "peril"),
        ("safety",               "safety"),
        ("power",                "power"),
        ("strength",             "might"),
        ("weakness",             "feebleness"),
        ("beauty",               "beauteous grace"),
        ("ugliness",             "foulness"),
        ("kindness",             "kindness"),
        ("cruelty",              "cruelty"),
        ("mercy",                "mercy"),
        ("justice",              "justice"),
        ("sin",                  "sin"),
        ("virtue",               "virtue"),
        ("evil",                 "wickedness"),
        ("goodness",             "goodness"),
        ("magic",                "sorcery"),
        ("dream",                "dream"),
        ("dreams",               "dreams"),
        ("nightmare",            "dark dream"),
        ("memory",               "memory"),
        ("thought",              "thought"),
        ("idea",                 "notion"),
        ("word",                 "word"),
        ("words",                "words"),
        ("story",                "tale"),
        ("song",                 "song"),
        ("music",                "music"),
        ("silence",              "silence"),
        ("noise",                "clamour"),
        ("sound",                "sound"),
        ("voice",                "voice"),
        ("promise",              "oath"),
        ("oath",                 "oath"),
        ("vow",                  "vow"),
        ("curse",                "curse"),
        ("blessing",             "blessing"),
        ("prayer",               "prayer"),
        ("faith",                "faith"),
        ("doubt",                "doubt"),
        ("trust",                "trust"),
        ("betrayal",             "betrayal"),
        ("revenge",              "vengeance"),
        ("forgiveness",          "pardon"),
        ("friendship",           "fellowship"),
        ("loyalty",              "fealty"),
        ("freedom",              "liberty"),
        ("slavery",              "bondage"),
        ("wealth",               "riches"),
        ("poverty",              "poverty"),
        ("money",                "coin"),
        ("gold",                 "gold"),
        ("silver",               "silver"),
        ("treasure",             "treasure"),
        ("gift",                 "gift"),
        ("reward",               "reward"),
        ("punishment",           "punishment"),

        // ── Common nouns — objects ───────────────────────────────────────
        ("sword",                "sword"),
        ("knife",                "dagger"),
        ("dagger",               "dagger"),
        ("shield",               "shield"),
        ("armor",                "armour"),
        ("armour",               "armour"),
        ("helmet",               "helm"),
        ("bow",                  "bow"),
        ("arrow",                "arrow"),
        ("spear",                "spear"),
        ("weapon",               "weapon"),
        ("horse",                "steed"),
        ("ship",                 "vessel"),
        ("boat",                 "vessel"),
        ("house",                "dwelling"),
        ("home",                 "abode"),
        ("castle",               "castle"),
        ("tower",                "tower"),
        ("door",                 "door"),
        ("window",               "window"),
        ("wall",                 "wall"),
        ("road",                 "road"),
        ("path",                 "path"),
        ("bridge",               "bridge"),
        ("book",                 "tome"),
        ("letter",               "letter"),
        ("map",                  "map"),
        ("candle",               "candle"),
        ("torch",                "torch"),
        ("lantern",              "lantern"),
        ("cup",                  "goblet"),
        ("glass",                "goblet"),
        ("bottle",               "flagon"),
        ("food",                 "victuals"),
        ("bread",                "bread"),
        ("meat",                 "meat"),
        ("wine",                 "wine"),
        ("water",                "water"),
        ("coin",                 "coin"),
        ("ring",                 "ring"),
        ("crown",                "crown"),
        ("key",                  "key"),
        ("chain",                "chain"),
        ("rope",                 "rope"),
        ("cloth",                "cloth"),
        ("cloak",                "cloak"),
        ("robe",                 "robe"),
        ("dress",                "gown"),
        ("hat",                  "hat"),
        ("bed",                  "bed"),
        ("throne",               "throne"),
        ("table",                "table"),
        ("chair",                "chair"),
        ("mirror",               "looking glass"),
        ("bag",                  "satchel"),
        ("box",                  "coffer"),

        // ── Adjectives — character ───────────────────────────────────────
        ("good",                 "good"),
        ("bad",                  "ill"),
        ("evil",                 "wicked"),
        ("wicked",               "wicked"),
        ("holy",                 "holy"),
        ("sacred",               "sacred"),
        ("cursed",               "accursed"),
        ("brave",                "valiant"),
        ("cowardly",             "craven"),
        ("strong",               "mighty"),
        ("weak",                 "feeble"),
        ("wise",                 "wise"),
        ("foolish",              "foolish"),
        ("clever",               "cunning"),
        ("stupid",               "witless"),
        ("kind",                 "kind"),
        ("cruel",                "cruel"),
        ("gentle",               "gentle"),
        ("harsh",                "harsh"),
        ("proud",                "proud"),
        ("humble",               "humble"),
        ("honest",               "honest"),
        ("dishonest",            "false"),
        ("loyal",                "faithful"),
        ("treacherous",          "treacherous"),
        ("generous",             "generous"),
        ("greedy",               "greedy"),
        ("jealous",              "jealous"),
        ("envious",              "envious"),
        ("merciful",             "merciful"),
        ("ruthless",             "ruthless"),
        ("patient",              "patient"),
        ("impatient",            "impatient"),
        ("calm",                 "calm"),
        ("angry",                "wroth"),
        ("happy",                "merry"),
        ("sad",                  "woeful"),
        ("tired",                "weary"),
        ("afraid",               "afeared"),
        ("scared",               "afeared"),
        ("brave",                "valiant"),
        ("bold",                 "bold"),
        ("shy",                  "bashful"),
        ("proud",                "proud"),
        ("arrogant",             "haughty"),
        ("modest",               "modest"),
        ("lazy",                 "idle"),
        ("hardworking",          "industrious"),
        ("ambitious",            "ambitious"),
        ("content",              "content"),
        ("miserable",            "wretched"),
        ("mad",                  "mad"),
        ("crazy",                "mad"),
        ("sane",                 "of sound mind"),
        ("drunk",                "in his cups"),
        ("sober",                "sober"),

        // ── Adjectives — appearance / quality ───────────────────────────
        ("beautiful",            "beauteous"),
        ("pretty",               "fair"),
        ("handsome",             "comely"),
        ("ugly",                 "foul"),
        ("old",                  "aged"),
        ("young",                "youthful"),
        ("new",                  "new"),
        ("ancient",              "ancient"),
        ("big",                  "great"),
        ("large",                "great"),
        ("huge",                 "vast"),
        ("enormous",             "vast"),
        ("small",                "little"),
        ("tiny",                 "tiny"),
        ("tall",                 "tall"),
        ("short",                "short"),
        ("long",                 "long"),
        ("wide",                 "wide"),
        ("narrow",               "narrow"),
        ("thick",                "thick"),
        ("thin",                 "thin"),
        ("heavy",                "heavy"),
        ("light",                "light"),
        ("fast",                 "swift"),
        ("slow",                 "slow"),
        ("quick",                "swift"),
        ("dark",                 "dark"),
        ("bright",               "bright"),
        ("shining",              "gleaming"),
        ("pale",                 "pale"),
        ("red",                  "red"),
        ("white",                "white"),
        ("black",                "black"),
        ("golden",               "golden"),
        ("silver",               "silver"),
        ("green",                "green"),
        ("blue",                 "blue"),
        ("clean",                "clean"),
        ("dirty",                "filthy"),
        ("smooth",               "smooth"),
        ("rough",                "rough"),
        ("sharp",                "sharp"),
        ("dull",                 "dull"),
        ("hard",                 "hard"),
        ("soft",                 "soft"),
        ("warm",                 "warm"),
        ("cold",                 "cold"),
        ("hot",                  "hot"),
        ("wet",                  "wet"),
        ("dry",                  "dry"),
        ("empty",                "empty"),
        ("full",                 "full"),
        ("rich",                 "rich"),
        ("poor",                 "poor"),
        ("famous",               "renowned"),
        ("unknown",              "unknown"),
        ("powerful",             "powerful"),
        ("powerless",            "powerless"),
        ("important",            "of great import"),
        ("worthless",            "worthless"),
        ("precious",             "precious"),
        ("terrible",             "terrible"),
        ("wonderful",            "wondrous"),
        ("strange",              "strange"),
        ("normal",               "common"),
        ("special",              "special"),
        ("secret",               "secret"),
        ("hidden",               "hidden"),
        ("true",                 "true"),
        ("false",                "false"),
        ("real",                 "real"),
        ("fake",                 "counterfeit"),
        ("alive",                "alive"),
        ("dead",                 "dead"),
        ("mortal",               "mortal"),
        ("immortal",             "immortal"),
        ("alone",                "alone"),
        ("together",             "together"),
        ("lost",                 "lost"),
        ("free",                 "free"),
        ("trapped",              "ensnared"),
        ("safe",                 "safe"),
        ("dangerous",            "perilous"),

        // ── Verbs — motion ───────────────────────────────────────────────
        ("go",                   "go"),
        ("come",                 "come"),
        ("walk",                 "walk"),
        ("run",                  "run"),
        ("flee",                 "flee"),
        ("follow",               "follow"),
        ("lead",                 "lead"),
        ("leave",                "depart"),
        ("arrive",               "arrive"),
        ("return",               "return"),
        ("enter",                "enter"),
        ("exit",                 "depart"),
        ("rise",                 "rise"),
        ("fall",                 "fall"),
        ("climb",                "climb"),
        ("jump",                 "leap"),
        ("leap",                 "leap"),
        ("swim",                 "swim"),
        ("fly",                  "fly"),
        ("travel",               "journey"),
        ("wander",               "wander"),
        ("chase",                "pursue"),
        ("escape",               "escape"),
        ("advance",              "advance"),
        ("retreat",              "retreat"),
        ("kneel",                "kneel"),
        ("bow",                  "bow"),
        ("stand",                "stand"),
        ("sit",                  "sit"),
        ("lie",                  "lie"),
        ("sleep",                "slumber"),
        ("wake",                 "wake"),
        ("hide",                 "hide"),
        ("approach",             "draw nigh"),

        // ── Verbs — speech / thought ─────────────────────────────────────
        ("say",                  "say"),
        ("speak",                "speak"),
        ("talk",                 "speak"),
        ("tell",                 "tell"),
        ("ask",                  "ask"),
        ("answer",               "answer"),
        ("reply",                "reply"),
        ("shout",                "cry out"),
        ("whisper",              "whisper"),
        ("sing",                 "sing"),
        ("read",                 "read"),
        ("write",                "write"),
        ("think",                "think"),
        ("know",                 "know"),
        ("understand",           "comprehend"),
        ("believe",              "believe"),
        ("doubt",                "doubt"),
        ("wonder",               "wonder"),
        ("imagine",              "imagine"),
        ("remember",             "recall"),
        ("forget",               "forget"),
        ("learn",                "learn"),
        ("teach",                "teach"),
        ("listen",               "hark"),
        ("hear",                 "hear"),
        ("watch",                "observe"),
        ("look",                 "behold"),
        ("see",                  "see"),
        ("notice",               "perceive"),
        ("ignore",               "disregard"),
        ("call",                 "call"),
        ("name",                 "name"),
        ("curse",                "curse"),
        ("praise",               "praise"),
        ("lie",                  "speak falsely"),
        ("swear",                "swear"),
        ("promise",              "promise"),
        ("warn",                 "warn"),
        ("threaten",             "threaten"),
        ("boast",                "boast"),
        ("complain",             "complain"),
        ("confess",              "confess"),
        ("deny",                 "deny"),
        ("explain",              "explain"),
        ("describe",             "describe"),
        ("announce",             "proclaim"),
        ("declare",              "declare"),
        ("command",              "command"),
        ("order",                "command"),
        ("beg",                  "beseech"),
        ("plead",                "plead"),
        ("argue",                "argue"),
        ("agree",                "agree"),
        ("disagree",             "disagree"),
        ("convince",             "persuade"),
        ("eat",                   "feed"),

        // ── Verbs — action ───────────────────────────────────────────────
        ("do",                   "do"),
        ("make",                 "make"),
        ("take",                 "take"),
        ("give",                 "give"),
        ("get",                  "get"),
        ("have",                 "have"),
        ("keep",                 "keep"),
        ("hold",                 "hold"),
        ("carry",                "carry"),
        ("bring",                "bring"),
        ("send",                 "send"),
        ("throw",                "throw"),
        ("catch",                "catch"),
        ("drop",                 "drop"),
        ("put",                  "put"),
        ("place",                "place"),
        ("move",                 "move"),
        ("push",                 "push"),
        ("pull",                 "pull"),
        ("lift",                 "lift"),
        ("open",                 "open"),
        ("close",                "close"),
        ("break",                "break"),
        ("build",                "build"),
        ("destroy",              "destroy"),
        ("create",               "create"),
        ("find",                 "find"),
        ("lose",                 "lose"),
        ("search",               "seek"),
        ("seek",                 "seek"),
        ("try",                  "endeavour"),
        ("attempt",              "endeavour"),
        ("use",                  "use"),
        ("help",                 "aid"),
        ("stop",                 "cease"),
        ("start",                "commence"),
        ("begin",                "commence"),
        ("finish",               "finish"),
        ("end",                  "end"),
        ("wait",                 "tarry"),
        ("hurry",                "make haste"),
        ("fight",                "combat"),
        ("attack",               "assail"),
        ("defend",               "defend"),
        ("kill",                 "slay"),
        ("die",                  "perish"),
        ("save",                 "save"),
        ("protect",              "protect"),
        ("steal",                "steal"),
        ("buy",                  "purchase"),
        ("sell",                 "sell"),
        ("pay",                  "pay"),
        ("win",                  "prevail"),
        ("lose",                 "suffer defeat"),
        ("play",                 "play"),
        ("work",                 "labour"),
        ("eat",                  "eat"),
        ("drink",                "drink"),
        ("cook",                 "cook"),
        ("grow",                 "grow"),
        ("plant",                "plant"),
        ("cut",                  "cut"),
        ("wash",                 "wash"),
        ("wear",                 "wear"),
        ("dress",                "dress"),
        ("choose",               "choose"),
        ("decide",               "decide"),
        ("change",               "change"),
        ("show",                 "show"),
        ("hide",                 "hide"),
        ("reveal",               "reveal"),
        ("discover",             "discover"),
        ("invent",               "invent"),
        ("plan",                 "plan"),
        ("prepare",              "prepare"),
        ("practice",             "practise"),
        ("succeed",              "succeed"),
        ("fail",                 "fail"),
        ("suffer",               "suffer"),
        ("enjoy",                "enjoy"),
        ("hate",                 "despise"),
        ("love",                 "love"),
        ("want",                 "desire"),
        ("need",                 "require"),
        ("wish",                 "wish"),
        ("hope",                 "hope"),
        ("fear",                 "dread"),
        ("trust",                "trust"),
        ("forgive",              "pardon"),
        ("punish",               "punish"),
        ("reward",               "reward"),
        ("accept",               "accept"),
        ("refuse",               "refuse"),
        ("obey",                 "obey"),
        ("disobey",              "disobey"),
        ("betray",               "betray"),
        ("swear",                "swear"),
        ("pray",                 "pray"),
        ("bow",                  "bow"),
        ("worship",              "worship"),
        ("marry",                "wed"),
        ("wed",                  "wed"),
        ("divorce",              "put asunder"),
        ("born",                 "born"),
        ("live",                 "live"),
        ("survive",              "survive"),

        // ── Exclamations ─────────────────────────────────────────────────
        ("wow",                  "marry"),
        ("oh",                   "oh"),
        ("ah",                   "ah"),
        ("oh no",                "alas"),
        ("alas",                 "alas"),
        ("oh my",                "good heavens"),
        ("oh god",               "good heavens"),
        ("what the",             "what in the name of"),
        ("damn",                 "'sdeath"),
        ("dammit",               "zounds"),
        ("hell",                 "the devil"),
        ("great",                "excellent"),
        ("amazing",              "wondrous"),
        ("awesome",              "most wondrous"),
        ("terrible",             "most grievous"),
        ("horrible",             "most grievous"),
        ("perfect",              "most perfect"),
        ("excellent",            "excellent"),
        ("wonderful",            "wondrous"),
        ("good job",             "well done"),
        ("well done",            "well done"),
        ("bravo",                "huzzah"),
        ("congratulations",      "huzzah"),
        ("hurray",               "huzzah"),

        // ── Misc / modern phrases ────────────────────────────────────────
        ("actually",             "in sooth"),
        ("basically",            "in brief"),
        ("seriously",            "in good earnest"),
        ("whatever",             "as thou wilt"),
        ("anyway",               "howbeit"),
        ("like",                 "as"),
        ("stuff",                "sundry things"),
        ("thing",                "thing"),
        ("things",               "things"),
        ("something",            "somewhat"),
        ("anything",             "aught"),
        ("nothing",              "naught"),
        ("everything",           "all"),
        ("everyone",             "every soul"),
        ("nobody",               "none"),
        ("someone",              "some soul"),
        ("anyone",               "any soul"),
        ("somewhere",            "somewhere"),
        ("nowhere",              "nowhere"),
        ("everywhere",           "wheresoever"),
        ("somehow",              "by some means"),
        ("anyway",               "howsoever"),
        ("kind of",              "in a manner"),
        ("sort of",              "in a manner"),
        ("a lot",                "in great measure"),
        ("a little",             "in small measure"),
        ("at least",             "at the least"),
        ("at most",              "at the most"),
        ("in fact",              "in sooth"),
        ("in general",           "in general"),
        ("for example",          "for example"),
        ("such as",              "such as"),
        ("as well",              "likewise"),
        ("as well as",           "as well as"),
        ("instead of",           "in lieu of"),
        ("because of",           "by reason of"),
        ("in spite of",          "despite"),
        ("according to",         "as saith"),
        ("in order to",          "so as to"),
        ("as long as",           "so long as"),
        ("as soon as",           "so soon as"),
        ("even if",              "even if"),
        ("so that",              "so that"),
        ("in case",              "lest"),
        ("right",                "right"),
        ("left",                 "left"),
        ("next",                 "next"),
        ("last",                 "last"),
        ("first",                "first"),
        ("second",               "second"),
        ("third",                "third"),
        ("other",                "other"),
        ("same",                 "same"),
        ("different",            "different"),
        ("own",                  "own"),
        ("each",                 "each"),
        ("every",                "every"),
        ("all",                  "all"),
        ("both",                 "both"),
        ("few",                  "few"),
        ("many",                 "many"),
        ("much",                 "much"),
        ("more",                 "more"),
        ("most",                 "most"),
        ("some",                 "some"),
        ("any",                  "any"),
        ("no",                   "no"),
        ("none",                 "none"),
        ("if",                   "an"),
        ("will",                 "shall")
    ];

    for (modern, shakes) in vocab {
        if lower == *modern {
            return Some(preserve_case(word, shakes));
        }
    }

    None
}

// ─────────────────────────────────────────────────────────────────────────────
// 7. ARTICLE AGREEMENT FIX  (a / an)
// ─────────────────────────────────────────────────────────────────────────────

/// After all substitutions, scan the token list and correct any `a`/`an`
/// article that no longer agrees with its following word.
fn fix_articles(tokens: &mut Vec<(Token, Pos)>) {
    let indices: Vec<usize> = tokens
        .iter()
        .enumerate()
        .filter_map(|(i, (_, pos))| {
            if *pos == Pos::Article {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    for idx in indices {
        // Find next Word token (skip whitespace/punctuation)
        let next_word = tokens[idx + 1..]
            .iter()
            .find_map(|(tok, _)| {
                if let Token::Word(w) = tok {
                    Some(w.clone())
                } else {
                    None
                }
            });

        if let Some(next) = next_word {
            let starts_vowel = next
                .chars()
                .next()
                .map(|c| "aeiouAEIOU".contains(c))
                .unwrap_or(false);

            if let (Token::Word(article), _) = &mut tokens[idx] {
                let is_cap = article.chars().next().map(|c| c.is_uppercase()).unwrap_or(false);
                let correct = if starts_vowel { "an" } else { "a" };
                *article = if is_cap {
                    capitalise_first(correct)
                } else {
                    correct.to_string()
                };
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 8. SHAKESPEAREAN CONTRACTIONS
// ─────────────────────────────────────────────────────────────────────────────

fn apply_contractions(text: &str) -> String {
    let contractions: &[(&str, &str)] = &[
        ("it is",    "'tis"),
        ("it was",   "'twas"),
        ("it will",  "'twill"),
        ("it would", "'twould"),
        ("it were",  "'twere"),
        ("do not",   "dost not"),
        ("does not", "doth not"),
        ("did not",  "didst not"),
        ("will not", "wilt not"),
        ("cannot",   "canst not"),
        ("can not",  "canst not"),
        ("over",     "o'er"),
        ("never",    "ne'er"),
        ("ever",     "e'er"),
        ("them",     "'em"),
    ];

    let mut result = text.to_string();
    for (modern, shakes) in contractions {
        result = replace_whole_phrase_ci(&result, modern, shakes);
    }
    result
}

fn replace_whole_phrase_ci(text: &str, pattern: &str, replacement: &str) -> String {
    let lower = text.to_lowercase();
    let pat_len = pattern.len();
    let mut new = String::with_capacity(text.len());
    let mut last = 0;

    while let Some(rel) = lower[last..].find(pattern) {
        let abs = last + rel;

        let before_ok = abs == 0
            || !text[..abs]
                .chars()
                .last()
                .map(|c| c.is_alphanumeric())
                .unwrap_or(false);
        let after_ok = abs + pat_len >= text.len()
            || !text[abs + pat_len..]
                .chars()
                .next()
                .map(|c| c.is_alphanumeric())
                .unwrap_or(false);

        if before_ok && after_ok {
            new.push_str(&text[last..abs]);
            let is_cap = text[abs..].chars().next().map(|c| c.is_uppercase()).unwrap_or(false);
            let rep = if is_cap {
                capitalise_first(replacement)
            } else {
                replacement.to_string()
            };
            new.push_str(&rep);
            last = abs + pat_len;
        } else {
            new.push_str(&text[last..abs + 1]);
            last = abs + 1;
        }
    }

    new.push_str(&text[last..]);
    new
}

// ─────────────────────────────────────────────────────────────────────────────
// 9. CAPITALISATION
// ─────────────────────────────────────────────────────────────────────────────

fn fix_capitalisation(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut capitalise_next = true;

    for ch in text.chars() {
        if capitalise_next && ch.is_alphabetic() {
            result.extend(ch.to_uppercase());
            capitalise_next = false;
        } else {
            result.push(ch);
            if matches!(ch, '.' | '!' | '?') {
                capitalise_next = true;
            }
        }
    }

    fix_standalone_i(&result)
}

fn fix_standalone_i(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();
    let mut result = String::with_capacity(n);

    for i in 0..n {
        let ch = chars[i];
        let prev_boundary = i == 0 || !chars[i - 1].is_alphabetic();
        let next_boundary = i + 1 >= n || !chars[i + 1].is_alphabetic();

        if (ch == 'i' || ch == 'I') && prev_boundary && next_boundary {
            result.push('I');
        } else {
            result.push(ch);
        }
    }

    result
}

// ─────────────────────────────────────────────────────────────────────────────
// HELPERS
// ─────────────────────────────────────────────────────────────────────────────

fn capitalise_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn preserve_case(original: &str, target: &str) -> String {
    let is_upper = original
        .chars()
        .next()
        .map(|c| c.is_uppercase())
        .unwrap_or(false);
    if is_upper {
        capitalise_first(target)
    } else {
        target.to_string()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MAIN PIPELINE
// ─────────────────────────────────────────────────────────────────────────────

pub fn translate(input: &str) -> String {
    // ── Stage 0: expand modern contractions ──────────────────────────────────
    let expanded = expand_contractions(input);

    // ── Stage 1: tokenize ────────────────────────────────────────────────────
    let raw_tokens = tokenize(&expanded);

    // ── Stage 2: POS-tag word tokens ─────────────────────────────────────────
    let words: Vec<&str> = raw_tokens
        .iter()
        .filter_map(|t| {
            if let Token::Word(w) = t {
                Some(w.as_str())
            } else {
                None
            }
        })
        .collect();
    let tags = pos_tag(&words);

    // Zip each token with its POS (non-word tokens get Pos::Other)
    let mut tagged_tokens: Vec<(Token, Pos)> = {
        let mut tag_iter = tags.into_iter();
        raw_tokens
            .into_iter()
            .map(|tok| {
                let pos = if matches!(tok, Token::Word(_)) {
                    tag_iter.next().unwrap_or(Pos::Other)
                } else {
                    Pos::Other
                };
                (tok, pos)
            })
            .collect()
    };

    // ── Stages 3-6: per-token transformations ─────────────────────────────────
    // Pre-compute word-token indices for next-word lookahead.
    let word_indices: Vec<usize> = tagged_tokens
        .iter()
        .enumerate()
        .filter_map(|(i, (tok, _))| if matches!(tok, Token::Word(_)) { Some(i) } else { None })
        .collect();

    let wi_pos_map: std::collections::HashMap<usize, usize> = word_indices
        .iter()
        .enumerate()
        .map(|(pos_in_wi, &tok_idx)| (tok_idx, pos_in_wi))
        .collect();

    let mut prev_word_lower = String::new();
    let n = tagged_tokens.len();

    for tok_idx in 0..n {
        if !matches!(tagged_tokens[tok_idx].0, Token::Word(_)) {
            continue;
        }

        let replacement = {
            let (tok, pos) = &tagged_tokens[tok_idx];
            let original = if let Token::Word(w) = tok { w.as_str() } else { unreachable!() };
            let lower = original.to_lowercase();

            // Lookahead: word immediately after this one
            let next_word = wi_pos_map
                .get(&tok_idx)
                .and_then(|&wi| word_indices.get(wi + 1))
                .and_then(|&next_idx| {
                    if let Token::Word(w) = &tagged_tokens[next_idx].0 {
                        Some(w.as_str())
                    } else {
                        None
                    }
                });

            // Stage 3: pronoun substitution
            if let Some(sub) = substitute_pronouns(original, pos, next_word) {
                sub
            }
            // Stage 4/5: verb conjugation
            else if prev_word_lower == "thou" && matches!(pos, Pos::Verb | Pos::Auxiliary) {
                conjugate_for_thou(original)
            }
            // Conjugate linking verbs that *precede* you/thee (e.g. "How are you?")
            else if matches!(lower.as_str(), "are" | "is" | "was" | "were" | "do" | "does" | "did")
                && matches!(next_word, Some(w) if matches!(w.to_lowercase().as_str(), "you" | "thou" | "thee"))
            {
                conjugate_for_thou(original)
            } else if matches!(prev_word_lower.as_str(), "he" | "she" | "it" | "one")
                && matches!(pos, Pos::Verb | Pos::Auxiliary)
            {
                conjugate_third_person(original)
            }
            // Stage 6: vocabulary substitution
            else if let Some(sub) = vocabulary_substitution(original) {
                sub
            } else {
                original.to_string()
            }
        };

        prev_word_lower = replacement.to_lowercase();
        tagged_tokens[tok_idx].0 = Token::Word(replacement);
    }

    // ── Stage 7: fix article agreement ───────────────────────────────────────
    fix_articles(&mut tagged_tokens);

    // ── Rebuild string ────────────────────────────────────────────────────────
    let joined: String = tagged_tokens
        .iter()
        .map(|(tok, _)| match tok {
            Token::Word(w) => w.as_str(),
            Token::Punctuation(p) => p.as_str(),
            Token::Whitespace(ws) => ws.as_str(),
        })
        .collect();

    // ── Stage 8: Shakespearean contractions ───────────────────────────────────
    let contracted = apply_contractions(&joined);

    // ── Stage 9: capitalisation ───────────────────────────────────────────────
    fix_capitalisation(&contracted)
}

// ─────────────────────────────────────────────────────────────────────────────
// TESTS
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Stage 0 – contraction expansion
    #[test]
    fn test_expand_youre() {
        let result = translate("You're my friend.");
        assert!(result.to_lowercase().contains("thou art"), "got: {result}");
    }

    #[test]
    fn test_expand_dont() {
        let result = translate("Don't go.");
        assert!(
            result.to_lowercase().contains("dost not"),
            "got: {result}"
        );
    }

    #[test]
    fn test_expand_its() {
        let result = translate("It's a fine day.");
        assert!(result.to_lowercase().contains("'tis"), "got: {result}");
    }

    // Stage 3 – pronoun substitution
    #[test]
    fn test_pronoun_subject() {
        let result = translate("You are brave.");
        assert!(result.to_lowercase().contains("thou"), "got: {result}");
    }

    #[test]
    fn test_pronoun_object() {
        let result = translate("I see you.");
        assert!(result.to_lowercase().contains("thee"), "got: {result}");
    }

    #[test]
    fn test_possessive_consonant() {
        // "your sword" → "thy sword"  (consonant-initial)
        let result = translate("Your sword is sharp.");
        assert!(result.to_lowercase().contains("thy"), "got: {result}");
    }

    #[test]
    fn test_possessive_vowel() {
        // "your enemy" → "thine enemy"  (vowel-initial)
        let result = translate("Your enemy is near.");
        assert!(result.to_lowercase().contains("thine"), "got: {result}");
    }

    // Stages 4/5 – conjugation
    #[test]
    fn test_verb_conjugation_thou() {
        let result = translate("You speak well.");
        assert!(
            result.to_lowercase().contains("speakest"),
            "got: {result}"
        );
    }

    #[test]
    fn test_irregular_hath() {
        let result = translate("She has spoken.");
        assert!(result.to_lowercase().contains("hath"), "got: {result}");
    }

    #[test]
    fn test_irregular_art() {
        let result = translate("You are my enemy.");
        assert!(result.to_lowercase().contains("art"), "got: {result}");
    }

    // Stage 6 – vocabulary
    #[test]
    fn test_vocab_foe() {
        let result = translate("He is my enemy.");
        assert!(result.to_lowercase().contains("foe"), "got: {result}");
    }

    #[test]
    fn test_vocab_ere() {
        let result = translate("Think before you speak.");
        assert!(result.to_lowercase().contains("ere"), "got: {result}");
    }

    // Stage 7 – article agreement
    #[test]
    fn test_article_a_before_consonant() {
        // "an enemy" → vocab replaces "enemy" with "foe" → article should become "a"
        let result = translate("He is an enemy.");
        assert!(
            result.to_lowercase().contains("a foe"),
            "got: {result}"
        );
    }

    // Stage 8 – contractions
    #[test]
    fn test_contraction_tis() {
        let result = translate("It is a fine day.");
        assert!(result.to_lowercase().contains("'tis"), "got: {result}");
    }

    #[test]
    fn test_contraction_twas() {
        let result = translate("It was a dark night.");
        assert!(result.to_lowercase().contains("'twas"), "got: {result}");
    }

    // Stage 9 – capitalisation
    #[test]
    fn test_sentence_start_capitalised() {
        let result = translate("you are brave.");
        assert!(result.starts_with('T'), "got: {result}"); // "Thou"
    }

    #[test]
    fn test_pronoun_i_capitalised() {
        let result = translate("she and i are friends.");
        assert!(result.contains(" I "), "got: {result}");
    }

    // Full sentence smoke tests
    #[test]
    fn test_full_sentence_contraction() {
        let result = translate("You're my enemy and it's your fault.");
        println!("Full: {result}");
        assert!(result.to_lowercase().contains("thou art"), "got: {result}");
        assert!(result.to_lowercase().contains("'tis"), "got: {result}");
    }

    #[test]
    fn test_full_sentence_complex() {
        let result = translate("Don't be afraid, because you are very brave.");
        println!("Complex: {result}");
    }

    #[test]
    fn test_how_are_you() {
        let result = translate("How are you?");
        assert!(result.to_lowercase().contains("art"), "got: {result}");
        assert!(result.to_lowercase().contains("thee"), "got: {result}");
    }

    #[test]
    fn test_where_are_you() {
        let result = translate("Where are you?");
        assert!(result.to_lowercase().contains("art"), "got: {result}");
        assert!(result.to_lowercase().contains("thee"), "got: {result}");
    }
}
