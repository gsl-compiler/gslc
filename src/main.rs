use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write, BufRead};
use std::time::Instant;
use std::process::Command;

struct Translator {
    properties: HashMap<&'static str, &'static str>,
    relationships: HashMap<&'static str, &'static str>,
    theorems: HashMap<&'static str, (&'static str, &'static str)>,
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
        theorems.insert("_TI", ("Triangle Inequality", "In any triangle, the sum of the lengths of any two sides must be greater than the length of the remaining side."));
        theorems.insert("_ST", ("Stewart's Theorem", "Relates the length of a cevian in a triangle to the side lengths."));
        theorems.insert("_AT", ("Apollonius Theorem", "Relates the length of a median to the side lengths."));
        theorems.insert("_VT", ("Viviani's Theorem", "The sum of perpendiculars from any interior point to the sides of an equilateral triangle equals its altitude."));
        theorems.insert("_NP", ("Napoleon's Theorem", "Constructing equilateral triangles on the sides of any triangle creates centers that form an equilateral triangle."));
        theorems.insert("_EL", ("Euler Line", "The circumcenter, centroid, and orthocenter are collinear."));
        theorems.insert("_9C", ("Nine-Point Circle", "A circle passing through nine notable points of a triangle."));
        theorems.insert("_SL", ("Simson Line", "The feet of perpendiculars from a point on the circumcircle to the sides are collinear."));
        theorems.insert("_CV", ("Ceva's Theorem", "Conditions for three cevians to be concurrent."));
        theorems.insert("_ML", ("Menelaus' Theorem", "Conditions for three points on triangle sides to be collinear."));
        theorems.insert("_AB", ("Angle Bisector Theorem", "An angle bisector divides the opposite side in the ratio of the adjacent sides."));
        theorems.insert("_IE", ("Incenter-Excenter Lemma", "Relationships between incenter and excenters."));
        theorems.insert("_CT", ("Carnot's Theorem", "Relates signed distances from vertices to the sides."));
        theorems.insert("_M", ("Miquel's Theorem", "Four circles through vertices of a complete quadrilateral meet at a point."));
        theorems.insert("_ET", ("Euler's Theorem", "In a triangle, R² = 2Rr where R is circumradius and r is inradius."));
        theorems.insert("_DT", ("Desargue's Theorem", "Two triangles are in perspective axially if and only if they are in perspective centrally."));
        theorems.insert("_HF", ("Heron's Formula", "Area = √(s(s-a)(s-b)(s-c)) where s is the semiperimeter."));
        theorems.insert("_QF", ("Bretschneider's Formula", "Generalizes Brahmagupta's formula to general quadrilaterals."));
        theorems.insert("_BF", ("Brahmagupta's Formula", "Area formula for cyclic quadrilaterals."));
        theorems.insert("_JT", ("Japanese Theorem", "The centers of incircles of triangles formed by a cyclic polygon lie on a circle."));
        theorems.insert("_NT", ("Newton's Theorem", "The center of a circumcircle of a tangential quadrilateral lies on the Newton line."));
        theorems.insert("_PT", ("Ptolemy's Theorem", "In a cyclic quadrilateral: AC·BD = AB·CD + BC·AD."));
        theorems.insert("_PP", ("Power of a Point", "For a point and circle, the product of signed distances along any line is constant."));
        theorems.insert("_BT", ("Butterfly Theorem", "A chord perpendicular to a diameter creates equal segments."));
        theorems.insert("_PC", ("Pascal's Theorem", "Points of intersection of opposite sides of a hexagon inscribed in a conic are collinear."));
        theorems.insert("_LC", ("Law of Cosines", "c² = a² + b² - 2ab·cos(C)"));
        theorems.insert("_LS", ("Law of Sines", "a/sin(A) = b/sin(B) = c/sin(C) = 2R"));
        theorems.insert("_LT", ("Law of Tangents", "Relates tangent of half-angles to side lengths."));
        theorems.insert("_PK", ("Pick's Theorem", "Area = I + B/2 - 1 for lattice polygons."));
        theorems.insert("_SH", ("Shoelace Theorem", "Formula for polygon area using coordinates."));
        theorems.insert("_SSC", ("SSS Congruence", "Three sides determine triangle congruence."));
        theorems.insert("_SAC", ("SAS Congruence", "Two sides and included angle determine congruence."));
        theorems.insert("_SSA", ("SSA Congruence", "Two sides and non-included angle (ambiguous case)."));
        theorems.insert("_ASA", ("ASA Congruence", "Two angles and included side determine congruence."));
        theorems.insert("_AAS", ("AAS Congruence", "Two angles and non-included side determine congruence."));
        theorems.insert("_HL", ("HL Congruence", "Hypotenuse-leg congruence for right triangles."));
        theorems.insert("_AA", ("AA Similarity", "Two angles determine triangle similarity."));
        theorems.insert("_SAS", ("SAS Similarity", "Two sides and included angle determine similarity."));
        theorems.insert("_SSS", ("SSS Similarity", "Three sides in proportion determine similarity."));

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

