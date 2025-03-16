use crate::{args::Args, idl::Idl};
use pyo3::{
    marker::Python,
    types::{PyAnyMethods, PyModule},
};
use std::{env, error::Error, ffi::CString, fs, path};

struct PythonInfo {
    cpp_path: Vec<String>,
    cc_flags: String,
}

impl PythonInfo {
    fn parse(file_path: &str) -> Result<Self, Box<dyn Error>> {
        Python::with_gil(|py| {
            let filename = path::Path::new(file_path)
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| format!("Failed to extract filename from path: {file_path}"))?;

            let parent_dir = path::Path::new(file_path)
                .parent()
                .ok_or_else(|| format!("Invalid file path: {file_path}"))?
                .to_str()
                .ok_or_else(|| format!("Path conversion error: {file_path}"))?;

            let current_dir = env::current_dir()?;
            env::set_current_dir(parent_dir)?;

            py.import("sys")?
                .getattr("path")?
                .call_method1("append", (parent_dir,))?;

            let code = fs::read_to_string(file_path)?;
            let py_module = PyModule::from_code(
                py,
                CString::new(code)?.as_c_str(),
                CString::new(file_path)?.as_c_str(),
                CString::new(filename)?.as_c_str(),
            )?;
            let info = PythonInfo {
                cpp_path: py_module.getattr("CPPPATH")?.extract()?,
                cc_flags: py_module.getattr("CCFLAGS")?.extract()?,
            };

            env::set_current_dir(current_dir)?;

            Ok::<PythonInfo, Box<dyn Error>>(info)
        })
        .map_err(|err| format!("Python config parsing failed: {err}").into())
    }

    fn gen_clang_args(py_config_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let info = PythonInfo::parse(py_config_path)?;
        let ret: Vec<String> = info
            .cpp_path
            .iter()
            .map(|path| format!("-I{path}"))
            .chain(info.cc_flags.split_whitespace().map(ToString::to_string))
            .collect();

        println!("clang args: {ret:?}");

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
        let mut disable_pascal_case = false;
        let mut variant_name = original_variant_name.to_string();
        if let Some(mut enum_name_real) = enum_name {
            enum_name_real = enum_name_real
                .trim_start_matches("enum ")
                .trim_start_matches('_');

            /* 这些类型与大小写有关系，就不转成大驼峰命名了 */
            disable_pascal_case = ["key_code_t"].iter().any(|&s| s == enum_name_real);

            /* 去掉枚举名前缀 */
            if let Some(enum_) = self.idl.enums.get(enum_name_real) {
                if !enum_.prefix.is_empty() {
                    let sanitized = original_variant_name.trim_start_matches(&enum_.prefix);
                    variant_name = sanitized.into();
                }
            } else {
                let prefix = enum_name_real.trim_end_matches("_t").to_uppercase() + "_";
                let sanitized = original_variant_name.trim_start_matches(&prefix);
                variant_name = sanitized.into();
            }
        }
        if !disable_pascal_case {
            /* 将枚举名转为大驼峰命名 */
            variant_name = heck::ToPascalCase::to_pascal_case(variant_name.as_str());
        }
        /* 以数字开头时，前面加下划线 */
        if variant_name
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_digit())
        {
            variant_name = format!("_{variant_name}");
        }
        Some(variant_name)
    }
}

pub struct Builder {
    builder: bindgen::Builder,
}

impl Builder {
    fn new() -> Self {
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
            .fold(b.builder, |b, (class_name, class)| {
                class
                    .methods
                    .iter()
                    .fold(b.allowlist_type(class_name), |inner_b, method| {
                        inner_b.allowlist_function(&method.name)
                    })
            });

        b.builder = idl
            .enums
            .keys()
            .fold(b.builder, bindgen::Builder::allowlist_type);

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
