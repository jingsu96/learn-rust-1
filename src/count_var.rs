#![deny(clippy::all)]

use napi_derive::napi;
use oxc_allocator::Allocator;
use oxc_ast::ast::{Declaration, ModuleDeclaration, Statement};
use oxc_parser::Parser;
use oxc_span::SourceType;

#[napi(object)]
pub struct AnalysisResult {
  pub variable_declarations: i32,
  pub function_declarations: i32,
  pub class_declarations: i32,
  pub export_declarations: i32,
}

#[napi]
pub fn analysis_source_code(source_code: String) -> Result<AnalysisResult, napi::Error> {
  let allocator = Allocator::default();
  let ret = Parser::new(&allocator, &source_code, SourceType::default()).parse();

  let mut variable_declarations = 0;
  let mut function_declarations = 0;
  let mut class_declarations = 0;
  let mut export_declarations = 0;

  for statement in ret.program.body {
    match statement {
      Statement::Declaration(declaration) => match declaration {
        Declaration::VariableDeclaration(variable) => {
          variable_declarations += variable.declarations.len() as i32;
        }
        Declaration::FunctionDeclaration(_) => {
          function_declarations += 1;
        }
        Declaration::ClassDeclaration(_) => {
          class_declarations += 1;
        }
        _ => {}
      },
      Statement::ModuleDeclaration(module_declaration) => match &*module_declaration {
        ModuleDeclaration::ExportDefaultDeclaration(_) => {
          export_declarations += 1;
        }
        ModuleDeclaration::ExportAllDeclaration(_) => {
          export_declarations += 1;
        }
        ModuleDeclaration::ExportNamedDeclaration(export) => {
          export_declarations += 1;

          if let Some(declaration) = &export.declaration {
            match declaration {
              Declaration::VariableDeclaration(variable) => {
                variable_declarations += variable.declarations.len() as i32;
              }
              Declaration::FunctionDeclaration(_) => {
                function_declarations += 1;
              }
              Declaration::ClassDeclaration(_) => {
                class_declarations += 1;
              }
              _ => {}
            }
          }
        }
        _ => {}
      },
      _ => {}
    }
  }

  Ok(AnalysisResult {
    variable_declarations,
    function_declarations,
    class_declarations,
    export_declarations,
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_analysis_source_code() {
    let source_code = r#"
          let a = 1;
          function foo() {}
          class Bar {}
          export const b = 2;
          export function baz() {}
          export class Qux {}
          export default function() {}
          export * from 'module';
      "#;
    let result = analysis_source_code(source_code.to_string()).unwrap();
    assert_eq!(result.variable_declarations, 2);
    assert_eq!(result.function_declarations, 3);
    assert_eq!(result.class_declarations, 2);
    assert_eq!(result.export_declarations, 5);
  }
}
