use crate::animation::*;
use crate::game::*;
use crate::shop::*;
use bevy::audio::*;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
/// Countdown timer component for level time limits
pub struct Countdown {
    /// Timer that counts down the remaining time
    pub timer: Timer,
}

#[derive(Debug)]
/// States for the level exit portal
pub enum PortalState {
    /// Portal is closed and inactive
    Closed,
    /// Portal is in the process of opening
    Opening,
    /// Portal is fully open and can be used
    Open,
    /// Portal is in the process of closing
    Closing,
}

#[derive(Component)]
/// Component for the level exit portal
pub struct Portal {
    /// Current state of the portal
    pub portal_state: PortalState,
}

/// System that handles portal state transitions
pub fn portal_update(
    mut commands: Commands,
    portal_comp: Single<(&mut Animation, &mut Portal)>,
    mut game: ResMut<Game>,
) {
    let (mut anim_portal, mut portal) = portal_comp.into_inner();
    // Start openning portal if no bricks left and portal is closed
    if game.bricks_left == 0 && matches!(portal.portal_state, PortalState::Closed) {
        portal.portal_state = PortalState::Opening;
        anim_portal.frozen = false;
        anim_portal.reverse = false;
        commands.spawn((
            AudioPlayer::new(game.sound_portal.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }
    // Portal fully open at the end of the animation
    if matches!(portal.portal_state, PortalState::Opening) && anim_portal.frozen {
        portal.portal_state = PortalState::Open;
        game.portal_open = true;
    }
    // Portal fully closed at the end of the animation
    if matches!(portal.portal_state, PortalState::Closing) && anim_portal.frozen {
        portal.portal_state = PortalState::Closed;
    }
}

/// System that updates the countdown timer and manages portal state based on time
pub fn countdown_update(
    mut commands: Commands,
    countdown_comp: Single<(&mut Text2d, &mut Countdown)>,
    portal_comp: Single<(&mut Animation, &mut Portal)>,
    msg_comp: Single<(&mut Text2d, &mut InfoAreaMsg), Without<Countdown>>,
    mut game: ResMut<Game>,
) {
    let dt = game.avg_delta;

    // Countdown
    let (mut text, mut countdown) = countdown_comp.into_inner();
    countdown.timer.tick(Duration::from_secs_f32(dt));

    if game.seconds_left != countdown.timer.remaining_secs().ceil() {
        text.clear();
        game.seconds_left = countdown.timer.remaining_secs().ceil();
        text.push_str(format!("{:.0}", game.seconds_left).as_str());
    }

    // Close portal if level is unlocked and countdown done
    if countdown.timer.just_finished() && level_is_unlocked(game.current_level, &game) {
        let (mut anim_portal, mut portal) = portal_comp.into_inner();
        anim_portal.frozen = false;
        anim_portal.reverse = true;
        portal.portal_state = PortalState::Closing;
        game.portal_open = false;
        commands.spawn((
            AudioPlayer::new(game.sound_portal.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }

    // Clear up infoare message after a few seconds
    let (mut text, mut text_timer) = msg_comp.into_inner();
    text_timer.timer.tick(Duration::from_secs_f32(dt));
    if text_timer.timer.just_finished() {
        *text = Text2d::new("");
    }
}
