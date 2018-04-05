use super::errors::*;

#[derive(Debug)]
pub enum Codes {
    G(u8),
    M(u8),
    T(u8),
    X(f32),
    Y(f32),
    Z(f32),
    I(f32),
    J(f32),
    K(f32),
    F(f32),
}

fn create_code(code: char, acc: &String) -> Result<Option<Codes>> {
    let acc = acc.trim();
    match code {
        'G' => {
            let value = acc.parse()
                .chain_err(|| "Error parsing value")?;
            Ok(Some(Codes::G(value)))
        },
        'M' => {
            let value = acc.parse()
                .chain_err(|| "Error parsing value")?;
            Ok(Some(Codes::M(value)))
        },
        'T' => {
            let value = acc.parse()
                .chain_err(|| "Error parsing value")?;
            Ok(Some(Codes::T(value)))
        },
        'X' => {
            let value = acc.parse()
                .chain_err(|| "Error parsing value")?;
            Ok(Some(Codes::X(value)))
        },
        'Y' => {
            let value = acc.parse()
                .chain_err(|| "Error parsing value")?;
            Ok(Some(Codes::Y(value)))
        },
        'Z' => {
            let value = acc.parse()
                .chain_err(|| "Error parsing value")?;
            Ok(Some(Codes::Z(value)))
        },
        'I' => {
            let value = acc.parse()
                .chain_err(|| "Error parsing value")?;
            Ok(Some(Codes::I(value)))
        },
        'J' => {
            let value = acc.parse()
                .chain_err(|| "Error parsing value")?;
            Ok(Some(Codes::J(value)))
        },
        'K' => {
            let value = acc.parse()
                .chain_err(|| "Error parsing value")?;
            Ok(Some(Codes::K(value)))
        },
        'F' => {
            let value = acc.parse()
                .chain_err(|| "Error parsing value")?;
            Ok(Some(Codes::F(value)))
        },
        _   => Ok(None),
    }
}


fn process_letter(cl: &Option<char>, acc: &String, mut vec: Vec<Codes>) -> Result<Vec<Codes>> {
    if let &Some(letter) = cl {
        if acc.is_empty() {
            bail!("Syntax error");
        }
        if let Some(code) = create_code(letter, &acc)
            .chain_err(|| "Error creating code")? {
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
