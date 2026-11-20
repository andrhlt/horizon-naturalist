use super::Navigation;
use crate::mobs::Mob;
use horizon_core::random::RandomSource;

pub const KIND: &str = "naturalist:clam";

#[derive(Clone, Debug, Default)]
pub struct Shell {
    pub open: bool,
    pub has_treasure: bool,
    pub state_timer: i32,
    pub snap_cooldown: i32,
    pub launch_delay: i32,
}

pub fn state(nav: &Navigation) -> &Shell {
    nav.__horizon_mixins_naturalist_clam()
}

pub fn state_mut(nav: &mut Navigation) -> &mut Shell {
    nav.__horizon_mixins_naturalist_clam_mut()
}

pub fn set_open(mob: &mut Mob, open: bool) {
    let timer = if open {
        100 + mob.random.next_int_bound(101).unwrap_or(0)
    } else {
        200 + mob.random.next_int_bound(201).unwrap_or(0)
    };
    let shell = state_mut(&mut mob.ai.nav);
    shell.open = open;
    shell.state_timer = timer;
}

#[derive(Default)]
pub struct Signals {
    pub slam: bool,
    pub launch: bool,
}

pub fn advance(mob: &mut Mob, near_closed: bool, near_treasure: bool) -> Signals {
    let shell = state_mut(&mut mob.ai.nav);
    shell.snap_cooldown = (shell.snap_cooldown - 1).max(0);
    let launch = shell.launch_delay == 1;
    shell.launch_delay = (shell.launch_delay - 1).max(0);
    if shell.snap_cooldown == 0 {
        if shell.open && shell.has_treasure && near_treasure {
            set_open(mob, false);
            return Signals { slam: true, launch };
        }
        if !shell.open && near_closed {
            shell.snap_cooldown = 60;
            shell.launch_delay = 4;
            set_open(mob, true);
            return Signals { slam: false, launch };
        }
    }
    shell.state_timer -= 1;
    if shell.state_timer <= 0 {
        let opening = !shell.open;
        set_open(mob, opening);
    }
    Signals { slam: false, launch }
}
