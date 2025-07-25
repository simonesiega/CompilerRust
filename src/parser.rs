use crate::ast::OperatorDef;

pub fn parse_operator_file(contents: &str) -> Vec<OperatorDef> {
    let mut operators = Vec::new();

    let re = regex::Regex::new(r"operator\s+(\w+)\[.*\]\((.*?)\)\s*->\s*(\w+)\s*\{([\s\S]*?)\}").unwrap();

    for cap in re.captures_iter(contents) {
        let name = cap[1].trim().to_string();
        let params = cap[2].split(',').map(|s| s.trim().to_string()).collect();
        let ret = cap[3].trim().to_string();
        let body = cap[4].trim().to_string();

        operators.push(OperatorDef {
            name,
            params,
            return_type: ret,
            body,
        });
    }

    operators
}