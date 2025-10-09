use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

struct Translator {
    properties: HashMap<&'static str, &'static str>,
    relationships: HashMap<&'static str, &'static str>,
    theorems: HashMap<&'static str, &'static str>,
    constants: HashMap<&'static str, &'static str>,
}

impl Translator {
    fn new() -> Self {
        let mut properties = HashMap::new();
        properties.insert("R", "regular");
        properties.insert("CV", "convex");
        properties.insert("CC", "concave");
        properties.insert("RT", "right");
        properties.insert("OB", "obtuse");
        properties.insert("AC", "acute");
        properties.insert("SC", "scalene");
        properties.insert("IS", "isosceles");
        properties.insert("TR", "trapezoid");
        properties.insert("PL", "parallelogram");
        properties.insert("EQ", "equilateral");
        properties.insert("EA", "equiangular");
        properties.insert("C", "cyclic");
        properties.insert("TP", "tangential");

        let mut relationships = HashMap::new();
        relationships.insert("S", "collinear");
        relationships.insert("P", "parallel");
        relationships.insert("PR", "perpendicular");
        relationships.insert("CG", "congruent");
        relationships.insert("SM", "similar");

        let mut theorems = HashMap::new();
        theorems.insert("_TI", "Triangle Inequality");
        theorems.insert("_ST", "Stewart's Theorem");
        theorems.insert("_AT", "Apollonius Theorem");
        theorems.insert("_VT", "Viviani's Theorem");
        theorems.insert("_NP", "Napoleon's Theorem");
        theorems.insert("_EL", "Euler Line");
        theorems.insert("_9C", "Nine-Point Circle");
        theorems.insert("_SL", "Simson Line");
        theorems.insert("_CV", "Ceva's Theorem");
        theorems.insert("_ML", "Menelaus' Theorem");
        theorems.insert("_AB", "Angle Bisector Theorem");
        theorems.insert("_IE", "Incenter-Excenter Lemma");
        theorems.insert("_CT", "Carnot's Theorem");
        theorems.insert("_M", "Miquel's Theorem");
        theorems.insert("_ET", "Euler's Theorem");
        theorems.insert("_DT", "Desargue's Theorem");
        theorems.insert("_HF", "Heron's Formula");
        theorems.insert("_QF", "Bretschinder's Formula");
        theorems.insert("_BF", "Brahmagupta's Formula");
        theorems.insert("_JT", "Japanese Theorem");
        theorems.insert("_NT", "Newton's Theorem");
        theorems.insert("_PT", "Ptolemy's Theorem");
        theorems.insert("_PP", "Power of a Point Theorem");
        theorems.insert("_BT", "Butterfly Theorem");
        theorems.insert("_PC", "Pascal's Theorem");
        theorems.insert("_LC", "Law of Cosines");
        theorems.insert("_LS", "Law of Sines");
        theorems.insert("_LT", "Law of Tangents");
        theorems.insert("_PK", "Pick's Theorem");
        theorems.insert("_SH", "Shoelace Theorem");
        theorems.insert("_SSC", "SSS Congruence");
        theorems.insert("_SAC", "SAS Congruence");
        theorems.insert("_SSA", "SSA Congruence");
        theorems.insert("_ASA", "ASA Congruence");
        theorems.insert("_AAS", "AAS Congruence");
        theorems.insert("_HL", "HL Congruence");
        theorems.insert("_AA", "AA Similarity");
        theorems.insert("_SAS", "SAS Similarity");
        theorems.insert("_SSS", "SSS Similarity");

        let mut constants = HashMap::new();
        constants.insert("\\T", "τ (tau)");
        constants.insert("\\P", "π (pi)");
        constants.insert("\\G", "φ (phi)");

        Translator {
            properties,
            relationships,
            theorems,
            constants,
        }
    }

