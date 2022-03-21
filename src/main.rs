use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    // vector
    let mut v1 = vec![10, 4, 8, 9, 32];
    let mut v2 = vec![10, 4, 8, 9, 32, 5];
    let result1 = calc_median(&mut v1);
    let result2 = calc_median(&mut v2);
    println!("{}", *result1);
    println!("{}", *result2);

    // string
    convert_strings_to_pig_latin("first apple");

    // vector & hash map
    let mut employee_map: HashMap<String, Vec<String>> = HashMap::new();
    add_employee_to_department(&mut employee_map, "Engineering".to_string(), "Sally".to_string());
    add_employee_to_department(&mut employee_map, "Sales".to_string(), "Amir".to_string());
    add_employee_to_department(&mut employee_map, "Sales".to_string(), "Bob".to_string());

    println!("{:?}", &employee_map);
}

fn calc_median(vector: &mut Vec<i32>) -> &i32 {
    vector.sort();
    let length = vector.len();
    &vector[length / 2]
}

fn convert_strings_to_pig_latin(text: &str) -> () {
    let mut result = String::new();
    for word in text.split_whitespace() {
        let converted_word = match &word[0..1] {
            "a" | "i" | "u" | "e" | "o" => format!("{}-hay", &word),
            _ => format!("{}{}ay", &word[1..], &word[0..1]),
        };
        result.push_str(format!(" {}", converted_word.as_str()).as_str());
    }
    println!("{}", result.trim());
}

fn add_employee_to_department(map: &mut HashMap<String, Vec<String>>, department: String, employee: String) -> &mut HashMap<String, Vec<String>>{
    let employees: &mut Vec<String> = map.entry(department).or_insert(Vec::new());

    employees.push(employee);

    map
}
