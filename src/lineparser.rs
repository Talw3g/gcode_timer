use super::errors::*;

#[derive(Debug)]
pub enum Codes {
    G(u8),
    M(u8),
    X(f32),
    Y(f32),
    Z(f32),
    I(f32),
    J(f32),
    K(f32),
    F(f32),
}

fn create_code(code: char, acc: &String) -> Option<Codes> {
    let acc = acc.trim();
    match code {
        'G' => {
            let value = acc.parse().unwrap();
            Some(Codes::G(value))
        },
        'X' => {
            let value = acc.parse().unwrap();
            Some(Codes::X(value))
        },
        'Y' => {
            let value = acc.parse().unwrap();
            Some(Codes::Y(value))
        },
        'Z' => {
            let value = acc.parse().unwrap();
            Some(Codes::Z(value))
        },
        'I' => {
            let value = acc.parse().unwrap();
            Some(Codes::I(value))
        },
        'J' => {
            let value = acc.parse().unwrap();
            Some(Codes::J(value))
        },
        'K' => {
            let value = acc.parse().unwrap();
            Some(Codes::K(value))
        },
        'F' => {
            let value = acc.parse().unwrap();
            Some(Codes::F(value))
        },
        'M' => {
            let value = acc.parse().unwrap();
            Some(Codes::M(value))
        },
        _   => None,
    }
}


fn process_letter(cl: &Option<char>, acc: &String, mut vec: Vec<Codes>) -> Result<Vec<Codes>> {
    if let &Some(letter) = cl {
        if acc.is_empty() {
            bail!("Syntax error");
        }
        if let Some(code) = create_code(letter, &acc) {
            vec.push(code);
        }
    }
    Ok(vec)
}


pub fn parse_line(line: String) -> Result<Option<Vec<Codes>>> {
    let mut line_codes: Vec<Codes> = Vec::new();
    let mut acc = String::new();
    let mut current_letter = None;

    if line.starts_with("(") {
        return Ok(None)
    }

    for item in line.chars() {
        if item.is_alphabetic() {
            line_codes = process_letter(&current_letter, &acc, line_codes)
                .chain_err(|| "Error processing letter")?;
            current_letter = Some(item.to_ascii_uppercase());
            acc.clear();
        }
        else {
            acc.push(item);
        }
    }

    line_codes = process_letter(&current_letter, &acc, line_codes)
        .chain_err(|| "Error processing letter")?;

    if line_codes.is_empty() {
        Ok(None)
    }
    else {
        Ok(Some(line_codes))
    }
}
