use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

struct Translator {
    properties: HashMap<&'static str, &'static str>,
    relationships: HashMap<&'static str, &'static str>,
    theorems: HashMap<&'static str, &'static str>,
    constants: HashMap<&'static str, &'static str>,
    derived_constructions: HashMap<&'static str, &'static str>,
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
        relationships.insert("∥", "parallel");
        relationships.insert("PR", "perpendicular");
        relationships.insert("PD", "perpendicular");
        relationships.insert("⊥", "perpendicular");
        relationships.insert("TG", "tangent");
        relationships.insert("CG", "congruent");
        relationships.insert("≅", "congruent");
        relationships.insert("SM", "similar");
        relationships.insert("~", "similar");

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
        constants.insert("τ", "τ (tau)");
        constants.insert("\\P", "π (pi)");
        constants.insert("π", "π (pi)");
        constants.insert("\\G", "φ (phi)");
        constants.insert("φ", "φ (phi)");

        let mut derived_constructions = HashMap::new();
        derived_constructions.insert("PB", "perpendicular bisector");
        derived_constructions.insert("CCO", "circumcenter");
        derived_constructions.insert("CC", "circumcircle");
        derived_constructions.insert("AB", "angle bisector");
        derived_constructions.insert("ICO", "incenter");
        derived_constructions.insert("IC", "incircle");
        derived_constructions.insert("EAB", "exterior angle bisector");
        derived_constructions.insert("ECO", "excenter");
        derived_constructions.insert("EC", "excircle");
        derived_constructions.insert("M", "midpoint");
        derived_constructions.insert("MD", "median");
        derived_constructions.insert("CT", "centroid");
        derived_constructions.insert("PD", "perpendicular");
        derived_constructions.insert("OC", "orthocenter");
        derived_constructions.insert("PL", "parallel line");

        Translator {
            properties,
            relationships,
            theorems,
            constants,
            derived_constructions,
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

        // Handle logic operators
        if stmt.contains("||") || stmt.contains("∨") {
            return stmt.replace("||", " or ").replace("∨", " or ");
        }
        if stmt.contains("&&") || stmt.contains("∧") {
            return stmt.replace("&&", " and ").replace("∧", " and ");
        }
        if stmt.contains("=>") || stmt.contains("⊃") {
            return stmt.replace("=>", " implies ").replace("⊃", " implies ");
        }
        if stmt.starts_with("|A") || stmt.starts_with("∀") {
            return format!("For all {}", &stmt[2..]);
        }
        if stmt.starts_with("|E") || stmt.starts_with("∃") {
            return format!("There exists {}", &stmt[2..]);
        }

        // Handle casework
        if stmt.contains("<<") {
            if let Some(main_start) = stmt.find("<<") {
                let main_stmt = &stmt[..main_start];
                let case_part = &stmt[main_start + 2..];
                
                if case_part.contains(">>") {
                    let case_content = &case_part[..case_part.find(">>").unwrap_or(case_part.len())];
                    return format!(
                        "{}\nBegin casework: {}",
                        self.translate_statement(main_stmt),
                        self.translate_casework(case_content)
                    );
                }
                return format!("{}\nBegin casework.", self.translate_statement(main_stmt));
            }
        }
        if stmt.contains(">>") || stmt.contains("\\>>") {
            return "End casework.".to_string();
        }

        // Handle derived constructions
        if self.handle_derived_construction(stmt).is_some() {
            return self.handle_derived_construction(stmt).unwrap();
        }

        // Graph construction
        if stmt.starts_with("G:") {
            let eq = stmt[2..].trim().trim_matches(|c| c == '{' || c == '}');
            return format!("Graph the function {}.", eq);
        }

        // Point construction
        if stmt.starts_with("P:") {
            return self.handle_point_construction(&stmt[2..]);
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
            return self.handle_circle_construction(&stmt[2..]);
        }

        // Polygon
        if stmt.starts_with("J:") {
            return format!("Construct polygon {}.", &stmt[2..]);
        }

        // Regular polygon
        if stmt.starts_with("R:") {
            return format!("Construct {}.", self.translate_regular_polygon(&stmt[2..]));
        }

        // Handle arcs
        if stmt.starts_with("a") && (stmt.contains("=") || stmt.contains("?")) {
            return self.handle_arc(stmt);
        }

        // Handle sectors
        if stmt.starts_with("q") && (stmt.contains("=") || stmt.contains("?")) {
            return self.handle_sector(stmt);
        }

        // Area
        if stmt.contains('[') && stmt.contains(']') {
            return self.handle_area(stmt);
        }

        // Perimeter
        if stmt.contains('(') && stmt.contains(')') {
            return self.handle_perimeter(stmt);
        }

        // Angle
        if stmt.contains('<') || stmt.contains('∠') {
            return self.handle_angle(stmt);
        }

        // Question with proof inquiry
        if stmt.ends_with("\\?") {
            return format!("Prove that {}.", stmt.trim_end_matches("\\?"));
        }

        // Question
        if stmt.ends_with('?') {
            if stmt.contains('*') {
                // Property question
                return self.handle_property_question(stmt);
            }
            return format!("What is {}?", stmt.trim_end_matches('?'));
        }

        // Equality/Inequality statements
        if stmt.contains("!=") || stmt.contains("≠") {
            return stmt.replace("!=", " does not equal ").replace("≠", " does not equal ");
        }
        if stmt.contains(">=") || stmt.contains("≥") {
            return stmt.replace(">=", " is greater than or equal to ").replace("≥", " is greater than or equal to ");
        }
        if stmt.contains("<=") || stmt.contains("≤") {
            return stmt.replace("<=", " is less than or equal to ").replace("≤", " is less than or equal to ");
        }

        stmt.to_string()
    }

