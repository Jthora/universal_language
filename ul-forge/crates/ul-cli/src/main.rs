use clap::{Parser, Subcommand};
use std::io::{self, Read};

#[derive(Parser)]
#[command(name = "ul", about = "Universal Language command-line tools")]
#[command(version, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse UL-Script to GIR JSON
    Parse {
        /// Input file (omit for stdin)
        input: Option<String>,
        /// Output file (omit for stdout)
        #[arg(short, long)]
        output: Option<String>,
        /// Pretty-print JSON
        #[arg(long, default_value_t = false)]
        pretty: bool,
    },
    /// Render GIR JSON to SVG
    Render {
        /// Input GIR file (omit for stdin)
        input: Option<String>,
        /// Output file (omit for stdout)
        #[arg(short, long)]
        output: Option<String>,
        /// Output format
        #[arg(long, default_value = "svg")]
        format: String,
    },
    /// Validate GIR JSON
    Validate {
        /// Input GIR file (omit for stdin)
        input: Option<String>,
        /// Enable geometric satisfiability checks
        #[arg(long, default_value_t = false)]
        geometry: bool,
        /// Output as JSON
        #[arg(long, default_value_t = false)]
        json: bool,
    },
    /// Convert between formats
    Convert {
        /// Input file
        input: String,
        /// Output file
        #[arg(short, long)]
        output: String,
    },
    /// Compose Σ_UL operations on UL-Script operands
    Compose {
        /// Operation name (one of the 13 Σ_UL operations)
        #[arg(value_parser = [
            "predicate", "modify_entity", "modify_relation", "negate",
            "conjoin", "disjoin", "embed", "abstract", "compose",
            "invert", "quantify", "bind", "modify_assertion",
        ])]
        operation: String,
        /// UL-Script operand strings
        #[arg(required = true, num_args = 1..)]
        operands: Vec<String>,
        /// Output file (omit for stdout)
        #[arg(short, long)]
        output: Option<String>,
        /// Pretty-print JSON
        #[arg(long, default_value_t = false)]
        pretty: bool,
    },
    /// Analyze a GIR and detect Σ_UL operations
    Analyze {
        /// Input GIR file (omit for stdin)
        input: Option<String>,
    },
    /// Set performative force on an assertion
    Force {
        /// Force type
        #[arg(value_parser = ["assert", "query", "direct", "commit", "express", "declare"])]
        force: String,
        /// Input UL-Script (or GIR file with --gir)
        input: String,
        /// Read input as GIR JSON instead of UL-Script
        #[arg(long, default_value_t = false)]
        gir: bool,
    },
    /// Run pragmatic inference on a GIR
    Pragmatics {
        /// Input GIR file (omit for stdin)
        input: Option<String>,
    },
    /// Evaluate a GIR against composition rules
    Evaluate {
        /// Input GIR file (omit for stdin)
        input: Option<String>,
        /// Path to composition rules JSON file
        #[arg(short, long)]
        rules: Option<String>,
        /// Output as JSON
        #[arg(long, default_value_t = false)]
        json: bool,
    },
    /// Check UL-Script: parse, validate, detect operations, compare to expected
    Check {
        /// UL-Script input string
        input: String,
        /// Expected operation(s), comma-separated (e.g. "predicate,modify_entity")
        #[arg(long)]
        expect: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Parse {
            input,
            output,
            pretty,
        } => cmd_parse(input, output, pretty),
        Commands::Render {
            input,
            output,
            format,
        } => cmd_render(input, output, &format),
        Commands::Validate {
            input,
            geometry,
            json,
        } => cmd_validate(input, geometry, json),
        Commands::Convert { input, output } => cmd_convert(&input, &output),
        Commands::Compose {
            operation,
            operands,
            output,
            pretty,
        } => cmd_compose(&operation, &operands, output, pretty),
        Commands::Analyze { input } => cmd_analyze(input),
        Commands::Force { force, input, gir } => cmd_force(&force, &input, gir),
        Commands::Pragmatics { input } => cmd_pragmatics(input),
        Commands::Evaluate { input, rules, json } => cmd_evaluate(input, rules, json),
        Commands::Check { input, expect } => cmd_check(&input, expect),
    };

    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn read_input(path: Option<String>) -> io::Result<String> {
    match path {
        Some(p) => std::fs::read_to_string(p),
        None => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            Ok(buf)
        }
    }
}

fn write_output(path: Option<String>, content: &str) -> io::Result<()> {
    match path {
        Some(p) => std::fs::write(p, content),
        None => {
            print!("{content}");
            Ok(())
        }
    }
}

fn cmd_parse(
    input: Option<String>,
    output: Option<String>,
    pretty: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let text = read_input(input)?;
    let gir = ul_core::parser::parse(&text)?;
    let json = if pretty {
        gir.to_json_pretty()?
    } else {
        gir.to_json()?
    };
    write_output(output, &json)?;
    Ok(())
}

