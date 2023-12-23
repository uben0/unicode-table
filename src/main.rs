use std::collections::HashSet;

mod table {
    use std::collections::HashSet;

    use pest::{iterators::Pair, Parser};

    #[derive(pest_derive::Parser)]
    #[grammar = "table.pest"]
    pub struct TableParser;

    #[derive(Debug)]
    pub struct Entry<'a> {
        code: u32,
        name: &'a str,
    }
    impl<'a> From<Pair<'a, Rule>> for Entry<'a> {
        fn from(pair: Pair<'a, Rule>) -> Self {
            let mut children = pair.into_inner();
            let code = u32::from_str_radix(children.next().unwrap().as_str(), 16).unwrap();
            let name = children.skip(1).next().unwrap().as_str();
            Self { code, name }
        }
    }

    pub fn parse_entries(content: &str) -> Vec<Entry> {
        let mut entries = TableParser::parse(Rule::file, content)
            .unwrap()
            .into_iter()
            .next()
            .unwrap()
            .into_inner();
        entries.next_back().unwrap();
        let mut table = Vec::new();
        for entry in entries {
            table.push(entry.into());
        }
        table
    }
    pub fn extract(entries: &[Entry], filter: impl Fn(&str) -> bool) -> HashSet<u32> {
        return entries
            .iter()
            .filter(|e| filter(e.name))
            .map(|e| e.code)
            .collect();
    }
}

mod list {
    use std::collections::HashSet;

    use pest::{iterators::Pair, Parser};

    #[derive(pest_derive::Parser)]
    #[grammar = "list.pest"]
    pub struct ListParser;

    #[derive(Debug)]
    pub struct Entry<'a> {
        begin: u32,
        end: u32,
        name: &'a str,
    }

    impl<'a> From<Pair<'a, Rule>> for Entry<'a> {
        fn from(pair: Pair<'a, Rule>) -> Self {
            let mut children = pair.into_inner();
            let begin = u32::from_str_radix(children.next().unwrap().as_str(), 16).unwrap();
            let name = children.next_back().unwrap().as_str();
            let end = children
                .next()
                .map(|p| u32::from_str_radix(p.as_str(), 16).unwrap())
                .unwrap_or(begin);
            Self { begin, end, name }
        }
    }

    pub fn parse_entries(content: &str) -> Vec<Entry> {
        let mut entries = ListParser::parse(Rule::file, content)
            .unwrap()
            .into_iter()
            .next()
            .unwrap()
            .into_inner();
        entries.next_back().unwrap();
        let mut table = Vec::new();
        for entry in entries {
            table.push(entry.into());
        }
        table
    }

    pub fn extract(entries: &[Entry], filter: impl Fn(&str) -> bool) -> HashSet<u32> {
        return entries
            .iter()
            .filter(|e| filter(e.name))
            .map(|e| e.begin..=e.end)
            .flatten()
            .collect();
    }
}

fn print_table<const N: usize, const M: usize>(
    positives: [HashSet<u32>; N],
    negatives: [HashSet<u32>; M],
    name: &str,
) {
    let negative: HashSet<_> = negatives
        .into_iter()
        .map(|s| s.into_iter())
        .flatten()
        .collect();
    let mut chars = HashSet::new();
    for positive in positives {
        chars.extend(positive.into_iter().filter(|e| !negative.contains(e)));
    }
    let mut chars: Vec<_> = chars.into_iter().collect();
    chars.sort_unstable();

    let ranges = match chars.as_slice() {
        [] => Vec::new(),
        [head, tail @ ..] => {
            let mut ranges = Vec::new();
            let mut start = *head;
            let mut end = *head;

            for &elem in tail {
                if end + 1 < elem {
                    ranges.push(start..=end);
                    start = elem;
                    end = elem;
                } else {
                    end = end.max(elem);
                }
            }
            ranges.push(start..=end);
            ranges
        }
    };

    println!("#define UCD_LEN_{} {}", name.to_uppercase(), ranges.len());
    println!(
        "static struct unicode_range ucd_table_{}[{}] = {{",
        name,
        ranges.len()
    );

    for range in &ranges {
        println!("    {{0x{:0>8x}, 0x{:0>8x}}},", range.start(), range.end());
    }
    println!("}};");
}

fn main() {
    let props = list::parse_entries(include_str!("PropList.txt"));
    let scripts = list::parse_entries(include_str!("Scripts.txt"));
    let categs = table::parse_entries(include_str!("UnicodeData.txt"));

    print_table(
        [list::extract(&props, |n| matches!(n, "XID_Start"))],
        [],
        "xid_start",
    );
    print_table(
        [list::extract(&props, |n| matches!(n, "XID_Continue"))],
        [],
        "xid_continue",
    );
    print_table(
        [
            list::extract(&props, |n| matches!(n, "Alphabetic")),
            table::extract(&categs, |n| matches!(n, "Nd" | "Nl" | "No")),
        ],
        [list::extract(&scripts, |n| {
            matches!(n, "Katakana" | "Hiragana" | "Han")
        })],
        "in_word",
    );
}
