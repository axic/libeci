/*
 * libeci: Ethereum WebAssembly ABI compliance library
 *
 * Copyright (c) 2018 Jake Lang
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use parity_wasm::elements::{External, FunctionType, Internal, Module, Type};

/// Resolves a function's signature from its callable index. Borrowed from parity-wasm examples.
pub fn func_type_by_index(module: &Module, index: usize) -> FunctionType {
    let function_section = module
        .function_section()
        .expect("No function section found");
    let type_section = module.type_section().expect("No type section found");

    let import_section_len: usize = match module.import_section() {
        Some(import) => import
            .entries()
            .iter()
            .filter(|entry| match entry.external() {
                &External::Function(_) => true,
                _ => false,
            })
            .count(),
        None => 0,
    };

    let function_index_in_section = index - import_section_len;

    let func_type_ref: usize =
        function_section.entries()[function_index_in_section].type_ref() as usize;

    match type_section.types()[func_type_ref] {
        Type::Function(ref func_type) => func_type.clone(),
    }
}

/// Resolves an imported function's signature from its callable index.
pub fn imported_func_type_by_index(module: &Module, index: usize) -> FunctionType {
    let import_section = module.import_section().expect("No function section found");
    let type_section = module.type_section().expect("No type section found");

    let func_type_ref: usize = match import_section.entries()[index].external() {
        &External::Function(idx) => idx as usize,
        _ => usize::max_value(),
    };

    match type_section.types()[func_type_ref] {
        Type::Function(ref func_type) => func_type.clone(),
    }
}

/// Resolves an export name to a tuple containing its callable index and internal reference.
pub fn resolve_export_by_name(module: &Module, name: &str) -> Option<(u32, Internal)> {
    if !has_export_section(module) {
        None
    } else {
        let idx: Option<(u32, Internal)> = match module
            .export_section()
            .unwrap()
            .entries()
            .iter()
            .find(|export| if export.field() == name { true } else { false })
        {
            Some(export) => match *export.internal() {
                //halp
                Internal::Function(index) => Some((index, Internal::Function(index))),
                Internal::Memory(index) => Some((index, Internal::Memory(index))),
                Internal::Global(index) => Some((index, Internal::Global(index))),
                Internal::Table(index) => Some((index, Internal::Table(index))),
            },
            None => None,
        };
        idx
    }
}

/// Parses a module's import section into a list of tuples containing the import fields.
pub fn get_imports(module: &Module) -> Option<Vec<(&str, &str)>> {
    if !has_import_section(module) {
        return None;
    } else {
        let imports_list: Option<Vec<(&str, &str)>> = Some(
            module
                .import_section()
                .unwrap()
                .entries()
                .iter()
                .map(|x| (x.module(), x.field()))
                .collect(),
        );
        imports_list
    }
}

/// Determines whether a module has an export section.
pub fn has_export_section(module: &Module) -> bool {
    match module.export_section() {
        Some(_thing) => true,
        None => false,
    }
}

/// Determines whether a module has an import section.
pub fn has_import_section(module: &Module) -> bool {
    match module.import_section() {
        Some(_thing) => true,
        None => false,
    }
}
