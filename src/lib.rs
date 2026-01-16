use steel::steel_vm::ffi::{FFIModule, FFIValue, RegisterFFIFn};

steel::declare_module!(system_theme_module);

pub fn system_theme_module() -> FFIModule {
    let mut module = FFIModule::new("steel/system_theme");
    module.register_fn("detect", detect);
    module
}

fn get_sys_theme() -> String {
    let detected = ::dark_light::detect();
    match detected {
        Ok(dark_light::Mode::Dark) => "dark".to_string(),
        Ok(dark_light::Mode::Light) | Ok(dark_light::Mode::Unspecified) => "light".to_string(),
        Err(e) => e.to_string(),
    }
}

fn detect() -> FFIValue {
    let theme = get_sys_theme();
    FFIValue::StringV(theme.into())
}

#[cfg(test)]
mod test {

    #[test]
    fn test_detect() {
        let theme = super::get_sys_theme();
        assert_eq!(theme, "light");
    }
}