fn cmd_render(
    input: Option<String>,
    output: Option<String>,
    format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = read_input(input)?;
    let gir = ul_core::Gir::from_json(&json)?;
    let options = ul_core::renderer::RenderOptions {
        format: match format {
            "tikz" => ul_core::renderer::OutputFormat::TikZ,
            _ => ul_core::renderer::OutputFormat::Svg,
        },
        ..Default::default()
    };
    let result = ul_core::renderer::render(&gir, &options)?;
    write_output(output, &result)?;
    Ok(())
}

fn cmd_validate(
    input: Option<String>,
    geometry: bool,
    json_output: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = read_input(input)?;
    let gir = ul_core::Gir::from_json(&json)?;
    let result = ul_core::validator::validate(&gir, geometry);

    if json_output {
        let out = serde_json::json!({
            "valid": result.valid,
            "errors": result.errors.iter().map(|e| e.to_string()).collect::<Vec<_>>(),
            "warnings": result.warnings,
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else {
        if result.valid {
            println!("valid");
        } else {
            for err in &result.errors {
                eprintln!("{err}");
            }
        }
        for warn in &result.warnings {
            eprintln!("warning: {warn}");
        }
    }

    if result.valid {
        Ok(())
    } else {
        std::process::exit(1);
    }
}

fn cmd_convert(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Infer formats from file extensions
    let in_ext = input.rsplit('.').next().unwrap_or("");
    let out_ext = output.rsplit('.').next().unwrap_or("");

    match (in_ext, out_ext) {
        ("ul", "json") | ("ul", _) if output.ends_with(".ul.json") => {
            // UL-Script → GIR JSON
            let text = std::fs::read_to_string(input)?;
            let gir = ul_core::parser::parse(&text)?;
            let json = gir.to_json_pretty()?;
            std::fs::write(output, json)?;
        }
        ("json", "svg") => {
            // GIR JSON → SVG
            let json = std::fs::read_to_string(input)?;
            let gir = ul_core::Gir::from_json(&json)?;
            let options = ul_core::renderer::RenderOptions::default();
            let svg = ul_core::renderer::render(&gir, &options)?;
            std::fs::write(output, svg)?;
        }
        ("ul", "svg") => {
            // UL-Script → SVG (parse then render)
            let text = std::fs::read_to_string(input)?;
            let gir = ul_core::parser::parse(&text)?;
            let options = ul_core::renderer::RenderOptions::default();
            let svg = ul_core::renderer::render(&gir, &options)?;
            std::fs::write(output, svg)?;
        }
        ("json", "tikz") | ("json", "tex") => {
            // GIR JSON → TikZ
            let json = std::fs::read_to_string(input)?;
            let gir = ul_core::Gir::from_json(&json)?;
            let options = ul_core::renderer::RenderOptions {
                format: ul_core::renderer::OutputFormat::TikZ,
                ..Default::default()
            };
            let tikz = ul_core::renderer::render(&gir, &options)?;
            std::fs::write(output, tikz)?;
        }
        _ => {
            eprintln!("error: cannot determine conversion from '.{in_ext}' to '.{out_ext}'");
            eprintln!("supported: .ul → .ul.json / .svg, .json → .svg / .tikz");
            std::process::exit(2);
        }
    }

    Ok(())
}

fn cmd_compose(
    operation: &str,
    operands: &[String],
    output: Option<String>,
    pretty: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let girs: Vec<ul_core::Gir> = operands
        .iter()
        .map(|s| ul_core::parser::parse(s))
        .collect::<Result<Vec<_>, _>>()?;

    let result = match operation {
        "negate" => ul_core::composer::negate(&girs[0]),
        "embed" => ul_core::composer::embed(&girs[0]),
        "abstract" => ul_core::composer::abstract_op(&girs[0]),
        "invert" => ul_core::composer::invert(&girs[0]),
        "predicate" => ul_core::composer::predicate(&girs[0], &girs[1], &girs[2]),
        "modify_entity" => ul_core::composer::modify_entity(&girs[0], &girs[1]),
        "modify_relation" => ul_core::composer::modify_relation(&girs[0], &girs[1]),
        "conjoin" => ul_core::composer::conjoin(&girs[0], &girs[1]),
        "disjoin" => ul_core::composer::disjoin(&girs[0], &girs[1]),
        "compose" => ul_core::composer::compose(&girs[0], &girs[1]),
        "quantify" => ul_core::composer::quantify(&girs[0], &girs[1]),
        "bind" => ul_core::composer::bind(&girs[0], &girs[1]),
        "modify_assertion" => ul_core::composer::modify_assertion(&girs[0], &girs[1]),
        _ => unreachable!("clap validates operation name"),
    }?;

    let json = if pretty {
        result.to_json_pretty()?
    } else {
        result.to_json()?
    };
    write_output(output, &json)?;
    Ok(())
}

fn cmd_analyze(input: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let json_str = read_input(input)?;
    let gir = ul_core::Gir::from_json(&json_str)?;
    let ops = ul_core::composer::detect_operations(&gir);

    let out = serde_json::json!({
        "operations": ops.iter().map(|o| o.operation).collect::<Vec<_>>(),
        "node_count": gir.nodes.len(),
        "edge_count": gir.edges.len(),
    });
    println!("{}", serde_json::to_string_pretty(&out)?);
    Ok(())
}

fn cmd_force(
    force_str: &str,
    input: &str,
    is_gir: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let gir = if is_gir {
        let json_str = std::fs::read_to_string(input)?;
        ul_core::Gir::from_json(&json_str)?
    } else {
        ul_core::parser::parse(input)?
    };

    let force = match force_str {
        "assert" => ul_core::PerformativeForce::Assert,
        "query" => ul_core::PerformativeForce::Query,
        "direct" => ul_core::PerformativeForce::Direct,
        "commit" => ul_core::PerformativeForce::Commit,
        "express" => ul_core::PerformativeForce::Express,
        "declare" => ul_core::PerformativeForce::Declare,
        _ => unreachable!("clap validates force name"),
    };

    let result = ul_core::performative::with_force(&gir, force)?;
    println!("{}", result.to_json_pretty()?);
    Ok(())
}

fn cmd_pragmatics(input: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let json_str = read_input(input)?;
    let gir = ul_core::Gir::from_json(&json_str)?;
    let inferences = ul_core::pragmatic::infer(&gir);

    let results: Vec<serde_json::Value> = inferences
        .iter()
        .map(|inf| {
            serde_json::json!({
                "rule": format!("{:?}", inf.rule),
                "surface": serde_json::to_value(&inf.surface).unwrap_or_default(),
                "intended": serde_json::to_value(&inf.intended).unwrap_or_default(),
            })
        })
        .collect();

    let out = serde_json::json!({
        "inferences": results,
        "count": results.len()
    });
    println!("{}", serde_json::to_string_pretty(&out)?);
    Ok(())
}

fn cmd_evaluate(
    input: Option<String>,
    rules_path: Option<String>,
    json_output: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let json_str = read_input(input)?;
    let gir = ul_core::Gir::from_json(&json_str)?;

    // Load rules if provided, otherwise use default (empty) context
    let rules_json = match rules_path {
        Some(p) => std::fs::read_to_string(p)?,
        None => "[]".to_string(),
    };

    let config = ul_game::types::GameConfig {
        session_id: "cli".to_string(),
        rules_json,
    };
    let ctx = ul_game::context::GameContext::from_config(&config)
        .map_err(|e| format!("failed to create context: {e}"))?;
    let result = ul_game::evaluation::evaluate(&ctx, &gir);

    if json_output {
        println!("{}", serde_json::to_string_pretty(&serde_json::to_value(&result)?)?);
    } else {
        println!("Score:    {:.2}", result.score);
        println!("Grade:    {:?}", result.grade);
        if !result.matched_rules.is_empty() {
            println!("Matched:  {}", result.matched_rules.join(", "));
        }
        if !result.violated_rules.is_empty() {
            println!("Violated: {}", result.violated_rules.join(", "));
        }
        for fb in &result.feedback {
            println!("  {fb}");
        }
    }
    Ok(())
}

fn cmd_check(input: &str, expect: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Parse
    let gir = match ul_core::parser::parse(input) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Parse error: {e}");
            std::process::exit(1);
        }
    };
    println!("✓ Parsed ({} nodes, {} edges)", gir.nodes.len(), gir.edges.len());

    // Step 2: Validate
    let vr = ul_core::validator::validate(&gir, false);
    if vr.valid {
        println!("✓ Valid");
    } else {
        eprintln!("✗ Invalid:");
        for err in &vr.errors {
            eprintln!("  {err}");
        }
        std::process::exit(1);
    }

    // Step 3: Detect operations
    let ops = ul_core::composer::detect_operations(&gir);
    let op_names: Vec<&str> = ops.iter().map(|o| o.operation).collect();
    println!("  Operations: {}", if op_names.is_empty() { "(none)".to_string() } else { op_names.join(", ") });

    // Step 4: Compare to expected (if given)
    if let Some(expected) = expect {
        let expected_ops: Vec<&str> = expected.split(',').map(|s| s.trim()).collect();
        let mut all_found = true;
        for exp in &expected_ops {
            if op_names.contains(exp) {
                println!("✓ Found expected operation: {exp}");
            } else {
                eprintln!("✗ Missing expected operation: {exp}");
                all_found = false;
            }
        }
        if !all_found {
            std::process::exit(1);
        }
    }

    Ok(())
}
