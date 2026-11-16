mod toast;

pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_plugins((toast::plugin,));
}
