use std::collections::HashMap;

const DELIMITERS: [char; 6] = ['{', '}', '[', ']', ':', ','];
static mut SUM: isize = 0;

#[derive(Debug, PartialEq)]
enum Json {
    Obj(HashMap<String, Json>), // object keys always seem to be strings
    Arr(Vec<Json>),
    String(String),
    Num(isize),
}

fn sum_nums(json: Json) -> isize {
    match json {
        Json::Obj(map) => do_obj(map),
        Json::Arr(arr) => do_arr(arr),
        Json::String(_) => 0,
        Json::Num(n) => n,
    }
}

fn do_obj(obj: HashMap<String, Json>) -> isize {
    // check if obj contains "red" as value
    if obj.iter().find(|(_, v)| v == &&Json::String("red".to_string())).is_some() {
        return 0;
    }
    let mut sum = 0;
    for (_, v) in obj {
        sum += match v {
            Json::Obj(obj) => do_obj(obj),
            Json::Arr(arr) => do_arr(arr),
            Json::String(_) => 0,
            Json::Num(n) => n,
        };
    }

    sum
}

fn do_arr(arr: Vec<Json>) -> isize {
    let mut sum = 0; 
    for item in arr {
        sum += match item {
            Json::Obj(obj) => do_obj(obj),
            Json::Arr(arr) => do_arr(arr),
            Json::String(_) => 0,
            Json::Num(n) => n,
        };
    }
    sum
}

struct JsonParser {
    json_str: &'static str, 
    cursor: usize,
}

// All inputs are expected to be ascii
impl JsonParser {
    fn new(json_str: &'static str) -> Self {
        JsonParser { json_str, cursor: 0 } 
    }

    fn parse(&mut self) -> Json {
        match &self.json_str[..1] {
            "{" => Json::Obj(self.parse_obj()),
            "[" => Json::Arr(self.parse_arr()),
            "\"" => Json::String(self.parse_string()),
            _ => Json::Num(self.parse_num()),
        }
    }

    // expects input like "\"some_string\"
    fn parse_string(&mut self) -> String {
        self.move_cursor(1); // move cursor past the first "\""

        let len = self.json_str[self.cursor..].find("\"").unwrap();
        let string = String::from(&self.json_str[self.cursor..self.cursor + len]);

        self.move_cursor(len + 1); // move cursor to first char after \"

        String::from(string)
    }

    // expects input to be starting from the first digit
    fn parse_num(&mut self) -> isize {
        let len = self.json_str[self.cursor..].find(DELIMITERS).unwrap(); 
        let num = self.json_str[self.cursor..self.cursor + len].parse::<isize>().unwrap();

        unsafe { SUM += num }; // for part 1

        self.move_cursor(len); // move cursor to next delimiter

        num
    }

    fn parse_obj(&mut self) -> HashMap<String, Json> {
        let mut obj = HashMap::new();

        self.move_cursor(1); // move cursor past starting '{'

        while self.cursor < self.json_str.len() {
            // json objects are alway string, value pairs
            let key = self.parse_string();

            self.move_cursor(1); // move cursor past ':'

            match &self.json_str[self.cursor..self.cursor + 1] {
                "[" => obj.insert(key, Json::Arr(self.parse_arr())),
                "{" => obj.insert(key, Json::Obj(self.parse_obj())),
                "\"" => obj.insert(key, Json::String(self.parse_string())),
                _ => obj.insert(key, Json::Num(self.parse_num())), 
            };
            
            // check if parsing is complete
            if &self.json_str[self.cursor..self.cursor + 1] == "}" {
                break;
            }
            self.move_cursor(1); // if parsing is not complete, move cursor past ','
        }

        self.move_cursor(1); // move cursor past '}'

        obj
    }

    // expects input to start at '['
    fn parse_arr(&mut self) -> Vec<Json> {
        let mut arr = Vec::new();

        self.move_cursor(1); // move cursor past starting '['

        while self.cursor < self.json_str.len() {
            match &self.json_str[self.cursor..self.cursor + 1] {
                "[" => arr.push(Json::Arr(self.parse_arr())),
                "]" => break,
                "," => self.move_cursor(1), // go to next item
                "{" => arr.push(Json::Obj(self.parse_obj())),
                "\"" => arr.push(Json::String(self.parse_string())),
                _ => arr.push(Json::Num(self.parse_num())), 
            }
        }

        self.move_cursor(1); // move cursor past closing ']'

        arr
    }

    #[inline(always)]
    fn move_cursor(&mut self, steps: usize) {
        self.cursor += steps;
    }
}

pub fn get_solution_1() -> isize {
    let mut p = JsonParser::new(include_str!("../data/d12.txt"));

    let _ = p.parse();

    unsafe { SUM }
}

pub fn get_solution_2() -> isize {
    let mut p = JsonParser::new(include_str!("../data/d12.txt"));
    let json = p.parse();
    sum_nums(json)
}

#[test]
fn test_parse_string() {
    let input = "\"Henlo\"";
    let mut p = JsonParser::new(input);
    assert_eq!("Henlo".to_string(), p.parse_string());

    let input = "\"henlo\"blablablagewafewiofa32r231a";
    let mut p = JsonParser::new(input);
    assert_eq!("henlo".to_string(), p.parse_string());
}

#[test]
fn test_parse_num() {
    let input = "123,";
    let mut p = JsonParser::new(input);
    assert_eq!(123, p.parse_num());

    let input = "154]";
    let mut p = JsonParser::new(input);
    assert_eq!(154, p.parse_num());
}

#[test]
fn test_parse_arr() {
    let input = "[1,2,\"henlo\"]"; // only nums and strings
    let mut p = JsonParser::new(input);
    let expected = vec![Json::Num(1), Json::Num(2), Json::String("henlo".to_string())];
    assert_eq!(p.parse_arr(), expected);

    let input = "[1,2,[\"henlo\",3,4]]";    // nested array
    let mut p = JsonParser::new(input);
    let expected = vec![Json::Num(1), Json::Num(2),Json::Arr(vec![Json::String("henlo".to_string()),Json::Num(3),Json::Num(4)])];
    assert_eq!(p.parse_arr(), expected);

    let input = "[1,2,[\"henlo\",3,4],5,\"six\"]";    // nested array
    let mut p = JsonParser::new(input);
    let expected = vec![Json::Num(1), Json::Num(2),Json::Arr(vec![Json::String("henlo".to_string()),Json::Num(3),Json::Num(4)]),Json::Num(5), Json::String("six".to_string())];
    assert_eq!(p.parse_arr(), expected);

    let input = "[[1]]";
    let mut p = JsonParser::new(input);
    let expected = vec![Json::Arr(vec![Json::Num(1)])];
    assert_eq!(p.parse_arr(), expected);

}

#[test]
fn test_parse_obj() {
    let input = "{\"a\":1,\"b\":\"hello\"}";
    let mut p = JsonParser::new(input);
    let expected = HashMap::from([
        ("a".to_string(), Json::Num(1)),
        ("b".to_string(), Json::String("hello".to_string())),
    ]);
    assert_eq!(p.parse_obj(), expected);
}