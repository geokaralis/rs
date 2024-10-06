use std::fs;
use std::io;
use std::path::Path;
use std::process::{Command, Stdio};
use swc_common::GLOBALS;
use swc_common::{sync::Lrc, SourceMap};
use swc_ecma_ast::ImportDecl;
use swc_ecma_ast::ModuleDecl;
use swc_ecma_ast::ModuleItem;
use swc_ecma_ast::Program;
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};
use swc_ecma_parser::TsSyntax;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc_ecma_transforms_base::resolver;
use swc_ecma_transforms_react::{react, Options as ReactOptions};
use swc_ecma_transforms_typescript::typescript::strip;
use swc_ecma_visit::Fold;
use swc_ecma_visit::FoldWith;
use swc_ecma_visit::VisitMut;
use swc_ecma_visit::VisitMutWith;
use walkdir::WalkDir;

struct ResolveImport;

impl VisitMut for ResolveImport {
    fn visit_mut_module_items(&mut self, items: &mut Vec<ModuleItem>) {
        items.retain(|item| {
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) = item {
                if import_decl.src.value.contains("globals") {
                    return false;
                }
            }
            true
        });

        for item in items.iter_mut() {
            self.visit_mut_module_item(item);
        }
    }

    fn visit_mut_import_decl(&mut self, import_decl: &mut ImportDecl) {
        if !import_decl.src.value.contains("react") && !import_decl.src.value.contains("globals") {
            let import_path = import_decl.src.value.to_string();
            import_decl.src.value = format!("{}.mjs", import_path).into();
            import_decl.src.raw = Some(format!("\"{}.mjs\"", import_path).into());
        }
    }
}

fn font(app_dir: &Path, dist_dir: &Path) -> io::Result<()> {
    let font_src = app_dir.join("inter.woff2");
    let font_dest = dist_dir.join("inter.woff2");

    if font_src.exists() {
        fs::copy(font_src, font_dest)?;
        println!("copied dist/inter.woff2");
    } else {
        println!("inter.woff2 not found in app folder");
    }

    Ok(())
}

fn tailwindcss(app_dir: &Path, dist_dir: &Path) -> io::Result<()> {
    let tailwind_bin = Path::new("node_modules")
        .join(".bin")
        .join(if cfg!(windows) {
            "tailwindcss.cmd"
        } else {
            "tailwindcss"
        });

    let status = Command::new(tailwind_bin)
        .args(&[
            "-i",
            &app_dir.join("globals.css").to_string_lossy(),
            "-o",
            &dist_dir.join("output.css").to_string_lossy(),
            "--minify",
        ])
        .status()?;

    if !status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "tailwindcss failed"));
    }

    Ok(())
}

fn transpile(
    file_path: &Path,
    cm: Lrc<SourceMap>,
    output_dir: &Path,
    app_dir: &Path,
) -> io::Result<()> {
    let fm = cm.load_file(file_path).expect("failed to load file");

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax {
            tsx: true,
            ..Default::default()
        }),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().expect("failed to parse module");

    let unresolved_mark = swc_common::Mark::new();
    let top_level_mark = swc_common::Mark::new();

    let module = module.fold_with(&mut resolver(unresolved_mark, top_level_mark, true));

    let mut react_transform = react(
        cm.clone(),
        Some(swc_common::comments::NoopComments),
        ReactOptions::default(),
        top_level_mark,
        unresolved_mark,
    );

    let transformed_module = react_transform.fold_module(module);

    let program = Program::Module(transformed_module);
    let mut program = program.fold_with(&mut strip(unresolved_mark, top_level_mark));

    let mut resolve_import = ResolveImport;
    program.visit_mut_with(&mut resolve_import);

    let mut buf = vec![];
    {
        let writer = Box::new(JsWriter::new(cm.clone(), "\n", &mut buf, None));
        let mut emitter = Emitter {
            cfg: swc_ecma_codegen::Config::default(),
            cm: cm.clone(),
            wr: writer,
            comments: None,
        };
        emitter.emit_program(&program).unwrap();
    }

    let js_content = String::from_utf8(buf).expect("failed to convert buffer to string");

    let relative_path = file_path.strip_prefix(app_dir).unwrap();
    let output_file = output_dir.join(relative_path).with_extension("mjs");

    fs::create_dir_all(output_file.parent().unwrap())?;
    fs::write(output_file, js_content)?;

    Ok(())
}

fn main() -> io::Result<()> {
    GLOBALS.set(&Default::default(), || {
        let cm: Lrc<SourceMap> = Default::default();

        let app_dir = Path::new("app");
        let cache_dir = Path::new("node_modules/.cache");
        let dist_dir = Path::new("dist");

        fs::create_dir_all(cache_dir)?;
        fs::create_dir_all(dist_dir)?;

        for entry in WalkDir::new(app_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
                if extension == "ts" || extension == "tsx" {
                    transpile(path, cm.clone(), cache_dir, app_dir)?;
                }
            }
        }

        let render_script = r#"
        import React from 'react';
        import ReactDOMServer from 'react-dom/server';
        import Index from './index.mjs';
        
        async function render() {
            async function resolvePromises(element) {
                if (typeof element.type === 'function') {
                    const Component = element.type;
                    const props = element.props;
                    const result = Component(props);

                    if (result && typeof result.then === 'function') {
                        const resolvedComponent = await result;
                        return resolvePromises(resolvedComponent);
                    } else if (React.isValidElement(result)) {
                        const children = React.Children.toArray(result.props.children);
                        const resolvedChildren = await Promise.all(children.map(resolvePromises));
                        return React.cloneElement(result, {}, ...resolvedChildren);
                    }

                    return result;
                } else if (React.isValidElement(element)) {
                    const children = React.Children.toArray(element.props.children);
                    const resolvedChildren = await Promise.all(children.map(resolvePromises));
                    return React.cloneElement(element, {}, ...resolvedChildren);
                }

                return element;
            }

            const app = await resolvePromises(React.createElement(Index));
            const html = ReactDOMServer.renderToString(app);
            console.log(html);
        }

        render().catch(error => {
            console.error('an error occurred during rendering:', error);
            process.exit(1);
        });"#;

        fs::write(cache_dir.join("render.mjs"), render_script)?;

        let output = Command::new("node")
            .current_dir(cache_dir)
            .arg("render.mjs")
            .stdout(Stdio::piped())
            .output()?;

        let html = String::from_utf8(output.stdout).expect("failed to render html");

        println!("{:?}", html);

        let layout = format!(r#"<!doctype html>{}"#, html);

        fs::write(dist_dir.join("index.html"), layout)?;

        println!("created dist/index.html");

        tailwindcss(app_dir, dist_dir)?;
        font(app_dir, dist_dir)?;

        Ok(())
    })
}
