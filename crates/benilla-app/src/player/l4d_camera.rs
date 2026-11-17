//! L4D over-the-shoulder aim for the fork.
//!
//! The camera sits in an over-the-shoulder view by default. Right mouse tightens it for a shot;
//! left click swings at the current selection while it is up. `L` turns the mode off for players
//! who want the reference camera back. The rig math lives in [`super::camera::seat_camera`], so
//! with the blend at zero nothing differs from 1.12.

use bevy::input::mouse::MouseButton;
use bevy::prelude::*;

use crate::net::{ClientCommand, NetCommands, SelfPlayer};
use crate::target::{AttackNearestRequest, Selection};

use super::camera::CameraControl;

/// The mode's on/off switch. Default on: this fork exists to play the L4D mode.
#[derive(Resource)]
pub(crate) struct L4dAim {
    pub(crate) enabled: bool,
}

impl Default for L4dAim {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Ease the camera into and out of the shoulder view. Runs before the controller seats the rig.
pub(crate) fn update_aim(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut mode: ResMut<L4dAim>,
    mut rig: ResMut<CameraControl>,
) {
    if keys.just_pressed(KeyCode::KeyL) {
        mode.enabled = !mode.enabled;
    }
    // The mode sits in the shoulder view by default, and right mouse tightens it for a shot.
    // `L` returns the reference third-person camera.
    let target = if !mode.enabled {
        0.0
    } else if mouse.pressed(MouseButton::Right) {
        1.0
    } else {
        0.85
    };
    let rate = if target > rig.aim { 10.0 } else { 6.0 };
    let step = (rate * time.delta_secs()).max(0.0);
    rig.aim = (rig.aim + (target - rig.aim).clamp(-step, step)).clamp(0.0, 1.0);
}

/// FPS fire: while the shoulder view is up, left click attacks the current selection. This keeps
/// the stock click-to-select and right-click camera behaviour outside aim.
pub(crate) fn fire(
    rig: Res<CameraControl>,
    mouse: Res<ButtonInput<MouseButton>>,
    selection: Res<Selection>,
    net: Res<NetCommands>,
    mut acquire: MessageWriter<AttackNearestRequest>,
    self_player: Query<(), With<SelfPlayer>>,
) {
    if self_player.is_empty() || rig.aim < 0.5 || !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    if let Some(guid) = selection.guid {
        let _ = net.0.send(ClientCommand::AttackSwing { guid });
    } else {
        acquire.write(AttackNearestRequest);
    }
}