    fn translate_statement(&self, stmt: &str) -> String {
        let stmt = stmt.trim();
        if stmt.is_empty() {
            return String::new();
        }

        // Handle proof markers
        if stmt.starts_with("\\p:") {
            return format!("We will prove: {}", self.translate_statement(&stmt[3..]));
        }
        if stmt.starts_with("\\pC:") {
            return format!(
                "We will prove by contradiction: {}",
                self.translate_statement(&stmt[4..])
            );
        }
        if stmt == "\\q" || stmt == "□" {
            return "And that is what was to be shown.".to_string();
        }
        if stmt == "\\qC" || stmt == "↯" {
            return "Achieving a contradiction.".to_string();
        }
        if stmt == "\\bc" || stmt == "∵" {
            return "Because".to_string();
        }
        if stmt == "\\th" || stmt == "∴" {
            return "Therefore".to_string();
        }

        // Handle casework
        if stmt.contains("<<") || stmt.contains(">>") {
            if stmt.contains("<<") && stmt.contains(">>") {
                // Full casework block
                if let Some(main_start) = stmt.find("<<") {
                    let main_stmt = &stmt[..main_start];
                    return format!("{}\nBegin casework.", self.translate_statement(main_stmt));
                }
            }
            if stmt.contains("<<") {
                return "Begin casework analysis.".to_string();
            }
            if stmt.contains("\\>>") || stmt.contains(">>") {
                return "End casework.".to_string();
            }
        }

        // Graph construction
        if stmt.starts_with("G:") {
            let eq = stmt[2..].trim().trim_matches(|c| c == '{' || c == '}');
            return format!("Graph the function {}.", eq);
        }

        // Point construction
        if stmt.starts_with("P:") {
            let rest = &stmt[2..];

            // Multiple points
            if rest.contains(',') && !rest.contains('.') && !rest.contains('|') {
                return format!("Construct points {}.", rest);
            }

            // Intersection
            if rest.contains('x') && rest.contains('=') {
                if let Some(eq_pos) = rest.find('=') {
                    let point = &rest[..eq_pos];
                    let intersection = &rest[eq_pos + 1..];
                    if let Some(x_pos) = intersection.find('x') {
                        let obj1 = &intersection[..x_pos];
                        let obj2 = &intersection[x_pos + 1..];
                        return format!(
                            "Let point {} be the intersection of {} and {}.",
                            point, obj1, obj2
                        );
                    }
                }
            }

            // Point on object with conditions
            if rest.contains('.') && rest.contains('|') {
                if let Some(pipe_pos) = rest.find('|') {
                    let point_obj = &rest[..pipe_pos];
                    let conditions = &rest[pipe_pos + 1..];

                    if let Some(dot_pos) = point_obj.find('.') {
                        let point = &point_obj[..dot_pos];
                        let obj = &point_obj[dot_pos + 1..];

                        let cond_parts: Vec<&str> = conditions.split(',').collect();
                        let cond_strs: Vec<String> = cond_parts
                            .iter()
                            .map(|c| {
                                let c = c.trim();
                                if c.starts_with("R:") {
                                    self.translate_regular_polygon(&c[2..])
                                } else if c.contains('[') && c.contains(']') && c.contains('=') {
                                    if let Some(start) = c.find('[') {
                                        if let Some(end) = c.find(']') {
                                            let obj = &c[start + 1..end];
                                            let val = &c[end + 2..];
                                            format!("the area of {} is {}", obj, val)
                                        } else {
                                            c.to_string()
                                        }
                                    } else {
                                        c.to_string()
                                    }
                                } else if c.contains('*') {
                                    let parts: Vec<&str> = c.split('*').collect();
                                    if parts.len() == 2 {
                                        let prop_name = self
                                            .properties
                                            .get(parts[1])
                                            .or_else(|| self.relationships.get(parts[1]))
                                            .unwrap_or(&parts[1]);
                                        format!("{} is {}", parts[0], prop_name)
                                    } else {
                                        c.to_string()
                                    }
                                } else {
                                    c.to_string()
                                }
                            })
                            .collect();

                        return format!(
                            "Construct point {} on {} such that {}.",
                            point,
                            obj,
                            cond_strs.join(", ")
                        );
                    }
                }
            }

            // Point on object
            if rest.contains('.') {
                if let Some(dot_pos) = rest.find('.') {
                    let point = &rest[..dot_pos];
                    let obj = &rest[dot_pos + 1..];
                    return format!("Construct point {} on {}.", point, obj);
                }
            }

            return format!("Construct point {}.", rest);
        }

        // Segment
        if stmt.starts_with("S:") {
            return format!("Connect segment {}.", &stmt[2..]);
        }

        // Line
        if stmt.starts_with("L:") {
            return format!("Connect line {}.", &stmt[2..]);
        }

        // Ray
        if stmt.starts_with("W:") {
            return format!("Construct ray {}.", &stmt[2..]);
        }

        // Circle
        if stmt.starts_with("C:") {
            let rest = &stmt[2..];
            let parts: Vec<&str> = rest.split(';').collect();

            if parts.len() == 3 {
                return format!(
                    "Construct a circle through points {}, {}, and {}.",
                    parts[0], parts[1], parts[2]
                );
            }
            if parts.len() == 2 {
                if parts[1].chars().all(|c| c.is_numeric()) {
                    return format!(
                        "Construct a circle with center {} and radius {}.",
                        parts[0], parts[1]
                    );
                }
                return format!(
                    "Construct a circle with center {} passing through point {}.",
                    parts[0], parts[1]
                );
            }
        }

        // Polygon
        if stmt.starts_with("J:") {
            return format!("Construct polygon {}.", &stmt[2..]);
        }

        // Regular polygon
        if stmt.starts_with("R:") {
            return format!("Construct {}.", self.translate_regular_polygon(&stmt[2..]));
        }

        // Area
        if stmt.contains('[') && stmt.contains(']') {
            if let Some(start) = stmt.find('[') {
                if let Some(end) = stmt.find(']') {
                    let obj = &stmt[start + 1..end];
                    let rest = &stmt[end + 1..];
                    if rest.starts_with('=') {
                        return format!("Let the area of {} be {}.", obj, &rest[1..]);
                    }
                    if rest == "?" {
                        return format!("What is the area of {}?", obj);
                    }
                }
            }
        }

        // Perimeter
        if stmt.contains('(') && stmt.contains(')') {
            if let Some(start) = stmt.find('(') {
                if let Some(end) = stmt.find(')') {
                    let obj = &stmt[start + 1..end];
                    let rest = &stmt[end + 1..];
                    if rest.starts_with('=') {
                        return format!("Let the perimeter of {} be {}.", obj, &rest[1..]);
                    }
                    if rest == "?" {
                        return format!("What is the perimeter of {}?", obj);
                    }
                }
            }
        }

        // Angle
        if stmt.contains('<') || stmt.contains('∠') {
            let rest = stmt.replace('<', "").replace('∠', "");
            if rest.contains('=') {
                let parts: Vec<&str> = rest.split('=').collect();
                if parts.len() == 2 {
                    return format!("Angle {} measures {} degrees.", parts[0], parts[1]);
                }
            }
            if rest.ends_with('?') {
                return format!(
                    "What is the measure of angle {}?",
                    rest.trim_end_matches('?')
                );
            }
        }

        // Question
        if stmt.ends_with('?') {
            return format!("What is {}?", stmt.trim_end_matches('?'));
        }

        stmt.to_string()
    }

