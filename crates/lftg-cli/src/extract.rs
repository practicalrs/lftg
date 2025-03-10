pub fn extract_json(text: &str) -> String {
    let text = text.to_string();
    let lines = text.split('\n').collect::<Vec<&str>>();
    let mut code = String::new();
    let mut collect = false;

    for line in lines {
        let line = line.to_string();
        if line.starts_with("```") && collect {
            collect = false;
        }
        if collect {
            code.push_str(&format!("{line}\n"));
        }
        if line.starts_with("```json") && !collect {
            collect = true;
        }
    }

    code
}
