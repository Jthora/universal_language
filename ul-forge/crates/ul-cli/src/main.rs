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
