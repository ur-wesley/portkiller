use tauri::WebviewWindow;

pub fn apply_glass(window: &WebviewWindow) -> Result<(), tauri::Error> {
    #[cfg(target_os = "windows")]
    {
        use tauri::window::{Color, Effect, EffectsBuilder};
        window.set_effects(
            EffectsBuilder::new()
                .effect(Effect::Acrylic)
                .color(Color(9, 9, 11, 140))
                .build(),
        )?;
    }

    #[cfg(target_os = "macos")]
    {
        use tauri::window::{Effect, EffectState, EffectsBuilder};
        window.set_effects(
            EffectsBuilder::new()
                .effect(Effect::UnderWindowBackground)
                .state(EffectState::Active)
                .build(),
        )?;
    }

    Ok(())
}