    fn handle_derived_construction(&self, stmt: &str) -> Option<String> {
        for (prefix, name) in &self.derived_constructions {
            if stmt.starts_with(&format!("{}:", prefix)) {
                let rest = &stmt[prefix.len() + 1..];
                return Some(format!("Construct the {} of {}.", name, rest));
            }
        }
        None
    }

    fn handle_point_construction(&self, rest: &str) -> String {
        // Multiple points
        if rest.contains(',') && !rest.contains('.') && !rest.contains('|') && !rest.contains('x') {
            return format!("Construct points {}.", rest);
        }

        // Intersection with assignment
        if rest.contains('x') && rest.contains('=') {
            if let Some(eq_pos) = rest.find('=') {
                let point = &rest[..eq_pos];
                let intersection = &rest[eq_pos + 1..];
                if let Some(x_pos) = intersection.find('x') {
                    let obj1 = &intersection[..x_pos];
                    let obj2 = &intersection[x_pos + 1..];
                    
                    // Handle prefixes (w for ray, l for line, c for circle)
                    let obj1_desc = self.get_object_description(obj1);
                    let obj2_desc = self.get_object_description(obj2);
                    
                    return format!(
                        "Let point {} be the intersection of {} and {}.",
                        point, obj1_desc, obj2_desc
                    );
                }
            }
        }

        // Point with coordinates
        if rest.contains('{') && rest.contains('}') {
            if let Some(start) = rest.find('{') {
                if let Some(end) = rest.find('}') {
                    let point = &rest[..rest.find('|').unwrap_or(start)];
                    let coords = &rest[start + 1..end];
                    return format!("Let point {} be at coordinates {}.", point, coords);
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

                    let cond_strs = self.parse_conditions(conditions);

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

        format!("Construct point {}.", rest)
    }

    fn get_object_description(&self, obj: &str) -> String {
        if obj.starts_with('w') {
            format!("ray {}", &obj[1..])
        } else if obj.starts_with('l') {
            format!("line {}", &obj[1..])
        } else if obj.starts_with('c') {
            format!("circle {}", &obj[1..])
        } else {
            format!("segment {}", obj)
        }
    }

    fn parse_conditions(&self, conditions: &str) -> Vec<String> {
        conditions
            .split(',')
            .map(|c| {
                let c = c.trim();
                if c.starts_with("R:") {
                    self.translate_regular_polygon(&c[2..])
                } else if c.contains('[') && c.contains(']') && c.contains('=') {
                    if let Some(start) = c.find('[') {
                        if let Some(end) = c.find(']') {
                            let obj = &c[start + 1..end];
                            let val = &c[end + 2..];
                            return format!("the area of {} is {}", obj, val);
                        }
                    }
                    c.to_string()
                } else if c.contains('*') {
                    let parts: Vec<&str> = c.split('*').collect();
                    if parts.len() == 2 {
                        let prop_name = self
                            .properties
                            .get(parts[1])
                            .or_else(|| self.relationships.get(parts[1]))
                            .unwrap_or(&parts[1]);
                        return format!("{} is {}", parts[0], prop_name);
                    }
                    c.to_string()
                } else if c.contains(';') && c.contains('*') {
                    // Handle relationship with semicolon (e.g., AB;BC*PR)
                    let parts: Vec<&str> = c.split('*').collect();
                    if parts.len() == 2 {
                        let objs = parts[0];
                        let rel = self.relationships.get(parts[1]).unwrap_or(&parts[1]);
                        return format!("{} are {}", objs, rel);
                    }
                    c.to_string()
                } else if c.starts_with("-u") {
                    "above".to_string()
                } else if c.starts_with("-d") {
                    "below".to_string()
                } else if c.starts_with("-r") {
                    "to the right".to_string()
                } else if c.starts_with("-l") {
                    "to the left".to_string()
                } else {
                    c.to_string()
                }
            })
            .collect()
    }

    fn handle_circle_construction(&self, rest: &str) -> String {
        let parts: Vec<&str> = rest.split(';').collect();

        if parts.len() == 3 {
            return format!(
                "Construct a circle through points {}, {}, and {}.",
                parts[0], parts[1], parts[2]
            );
        }
        if parts.len() == 2 {
            if parts[1].chars().all(|c| c.is_numeric() || c == '.') {
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
        if parts.len() == 1 {
            if rest.starts_with("=") {
                return format!("Construct circle {}.", &rest[1..]);
            }
            return format!("Construct a circle passing through point {}.", rest);
        }
        format!("Construct circle with parameters {}.", rest)
    }

    fn handle_arc(&self, stmt: &str) -> String {
        if stmt.contains("=") && !stmt.contains("!=") && !stmt.ends_with('?') {
            let parts: Vec<&str> = stmt.split('=').collect();
            if parts.len() == 2 {
                let arc = parts[0].trim_start_matches('a');
                let val = parts[1];
                if stmt.starts_with("<a") {
                    return format!("Arc {} has measure {} degrees.", arc, val);
                }
                return format!("Arc {} has length {}.", arc, val);
            }
        }
        if stmt.contains("!=") || stmt.contains("≠") {
            return stmt.replace("!=", " does not equal ").replace("≠", " does not equal ");
        }
        if stmt.ends_with('?') {
            let arc = stmt.trim_end_matches('?').trim_start_matches('a');
            return format!("What is the length of arc {}?", arc);
        }
        stmt.to_string()
    }

    fn handle_sector(&self, stmt: &str) -> String {
        if stmt.contains("=") && !stmt.ends_with('?') {
            let parts: Vec<&str> = stmt.split('=').collect();
            if parts.len() == 2 {
                let sector = parts[0].trim_start_matches('q');
                let val = parts[1];
                return format!("The area of sector {} is {}.", sector, val);
            }
        }
        if stmt.ends_with('?') {
            let sector = stmt.trim_end_matches('?').trim_start_matches('q');
            return format!("What is the area of sector {}?", sector);
        }
        stmt.to_string()
    }

    fn handle_area(&self, stmt: &str) -> String {
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
        stmt.to_string()
    }

    fn handle_perimeter(&self, stmt: &str) -> String {
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
        stmt.to_string()
    }

    fn handle_angle(&self, stmt: &str) -> String {
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
        format!("Angle {}", rest)
    }

    fn handle_property_question(&self, stmt: &str) -> String {
        let stmt_clean = stmt.trim_end_matches('?');
        if let Some(star_pos) = stmt_clean.find('*') {
            let obj = &stmt_clean[..star_pos];
            let prop = &stmt_clean[star_pos + 1..];
            
            if prop.contains(';') {
                // Relationship question
                let rel_name = self.relationships.get(prop).unwrap_or(&prop);
                return format!("Are {} {}?", obj, rel_name);
            } else {
                // Property question
                let prop_name = self.properties.get(prop).unwrap_or(&prop);
                return format!("Is {} {}?", obj, prop_name);
            }
        }
        format!("What is {}?", stmt_clean)
    }

    fn translate_casework(&self, cases: &str) -> String {
        let case_pairs: Vec<&str> = cases.split("),(").collect();
        let mut result = Vec::new();
        
        for (i, case) in case_pairs.iter().enumerate() {
            let case = case.trim_matches(|c| c == '(' || c == ')');
            let parts: Vec<&str> = case.split(';').collect();
            if parts.len() == 2 {
                result.push(format!(
                    "Case {}: if {}, then {}",
                    i + 1,
                    parts[0],
                    parts[1]
                ));
            }
        }
        
        result.join("; ")
    }

    fn translate_regular_polygon(&self, s: &str) -> String {
        let parts: Vec<&str> = s.split(';').collect();
        if parts.len() >= 2 {
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

fn show_about() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║           GSL Compiler (gslc) - About                         ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
    println!("Geometry Shorthand Language (GSL) Compiler");
    println!("Version: 2.0.0\n");
    println!("Created by: Aarav Desai (politikl on GitHub)");
    println!("Language by: LX and YY (initial were used to conceal the authors' real names)\n");
    println!("Description:");
    println!("  GSL is a highly logical and rigorous shorthand language for");
    println!("  expressing geometric constructions and proofs. This compiler");
    println!("  translates GSL shorthand into natural English descriptions.\n");
    println!("Quote:");
    println!("  \"Geometry Shorthand (Construction) is a highly logical and");
    println!("   rigorous language. A mistake in conventions will not be");
    println!("   tolerated.\" - LX\n");
    println!("Repository: https://github.com/politikl/gslc");
    println!("Language Docs: https://tinyurl.com/geoshorthand\n");
    println!("For help, use: gslc help");
    println!("For language reference, use: gslc lang\n");
}

fn show_help() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║           GSL Compiler (gslc) - Quick Start                   ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
    println!("USAGE:");
    println!("  gslc <shorthand>              Translate shorthand directly");
    println!("  gslc -f <file.gsl>            Translate from file");
    println!("  gslc <shorthand> -o <output>  Save translation to file");
    println!("  gslc about                    Show about information");
    println!("  gslc help                     Show this help message");
    println!("  gslc lang                     Open language documentation\n");
    println!("BASIC SYNTAX:");
    println!("  \\\\...\\\\                      Wrap shorthand in double backslashes");
    println!("  /                             Separate statements");
    println!("  P:A                           Construct point A");
    println!("  S:AB                          Connect segment AB");
    println!("  J:ABC                         Construct triangle ABC");
    println!("  [ABC]=20                      Area of ABC is 20");
    println!("  <ABC=90                       Angle ABC is 90 degrees\n");
    println!("EXAMPLES:");
    println!("  gslc \"\\\\P:A,B/S:AB\\\\\"");
    println!("    → 1. Construct points A, B.");
    println!("       2. Connect segment AB.\n");
    println!("  gslc \"\\\\J:ABC/R:3;AB=ABC\\\\\"");
    println!("    → 1. Construct polygon ABC.");
    println!("       2. Construct equilateral triangle ABC with side AB.\n");
    println!("  gslc -f problem.gsl -o solution.txt");
    println!("    → Translates problem.gsl and saves to solution.txt\n");
    println!("CONSTRUCTIONS:");
    println!("  P:   Point           S:   Segment        L:   Line");
    println!("  W:   Ray             C:   Circle         J:   Polygon");
    println!("  R:   Regular polygon G:   Graph\n");
    println!("QUERIES:");
    println!("  [ABC]?               What is the area of ABC?");
    println!("  AB?                  What is AB?");
    println!("  AB=BC\\?              Prove that AB = BC\n");
    println!("For complete language reference: gslc lang");
    println!("For more examples: https://github.com/politikl/gslc\n");
}

fn open_lang_docs() {
    println!("\nOpening GSL language documentation...");
    println!("URL: https://tinyurl.com/geoshorthand\n");
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(&["/C", "start", "https://tinyurl.com/geoshorthand"])
            .spawn()
            .ok();
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg("https://tinyurl.com/geoshorthand")
            .spawn()
            .ok();
    }
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg("https://tinyurl.com/geoshorthand")
            .spawn()
            .ok();
    }
    
    println!("If the browser didn't open automatically, visit:");
    println!("https://tinyurl.com/geoshorthand\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if we have any arguments
    if args.len() < 2 {
        eprintln!("Usage: gslc <shorthand> [-o output.txt]");
        eprintln!("       gslc -f input.gsl [-o output.txt]");
        eprintln!("\nExamples:");
        eprintln!("  gslc \"\\\\P:A/P:B/S:AB\\\\\"");
        eprintln!("  gslc \"\\\\P:A/P:B/S:AB\\\\\" -o output.txt");
        eprintln!("  gslc -f problem.gsl -o solution.txt");
        eprintln!("\nFor help: gslc help");
        std::process::exit(1);
    }

    // Handle special commands FIRST, before any other processing
    let first_arg = args[1].to_lowercase();
    if first_arg == "about" {
        show_about();
        std::process::exit(0);
    }
    if first_arg == "help" {
        show_help();
        std::process::exit(0);
    }
    if first_arg == "lang" {
        open_lang_docs();
        std::process::exit(0);
    }

    // Now proceed with normal translation logic
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

    // Check if input is empty after parsing
    if input.is_empty() {
        eprintln!("Error: No input provided");
        eprintln!("For help: gslc help");
        std::process::exit(1);
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