    fn validate(&self, input: &str) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        let input = input.trim();

        // Check for balanced delimiters
        if input.starts_with("\\\\") && !input.ends_with("\\\\") {
            errors.push("Missing closing \\\\ delimiter".to_string());
        }

        let open_brackets = input.matches('[').count();
        let close_brackets = input.matches(']').count();
        if open_brackets != close_brackets {
            errors.push(format!("Mismatched brackets: {} open, {} close", open_brackets, close_brackets));
        }

        let open_parens = input.matches('(').count();
        let close_parens = input.matches(')').count();
        if open_parens != close_parens {
            errors.push(format!("Mismatched parentheses: {} open, {} close", open_parens, close_parens));
        }

        let open_braces = input.matches('{').count();
        let close_braces = input.matches('}').count();
        if open_braces != close_braces {
            errors.push(format!("Mismatched braces: {} open, {} close", open_braces, close_braces));
        }

        // Check for valid construction prefixes
        let statements: Vec<&str> = input.split('/').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
        for (i, stmt) in statements.iter().enumerate() {
            let stmt = stmt.trim_start_matches("\\\\").trim_end_matches("\\\\");
            if stmt.contains(':') {
                let prefix = stmt.split(':').next().unwrap_or("");
                let valid_prefixes = ["P", "S", "L", "W", "C", "J", "R", "G", "PB", "CCO", "CC", "AB", "ICO", "IC", "EAB", "ECO", "EC", "M", "MD", "CT", "PD", "OC", "PL"];
                if !valid_prefixes.contains(&prefix) && !prefix.starts_with('\\') && !prefix.is_empty() {
                    errors.push(format!("Line {}: Unknown construction prefix '{}'", i + 1, prefix));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn analyze(&self, input: &str) -> AnalysisResult {
        let statements: Vec<&str> = input.split('/').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
        
        let mut points = 0;
        let mut segments = 0;
        let mut circles = 0;
        let mut lines = 0;
        let mut polygons = 0;
        let mut theorems_used = Vec::new();

        for stmt in &statements {
            if stmt.contains("P:") { points += 1; }
            if stmt.contains("S:") { segments += 1; }
            if stmt.contains("C:") { circles += 1; }
            if stmt.contains("L:") { lines += 1; }
            if stmt.contains("J:") || stmt.contains("R:") { polygons += 1; }
            
            for (abbr, _) in &self.theorems {
                if stmt.contains(abbr) {
                    theorems_used.push(abbr.to_string());
                }
            }
        }

        AnalysisResult {
            total_statements: statements.len(),
            points,
            segments,
            circles,
            lines,
            polygons,
            theorems_used,
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
        if rest.contains(',') && !rest.contains('.') && !rest.contains('|') && !rest.contains('x') {
            return format!("Construct points {}.", rest);
        }

        if rest.contains('x') && rest.contains('=') {
            if let Some(eq_pos) = rest.find('=') {
                let point = &rest[..eq_pos];
                let intersection = &rest[eq_pos + 1..];
                if let Some(x_pos) = intersection.find('x') {
                    let obj1 = &intersection[..x_pos];
                    let obj2 = &intersection[x_pos + 1..];
                    
                    let obj1_desc = self.get_object_description(obj1);
                    let obj2_desc = self.get_object_description(obj2);
                    
                    return format!(
                        "Let point {} be the intersection of {} and {}.",
                        point, obj1_desc, obj2_desc
                    );
                }
            }
        }

        if rest.contains('{') && rest.contains('}') {
            if let Some(start) = rest.find('{') {
                if let Some(end) = rest.find('}') {
                    let point = &rest[..rest.find('|').unwrap_or(start)];
                    let coords = &rest[start + 1..end];
                    return format!("Let point {} be at coordinates {}.", point, coords);
                }
            }
        }

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
                let rel_name = self.relationships.get(prop).unwrap_or(&prop);
                return format!("Are {} {}?", obj, rel_name);
            } else {
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

        if input.starts_with("\\\\") && input.ends_with("\\\\") {
            input = input[2..input.len() - 2].to_string();
        }

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

    fn format_output(&self, translations: &[String], format: &str) -> String {
        match format {
            "latex" => self.format_latex(translations),
            "markdown" | "md" => self.format_markdown(translations),
            "json" => self.format_json(translations),
            "html" => self.format_html(translations),
            _ => translations
                .iter()
                .enumerate()
                .map(|(i, t)| format!("{}. {}", i + 1, t))
                .collect::<Vec<_>>()
                .join("\n"),
        }
    }

    fn format_latex(&self, translations: &[String]) -> String {
        let mut output = String::from("\\begin{enumerate}\n");
        for trans in translations {
            output.push_str(&format!("  \\item {}\n", trans));
        }
        output.push_str("\\end{enumerate}");
        output
    }

    fn format_markdown(&self, translations: &[String]) -> String {
        translations
            .iter()
            .enumerate()
            .map(|(i, t)| format!("{}. {}", i + 1, t))
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    fn format_json(&self, translations: &[String]) -> String {
        let items: Vec<String> = translations
            .iter()
            .enumerate()
            .map(|(i, t)| {
                let escaped = t.replace('\\', "\\\\").replace('"', "\\\"");
                format!("  {{\n    \"step\": {},\n    \"description\": \"{}\"\n  }}", i + 1, escaped)
            })
            .collect();
        format!("{{\n  \"steps\": [\n{}\n  ]\n}}", items.join(",\n"))
    }

    fn format_html(&self, translations: &[String]) -> String {
        let mut output = String::from("<!DOCTYPE html>\n<html>\n<head>\n  <meta charset=\"UTF-8\">\n  <title>GSL Translation</title>\n  <style>\n    body { font-family: Arial, sans-serif; max-width: 800px; margin: 50px auto; padding: 20px; }\n    ol { line-height: 1.8; }\n    li { margin-bottom: 10px; }\n  </style>\n</head>\n<body>\n  <h1>Geometry Construction Steps</h1>\n  <ol>\n");
        for trans in translations {
            let escaped = trans.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");
            output.push_str(&format!("    <li>{}</li>\n", escaped));
        }
        output.push_str("  </ol>\n</body>\n</html>");
        output
    }

    fn generate_svg(&self, input: &str, width: u32, height: u32) -> String {
        let mut svg = format!(
            "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">\n",
            width, height
        );
        svg.push_str("  <rect width=\"100%\" height=\"100%\" fill=\"#f5f5f5\"/>\n");
        svg.push_str("  <style>\n");
        svg.push_str("    .point { fill: #000; }\n");
        svg.push_str("    .line { stroke: #000; stroke-width: 2; }\n");
        svg.push_str("    .label { font-family: Arial; font-size: 14px; fill: #000; }\n");
        svg.push_str("  </style>\n");

        let statements: Vec<&str> = input.split('/').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
        let mut points: HashMap<String, (f64, f64)> = HashMap::new();
        let mut point_index = 0;

        // Simple layout: arrange points in a circle
        let cx = width as f64 / 2.0;
        let cy = height as f64 / 2.0;
        let radius = (width.min(height) as f64 / 2.0) * 0.7;

        for stmt in statements {
            let stmt = stmt.trim_start_matches("\\\\").trim_end_matches("\\\\");
            
            // Extract point names
            if stmt.starts_with("P:") {
                let rest = &stmt[2..];
                let point_names: Vec<&str> = rest.split(',').collect();
                for name in point_names {
                    let name = name.trim();
                    if !points.contains_key(name) {
                        let angle = (point_index as f64) * 2.0 * std::f64::consts::PI / 6.0;
                        let x = cx + radius * angle.cos();
                        let y = cy + radius * angle.sin();
                        points.insert(name.to_string(), (x, y));
                        point_index += 1;
                    }
                }
            }
            
            // Draw segments
            if stmt.starts_with("S:") {
                let rest = &stmt[2..];
                if rest.len() >= 2 {
                    let p1 = &rest[0..1];
                    let p2 = &rest[1..2];
                    if let (Some(&(x1, y1)), Some(&(x2, y2))) = (points.get(p1), points.get(p2)) {
                        svg.push_str(&format!(
                            "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" class=\"line\"/>\n",
                            x1, y1, x2, y2
                        ));
                    }
                }
            }
            
            // Draw polygons
            if stmt.starts_with("J:") {
                let rest = &stmt[2..];
                let mut path = String::from("  <path d=\"M");
                for (i, ch) in rest.chars().enumerate() {
                    if let Some(&(x, y)) = points.get(&ch.to_string()) {
                        if i == 0 {
                            path.push_str(&format!(" {} {}", x, y));
                        } else {
                            path.push_str(&format!(" L {} {}", x, y));
                        }
                    }
                }
                path.push_str(" Z\" class=\"line\" fill=\"none\"/>\n");
                svg.push_str(&path);
            }
        }

        // Draw points last (so they appear on top)
        for (name, &(x, y)) in &points {
            svg.push_str(&format!("  <circle cx=\"{}\" cy=\"{}\" r=\"4\" class=\"point\"/>\n", x, y));
            svg.push_str(&format!("  <text x=\"{}\" y=\"{}\" class=\"label\">{}</text>\n", x + 10.0, y - 10.0, name));
        }

        svg.push_str("</svg>");
        svg
    }
}

struct AnalysisResult {
    total_statements: usize,
    points: usize,
    segments: usize,
    circles: usize,
    lines: usize,
    polygons: usize,
    theorems_used: Vec<String>,
}

impl AnalysisResult {
    fn display(&self) {
        println!("\n╔═══════════════════════════════════════════════════════════════╗");
        println!("║                    Analysis Results                           ║");
        println!("╚═══════════════════════════════════════════════════════════════╝\n");
        println!("Total Statements: {}", self.total_statements);
        println!("\nConstruction Breakdown:");
        println!("  Points:    {}", self.points);
        println!("  Segments:  {}", self.segments);
        println!("  Lines:     {}", self.lines);
        println!("  Circles:   {}", self.circles);
        println!("  Polygons:  {}", self.polygons);
        
        if !self.theorems_used.is_empty() {
            println!("\nTheorems Referenced:");
            for theorem in &self.theorems_used {
                println!("  • {}", theorem);
            }
        }
        
        let complexity = self.total_statements + self.theorems_used.len() * 2;
        println!("\nComplexity Score: {}", complexity);
        println!("  (Simple: <10, Moderate: 10-20, Complex: >20)\n");
    }
}

fn show_about() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║           GSL Compiler (gslc) - About                         ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
    println!("Geometry Shorthand Language (GSL) Compiler");
    println!("Version: 3.0.1\n");
    println!("Created by: Aarav Desai (politikl on GitHub)");
    println!("Language by: LX and YY\n");
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
    println!("  gslc <shorthand> -o <file>    Save translation to file");
    println!("  gslc <shorthand> --copy       Copy output to clipboard");
    println!("  gslc about                    Show about information");
    println!("  gslc help                     Show this help message");
    println!("  gslc lang                     Open language documentation");
    println!("  gslc theorems                 List all theorems");
    println!("  gslc theorem <code>           Show theorem details");
    println!("  gslc examples                 Show examples");
    println!("  gslc examples <topic>         Show topic-specific examples");
    println!("  gslc interactive              Start interactive mode");
    println!("  gslc validate <shorthand>     Validate syntax");
    println!("  gslc stats <shorthand>        Show construction statistics");
    println!("  gslc visualize <shorthand>    Generate SVG visualization");
    println!("  gslc clip                     Translate from clipboard");
    println!("  gslc benchmark                Run performance benchmark\n");
    println!("OUTPUT FORMATS:");
    println!("  --format latex                LaTeX enumerate format");
    println!("  --format markdown             Markdown format");
    println!("  --format json                 JSON format");
    println!("  --format html                 HTML document\n");
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
    println!("  gslc -f problem.gsl --format latex");
    println!("  gslc validate \"\\\\P:A/S:AB\\\\\"");
    println!("  gslc visualize \"\\\\P:A,B,C/J:ABC\\\\\" -o triangle.svg");
    println!("  gslc \"\\\\P:A,B/S:AB\\\\\" --copy\n");
    println!("For complete reference: gslc lang\n");
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

fn show_theorems(translator: &Translator) {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                    Available Theorems                         ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
    
    println!("TRIANGLE THEOREMS:");
    for (code, (name, _)) in &translator.theorems {
        if ["_TI", "_ST", "_AT", "_VT", "_NP", "_EL", "_9C", "_SL", "_CV", "_ML", "_AB", "_IE", "_CT", "_M", "_ET", "_DT", "_HF"].contains(&code.as_ref()) {
            println!("  {} - {}", code, name);
        }
    }
    
    println!("\nQUADRILATERAL THEOREMS:");
    for (code, (name, _)) in &translator.theorems {
        if ["_QF", "_BF", "_JT", "_NT"].contains(&code.as_ref()) {
            println!("  {} - {}", code, name);
        }
    }
    
    println!("\nCIRCLE THEOREMS:");
    for (code, (name, _)) in &translator.theorems {
        if ["_PT", "_PP", "_BT", "_PC"].contains(&code.as_ref()) {
            println!("  {} - {}", code, name);
        }
    }
    
    println!("\nTRIGONOMETRIC THEOREMS:");
    for (code, (name, _)) in &translator.theorems {
        if ["_LC", "_LS", "_LT"].contains(&code.as_ref()) {
            println!("  {} - {}", code, name);
        }
    }
    
    println!("\nCONGRUENCE & SIMILARITY:");
    for (code, (name, _)) in &translator.theorems {
        if ["_SSC", "_SAC", "_SSA", "_ASA", "_AAS", "_HL", "_AA", "_SAS", "_SSS"].contains(&code.as_ref()) {
            println!("  {} - {}", code, name);
        }
    }
    
    println!("\nANALYTICAL GEOMETRY:");
    for (code, (name, _)) in &translator.theorems {
        if ["_PK", "_SH"].contains(&code.as_ref()) {
            println!("  {} - {}", code, name);
        }
    }
    
    println!("\nUse 'gslc theorem <code>' for detailed information.\n");
}

fn show_theorem_detail(translator: &Translator, code: &str) {
    let code = if !code.starts_with('_') {
        format!("_{}", code)
    } else {
        code.to_string()
    };
    
    if let Some((name, desc)) = translator.theorems.get(code.as_str()) {
        println!("\n╔═══════════════════════════════════════════════════════════════╗");
        println!("║                    Theorem Details                            ║");
        println!("╚═══════════════════════════════════════════════════════════════╝\n");
        println!("Code: {}", code);
        println!("Name: {}\n", name);
        println!("Description:");
        println!("  {}\n", desc);
    } else {
        eprintln!("Error: Theorem '{}' not found.", code);
        eprintln!("Use 'gslc theorems' to see all available theorems.\n");
    }
}

fn show_examples(topic: Option<&str>) {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                    GSL Examples                               ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
    
    match topic {
        Some("triangles") | Some("triangle") => {
            println!("TRIANGLE CONSTRUCTIONS:\n");
            println!("1. Basic Triangle:");
            println!("   GSL:        \\\\P:A,B,C/J:ABC\\\\");
            println!("   English:    Construct points A, B, C. Construct triangle ABC.\n");
            
            println!("2. Equilateral Triangle:");
            println!("   GSL:        \\\\P:A,B/S:AB/R:3;AB=ABC\\\\");
            println!("   English:    Construct points A, B. Connect segment AB.");
            println!("               Construct equilateral triangle ABC with side AB.\n");
            
            println!("3. Right Triangle with Area:");
            println!("   GSL:        \\\\J:ABC/ABC*RT/[ABC]=12\\\\");
            println!("   English:    Construct triangle ABC. ABC is right. Area of ABC is 12.\n");
        }
        Some("circles") | Some("circle") => {
            println!("CIRCLE CONSTRUCTIONS:\n");
            println!("1. Circle with Center and Radius:");
            println!("   GSL:        \\\\P:O/C:O;5\\\\");
            println!("   English:    Construct point O. Circle with center O and radius 5.\n");
            
            println!("2. Circle through Three Points:");
            println!("   GSL:        \\\\P:A,B,C/C:A;B;C\\\\");
            println!("   English:    Construct points A, B, C. Circle through A, B, and C.\n");
            
            println!("3. Circumcircle of Triangle:");
            println!("   GSL:        \\\\J:ABC/CC:ABC=P:O\\\\");
            println!("   English:    Construct triangle ABC. Construct circumcircle of ABC.\n");
        }
        Some("proofs") | Some("proof") => {
            println!("PROOF CONSTRUCTIONS:\n");
            println!("1. Simple Proof:");
            println!("   GSL:        \\\\\\p:AB=CD/...steps.../\\q\\\\");
            println!("   English:    We will prove: AB equals CD. ... And that is what was to be shown.\n");
            
            println!("2. Proof by Contradiction:");
            println!("   GSL:        \\\\\\pC:AB!=CD/...steps.../\\qC\\\\");
            println!("   English:    We will prove by contradiction: AB does not equal CD.");
            println!("               ... Achieving a contradiction.\n");
        }
        None => {
            println!("AVAILABLE EXAMPLE TOPICS:");
            println!("  • triangles  - Triangle construction examples");
            println!("  • circles    - Circle construction examples");
            println!("  • proofs     - Proof examples");
            println!("  • basic      - Basic constructions\n");
            
            println!("BASIC EXAMPLES:\n");
            println!("1. Two Points and a Segment:");
            println!("   GSL:        \\\\P:A,B/S:AB\\\\");
            println!("   English:    Construct points A, B. Connect segment AB.\n");
            
            println!("2. Triangle with Given Area:");
            println!("   GSL:        \\\\J:ABC/[ABC]=20\\\\");
            println!("   English:    Construct triangle ABC. Area of ABC is 20.\n");
            
            println!("3. Perpendicular Bisector:");
            println!("   GSL:        \\\\S:AB/PB:AB=lCD\\\\");
            println!("   English:    Connect segment AB. Construct perpendicular bisector of AB.\n");
            
            println!("Use 'gslc examples <topic>' for more specific examples.\n");
        }
        _ => {
            println!("Topic '{}' not found.\n", topic.unwrap());
            println!("Available topics: triangles, circles, proofs, basic\n");
        }
    }
}

fn interactive_mode(translator: &Translator) {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║              GSL Interactive Mode                             ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
    println!("Enter GSL statements (without \\\\). Type 'exit' to quit.\n");
    
    let stdin = io::stdin();
    let mut step = 1;
    
    loop {
        print!("gsl> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        if input == "exit" || input == "quit" {
            println!("\nExiting interactive mode.\n");
            break;
        }
        
        if input == "help" {
            println!("\nInteractive Mode Commands:");
            println!("  Enter any GSL statement to translate");
            println!("  'clear' - Clear step counter");
            println!("  'exit' or 'quit' - Exit interactive mode\n");
            continue;
        }
        
        if input == "clear" {
            step = 1;
            println!("Step counter reset.\n");
            continue;
        }
        
        let translations = translator.translate(input);
        for trans in translations {
            println!("{}. {}", step, trans);
            step += 1;
        }
        println!();
    }
}

fn copy_to_clipboard(text: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let mut child = Command::new("pbcopy")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn pbcopy: {}", e))?;
        
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes())
                .map_err(|e| format!("Failed to write to clipboard: {}", e))?;
        }
        
        child.wait().map_err(|e| format!("Failed to wait for pbcopy: {}", e))?;
        Ok(())
    }
    
    #[cfg(target_os = "linux")]
    {
        let mut child = Command::new("xclip")
            .args(&["-selection", "clipboard"])
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn xclip (try: sudo apt install xclip): {}", e))?;
        
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes())
                .map_err(|e| format!("Failed to write to clipboard: {}", e))?;
        }
        
        child.wait().map_err(|e| format!("Failed to wait for xclip: {}", e))?;
        Ok(())
    }
    
    #[cfg(target_os = "windows")]
    {
        let mut child = Command::new("clip")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn clip: {}", e))?;
        
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes())
                .map_err(|e| format!("Failed to write to clipboard: {}", e))?;
        }
        
