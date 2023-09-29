use serde_json::json;
use std::fs::File;
use std::io::{Read, Write};

fn generate_tests() -> String {
    // Open the JSON file
    let mut file = File::open("test.json").expect("File not found");

    // Read the contents of the file into a string
    let mut json_string = String::new();
    file.read_to_string(&mut json_string)
        .expect("Error reading file");

    let mut test_code: String = String::new();

    // Parse the JSON string into a Rust data structure
    let json_objects: Vec<serde_json::Value> =
        serde_json::from_str(&json_string).expect("Error parsing JSON");

    // Generate the Rust code for tests
    for (index, json_obj) in json_objects.iter().enumerate() {
        if index == 100 {
            break;
        }
        let _name = &json_obj["name"].as_str().unwrap();
        let name: Vec<&str> = _name.split(" ").collect();
        let initial = &json_obj["initial"];
        let _final = &json_obj["final"];

        test_code.push_str(&format!(
            "#[test]\nfn test_{}_{}_{}_{}() {{\n",
            name[0], name[1], name[2], index
        ));

        test_code.push_str(&format!(
            "    let mut cpu = Cpu::new();\n    cpu.load(&vec![0x{}, 0x{}, 0x{}]);\n",
            name[0], name[1], name[2]
        ));

        for (key, value) in initial.as_object().unwrap() {
            let mut k = key.clone();
            if key != "ram" {
                let mut prefix: &str = "register_";
                if key == "p" {
                    prefix = "";
                    k = "status".to_string();
                } else if key == "pc" {
                    test_code.push_str(&format!("    cpu.pc = 0x8000;\n"));
                    continue;
                }
                test_code.push_str(&format!("    cpu.{}{} = {};\n", prefix, k, value));
            }
        }

        test_code.push_str("\n    cpu.run();\n\n");

        for (key, value) in _final.as_object().unwrap() {
            if key == "ram" {
                continue;
            }
            let mut prefix: &str = "register_";
            let mut k = key.clone();
            if key == "p" {
                prefix = "";
                k = "status".to_string();
            } else if key == "pc" {
                let a = "32768".parse::<u16>().unwrap() + name.len() as u16;
                test_code.push_str(&format!("    assert_eq!(cpu.pc, {:#8x});\n", a));
                continue;
            }
            test_code.push_str(&format!(
                "    assert_eq!(cpu.{}{}, {});\n",
                prefix, k, value
            ));
        }

        test_code.push_str("}\n\n");
    }

    test_code
}

fn main() {
    let test_code = generate_tests();

    let mut file = File::create("generated_tests.rs").expect("Unable to create file");
    file.write_all(test_code.as_bytes())
        .expect("Unable to write to file");

    println!("Generated tests written to 'generated_tests.rs'");
}
