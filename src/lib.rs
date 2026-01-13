use steel::steel_vm::ffi::{FFIModule, FFIValue, RegisterFFIFn};

steel::declare_module!(system_theme_module);

pub fn system_theme_module() -> FFIModule {
    let mut module = FFIModule::new("steel/system_theme");
    module.register_fn("detect", detect);
    module
}

fn detect() -> FFIValue {
    let system_theme = ::system_theme::SystemTheme::new().unwrap();
    match system_theme.theme_scheme() {
        Ok(::system_theme::ThemeScheme::Dark) => FFIValue::StringV("dark".into()),
        _ => FFIValue::StringV("light".into()),
    }
}