        child.wait().map_err(|e| format!("Failed to wait for clip: {}", e))?;
        Ok(())
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        Err("Clipboard not supported on this platform".to_string())
    }
}

fn read_from_clipboard() -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        let output = Command::new("pbpaste")
            .output()
            .map_err(|e| format!("Failed to run pbpaste: {}", e))?;
        
        String::from_utf8(output.stdout)
            .map_err(|e| format!("Invalid UTF-8 in clipboard: {}", e))
    }
    
    #[cfg(target_os = "linux")]
    {
        let output = Command::new("xclip")
            .args(&["-selection", "clipboard", "-o"])
            .output()
            .map_err(|e| format!("Failed to run xclip (try: sudo apt install xclip): {}", e))?;
        
        String::from_utf8(output.stdout)
            .map_err(|e| format!("Invalid UTF-8 in clipboard: {}", e))
    }
    
    #[cfg(target_os = "windows")]
    {
        // Windows PowerShell clipboard reading
        let output = Command::new("powershell")
            .args(&["-command", "Get-Clipboard"])
            .output()
            .map_err(|e| format!("Failed to read clipboard: {}", e))?;
        
        String::from_utf8(output.stdout)
            .map_err(|e| format!("Invalid UTF-8 in clipboard: {}", e))
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        Err("Clipboard not supported on this platform".to_string())
    }
}

