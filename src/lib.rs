use steel::steel_vm::ffi::{FFIModule, FFIValue, RegisterFFIFn};

steel::declare_module!(system_theme_module);

pub fn system_theme_module() -> FFIModule {
    let mut module = FFIModule::new("steel/system_theme");
    module.register_fn("detect", detect);
    module
}

fn detect() -> Result<FFIValue, String> {
    let detected = ::dark_light::detect().map_err(|err| err.to_string())?;
    let theme = match detected {
        dark_light::Mode::Dark => "dark",
        _ => "light",
    };
    Ok(FFIValue::StringV(theme.into()))
}
