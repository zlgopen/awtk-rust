use crate::{args::Args, idl::Idl};
use pyo3::{
    ffi::c_str,
    marker::Python,
    types::{PyAnyMethods, PyModule},
};
use std::{error::Error, ffi::CString, fs};

struct CompileInfo {
    cpp_path: Vec<String>,
    cc_flags: String,
}

fn get_info_from_py(file_path: &str) -> Result<CompileInfo, Box<dyn Error>> {
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
        let info = CompileInfo {
            cpp_path: py_module.getattr("CPPPATH")?.extract()?,
            cc_flags: py_module.getattr("CCFLAGS")?.extract()?,
        };

        std::env::set_current_dir(current_dir)?;

        Ok::<CompileInfo, Box<dyn Error>>(info)
    })
    .map_err(Into::into)
}

fn gen_clang_args(py_config_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut ret: Vec<String> = Vec::new();
    let info = get_info_from_py(py_config_path)?;

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
            .fold(b.builder, |bld, (class_name, methods)| {
                let bld = bld.allowlist_type(class_name);
                methods.iter().fold(bld, |inner_bld, method| {
                    inner_bld.allowlist_function(method)
                })
            });

        b.builder
            .header(&args.header_path)
            .clang_args(gen_clang_args(&args.py_config_path)?)
            .generate()?
            .write_to_file(&args.out_path)?;

        Ok(())
    }
}
