use crate::{args::Args, idl::Idl};
use pyo3::{
    ffi::c_str,
    marker::Python,
    types::{PyAnyMethods, PyModule},
};
use std::{error::Error, ffi::CString, fs};

struct PythonInfo {
    cpp_path: Vec<String>,
    cc_flags: String,
}

impl PythonInfo {
    fn parse(file_path: &str) -> Result<PythonInfo, Box<dyn Error>> {
        Python::with_gil(|py| {
            let parent_dir = std::path::Path::new(file_path)
                .parent()
                .ok_or("Invalid file path")?
                .to_str()
                .ok_or("Path conversion error")?;
            let current_dir = std::env::current_dir()?;
            std::env::set_current_dir(parent_dir)?;

            py.import("sys")?
                .getattr("path")?
                .call_method1("append", (parent_dir,))?;

            let code = fs::read_to_string(file_path)?;
            let py_module = PyModule::from_code(
                py,
                CString::new(code)?.as_c_str(),
                CString::new(file_path)?.as_c_str(),
                c_str!("awtk_config"),
            )?;
            let info = PythonInfo {
                cpp_path: py_module.getattr("CPPPATH")?.extract()?,
                cc_flags: py_module.getattr("CCFLAGS")?.extract()?,
            };

            std::env::set_current_dir(current_dir)?;

            Ok::<PythonInfo, Box<dyn Error>>(info)
        })
        .map_err(Into::into)
    }

    fn gen_clang_args(py_config_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let mut ret: Vec<String> = Vec::new();
        let info = PythonInfo::parse(py_config_path)?;

        info.cpp_path.iter().for_each(|cpp_path: &String| {
            ret.push("-I".to_string() + cpp_path);
        });

        let cc_flags: Vec<&str> = info.cc_flags.split_whitespace().collect();
        cc_flags.iter().for_each(|flag: &&str| {
            ret.push(flag.to_string());
        });

        println!("clang args: {:?}", ret);

        Ok(ret)
    }
}

#[derive(Debug)]
struct BuilderParseConverter {
    idl: Idl,
}
impl bindgen::callbacks::ParseCallbacks for BuilderParseConverter {
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        /* 将类名转为大驼峰命名 */
        if self.idl.classes.contains_key(original_item_name) || original_item_name.ends_with("_t") {
            let sanitized = original_item_name
                .trim_start_matches('_')
                .trim_end_matches("_t");
            Some(heck::ToPascalCase::to_pascal_case(sanitized))
        } else {
            Some(original_item_name.into())
        }
    }

    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        /* 将枚举名转为大驼峰命名 */
        if let Some(mut enum_name_real) = enum_name {
            enum_name_real = enum_name_real
                .trim_start_matches("enum ")
                .trim_start_matches('_');

            let mut name_opt: Option<String> = None;

            if let Some(enum_) = self.idl.enums.get(enum_name_real) {
                if !enum_.prefix.is_empty() {
                    let sanitized = original_variant_name.trim_start_matches(&enum_.prefix);
                    name_opt = Some(heck::ToPascalCase::to_pascal_case(sanitized));
                }
            } else {
                let prefix = enum_name_real.trim_end_matches("_t").to_uppercase() + "_";
                let sanitized = if original_variant_name.starts_with(&prefix) {
                    &original_variant_name[prefix.len()..]
                } else {
                    original_variant_name
                };
                name_opt = Some(heck::ToPascalCase::to_pascal_case(sanitized));
            }

            if let Some(mut name) = name_opt {
                /* 以数字开头时，前面加下划线 */
                if name.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                    name = format!("_{name}");
                }
                return Some(name);
            }
        }
        Some(heck::ToPascalCase::to_pascal_case(original_variant_name))
    }
}

pub struct Builder {
    builder: bindgen::Builder,
}

impl Builder {
    fn new() -> Builder {
        Builder {
            builder: bindgen::Builder::default()
                /* 白名单递归匹配所有关联类型 */
                .allowlist_recursively(true)
                /* 按语义而非字母顺序排序 */
                .sort_semantically(true)
                /* 将宏常量适配为合适类型 */
                .fit_macro_constants(true)
                /* 禁用 size_t 到 usize 的自动转换，保持与C一致的size_t类型表示 */
                .size_t_is_usize(false)
                /* 自动检测 #[no_mangle] 等函数属性 */
                .enable_function_attribute_detection()
                /* 保留 C 风格的数组指针 */
                .array_pointers_in_arguments(true)
                /* 枚举风格配置（生成标准Rust枚举） */
                .default_enum_style(bindgen::EnumVariation::Rust {
                    non_exhaustive: true, /* 阻止穷尽匹配，保持C枚举扩展兼容性 */
                })
                /* 使用 core 库替代 std（适用于 no_std 环境） */
                .use_core(),
        }
    }

    pub fn build(args: &Args, idl: &Idl) -> Result<(), Box<dyn Error>> {
        let mut b: Builder = Builder::new();
        b.builder = idl
            .classes
            .iter()
            .fold(b.builder, |bld, (class_name, class)| {
                let bld = bld.allowlist_type(class_name);
                class.methods.iter().fold(bld, |inner_bld, method| {
                    inner_bld.allowlist_function(&method.name)
                })
            });

        b.builder = idl.enums.iter().fold(b.builder, |bld, (enum_name, _enum)| {
            bld.allowlist_type(enum_name)
        });

        b.builder
            /* 添加命名转换回调 */
            .parse_callbacks(Box::new(BuilderParseConverter { idl: idl.clone() }))
            .headers(&args.header_paths)
            .clang_args(PythonInfo::gen_clang_args(&args.py_config_path)?)
            .generate()?
            .write_to_file(&args.out_path)?;

        Ok(())
    }
}