fn run_benchmark(translator: &Translator) {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                    Benchmark Results                          ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
    
    let test_cases = vec![
        ("Simple", "\\\\P:A,B/S:AB\\\\"),
        ("Medium", "\\\\P:A,B,C/J:ABC/R:3;AB=ABC/[ABC]=20\\\\"),
        ("Complex", "\\\\P:A,B/S:AB/R:3;AB=ABC/P:D.AC|R:3;AD=ADE,[ADE]=20/P:F.BC|R:3;BF=BFG,[BFG]=5/S:DF/P:H.AB|J:DFH*R/[DFH]?\\\\"),
    ];
    
    for (name, input) in test_cases {
        let iterations = 1000;
        let start = Instant::now();
        
        for _ in 0..iterations {
            translator.translate(input);
        }
        
        let duration = start.elapsed();
        let avg_time = duration.as_micros() / iterations;
        
        println!("{} case:", name);
        println!("  {} iterations in {:?}", iterations, duration);
        println!("  Average: {} μs per translation\n", avg_time);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: gslc <shorthand> [-o output.txt]");
        eprintln!("       gslc -f input.gsl [-o output.txt]");
        eprintln!("\nFor help: gslc help");
        std::process::exit(1);
    }

    let translator = Translator::new();
    let first_arg = args[1].to_lowercase();
    
    // Handle special commands
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
    if first_arg == "theorems" {
        show_theorems(&translator);
        std::process::exit(0);
    }
    if first_arg == "theorem" {
        if args.len() < 3 {
            eprintln!("Usage: gslc theorem <code>");
            eprintln!("Example: gslc theorem _PT");
            std::process::exit(1);
        }
        show_theorem_detail(&translator, &args[2]);
        std::process::exit(0);
    }
    if first_arg == "examples" {
        let topic = args.get(2).map(|s| s.as_str());
        show_examples(topic);
        std::process::exit(0);
    }
    if first_arg == "interactive" || first_arg == "-i" {
        interactive_mode(&translator);
        std::process::exit(0);
    }
    if first_arg == "benchmark" {
        run_benchmark(&translator);
        std::process::exit(0);
    }
    if first_arg == "validate" {
        if args.len() < 3 {
            eprintln!("Usage: gslc validate <shorthand>");
            std::process::exit(1);
        }
        match translator.validate(&args[2]) {
            Ok(()) => println!("✓ Syntax is valid!"),
            Err(errors) => {
                eprintln!("✗ Validation errors:");
                for error in errors {
                    eprintln!("  • {}", error);
                }
                std::process::exit(1);
            }
        }
        std::process::exit(0);
    }
    if first_arg == "stats" {
        if args.len() < 3 {
            eprintln!("Usage: gslc stats <shorthand>");
            std::process::exit(1);
        }
        let analysis = translator.analyze(&args[2]);
        analysis.display();
        std::process::exit(0);
    }
    if first_arg == "visualize" {
        if args.len() < 3 {
            eprintln!("Usage: gslc visualize <shorthand> [-o output.svg]");
            std::process::exit(1);
        }
        let svg = translator.generate_svg(&args[2], 800, 600);
        
        let mut output_file = None;
        let mut i = 3;
        while i < args.len() {
            if args[i] == "-o" && i + 1 < args.len() {
                output_file = Some(args[i + 1].clone());
                break;
            }
            i += 1;
        }
        
        if let Some(path) = output_file {
            fs::write(&path, &svg).unwrap_or_else(|e| {
                eprintln!("Error writing file: {}", e);
                std::process::exit(1);
            });
            println!("SVG visualization written to: {}", path);
        } else {
            println!("{}", svg);
        }
        std::process::exit(0);
    }
    if first_arg == "clip" {
        match read_from_clipboard() {
            Ok(input) => {
                let translations = translator.translate(&input);
                let output = translator.format_output(&translations, "plain");
                println!("{}", output);
            }
            Err(e) => {
                eprintln!("Error reading from clipboard: {}", e);
                std::process::exit(1);
            }
        }
        std::process::exit(0);
    }

    // Normal translation mode
    let mut input = String::new();
    let mut output_file: Option<String> = None;
    let mut format = "plain";
    let mut copy_to_clip = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-f" => {
                if i + 1 < args.len() {
                    i += 1;
                    input = fs::read_to_string(&args[i]).unwrap_or_else(|e| {
                        eprintln!("Error reading file: {}", e);
                        std::process::exit(1);
                    });
                }
            }
            "-o" => {
                if i + 1 < args.len() {
                    i += 1;
                    output_file = Some(args[i].clone());
                }
            }
            "--format" => {
                if i + 1 < args.len() {
                    i += 1;
                    format = &args[i];
                }
            }
            "--copy" => {
                copy_to_clip = true;
            }
            _ => {
                if input.is_empty() {
                    input = args[i].clone();
                }
            }
        }
        i += 1;
    }

    if input.is_empty() {
        eprintln!("Error: No input provided");
        std::process::exit(1);
    }

    // Validate first
    if let Err(errors) = translator.validate(&input) {
        eprintln!("⚠ Validation warnings:");
        for error in errors {
            eprintln!("  • {}", error);
        }
        eprintln!();
    }

    let translations = translator.translate(&input);
    let output = translator.format_output(&translations, format);

    if let Some(output_path) = output_file {
        fs::write(&output_path, &output).unwrap_or_else(|e| {
            eprintln!("Error writing file: {}", e);
            std::process::exit(1);
        });
        println!("Translation written to: {}", output_path);
    } else {
        println!("{}", output);
    }
    
    if copy_to_clip {
        match copy_to_clipboard(&output) {
            Ok(()) => println!("\n✓ Output copied to clipboard!"),
            Err(e) => eprintln!("\n✗ Failed to copy to clipboard: {}", e),
        }
    }
}