    fn translate_regular_polygon(&self, s: &str) -> String {
        let parts: Vec<&str> = s.split(';').collect();
        if parts.len() == 2 {
            if let Some(eq_pos) = parts[1].find('=') {
                let n = parts[0];
                let seg = &parts[1][..eq_pos];
                let poly = &parts[1][eq_pos + 1..];

                let shape = match n {
                    "3" => "equilateral triangle",
                    "4" => "square",
                    "5" => "regular pentagon",
                    "6" => "regular hexagon",
                    "8" => "regular octagon",
                    _ => return format!("regular {}-gon {} with side {}", n, poly, seg),
                };

                return format!("{} {} with side {}", shape, poly, seg);
            }
        }
        s.to_string()
    }

    fn translate(&self, input: &str) -> Vec<String> {
        let mut input = input.trim().to_string();

        // Remove outer markers
        if input.starts_with("\\\\") && input.ends_with("\\\\") {
            input = input[2..input.len() - 2].to_string();
        }

        // Split by /
        let statements: Vec<&str> = input
            .split('/')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        statements
            .iter()
            .map(|s| self.translate_statement(s))
            .collect()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: gslc <shorthand> [-o output.txt]");
        eprintln!("       gslc -f input.gsl [-o output.txt]");
        eprintln!("\nExamples:");
        eprintln!("  gslc \"\\\\P:A/P:B/S:AB\\\\\"");
        eprintln!("  gslc \"\\\\P:A/P:B/S:AB\\\\\" -o output.txt");
        eprintln!("  gslc -f problem.gsl -o solution.txt");
        std::process::exit(1);
    }

    let translator = Translator::new();
    let mut input = String::new();
    let mut output_file: Option<String> = None;

    // Parse arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-f" => {
                if i + 1 < args.len() {
                    i += 1;
                    let file_path = &args[i];
                    input = fs::read_to_string(file_path).unwrap_or_else(|e| {
                        eprintln!("Error reading file: {}", e);
                        std::process::exit(1);
                    });
                } else {
                    eprintln!("Error: -f requires a filename");
                    std::process::exit(1);
                }
            }
            "-o" => {
                if i + 1 < args.len() {
                    i += 1;
                    output_file = Some(args[i].clone());
                } else {
                    eprintln!("Error: -o requires a filename");
                    std::process::exit(1);
                }
            }
            _ => {
                if input.is_empty() {
                    input = args[i].clone();
                }
            }
        }
        i += 1;
    }

    // Translate
    let translations = translator.translate(&input);

    // Output
    if let Some(output_path) = output_file {
        let output = translations
            .iter()
            .enumerate()
            .map(|(i, t)| format!("{}. {}", i + 1, t))
            .collect::<Vec<_>>()
            .join("\n");

        fs::write(&output_path, output).unwrap_or_else(|e| {
            eprintln!("Error writing file: {}", e);
            std::process::exit(1);
        });

        println!("Translation written to: {}", output_path);
    } else {
        for (i, translation) in translations.iter().enumerate() {
            println!("{}. {}", i + 1, translation);
        }
    }
}
