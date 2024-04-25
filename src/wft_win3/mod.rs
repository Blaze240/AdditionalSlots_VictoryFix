use {
    smash::{
        app::{lua_bind::*, sv_animcmd::*, *},
        hash40,
        lib::{lua_const::*, L2CAgent, L2CValue},
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::*,
};
unsafe extern "C" fn wiifit_sound_win3a(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 0.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win10"));
        }
        frame(agent.lua_state_agent, 126.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_swing_01"));
        }
    } else {
        frame(agent.lua_state_agent, 40.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win08"));
        }
        frame(agent.lua_state_agent, 126.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_common_swing_01"));
        }
    }
}

unsafe extern "C" fn wiifit_sound_win3b(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 28.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win15"));
        }
        frame(agent.lua_state_agent, 126.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_swing_01"));
        }
    } else {
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win15"));
        }
        frame(agent.lua_state_agent, 126.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_common_swing_01"));
        }
    }
}

unsafe extern "C" fn wiifit_sound_win3c(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win18"));
        }
        frame(agent.lua_state_agent, 126.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_swing_01"));
        }
    } else {
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win18"));
        }
        frame(agent.lua_state_agent, 126.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_common_swing_01"));
        }
    }
}

unsafe extern "C" fn wiifit_sound_win3d(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 42.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win20"));
        }
        frame(agent.lua_state_agent, 126.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_swing_01"));
        }
    } else {
        frame(agent.lua_state_agent, 40.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win20"));
        }
        frame(agent.lua_state_agent, 126.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_common_swing_01"));
        }
    }
}

unsafe extern "C" fn wiifit_sound_win3e(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win21"));
        }
        frame(agent.lua_state_agent, 126.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_swing_01"));
        }
    } else {
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win21"));
        }
        frame(agent.lua_state_agent, 126.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_common_swing_01"));
        }
    }
}

pub fn install() {
    Agent::new("wiifit")
        .sound_acmd("sound_win3a", wiifit_sound_win3a, Priority::Low)
        .sound_acmd("sound_win3b", wiifit_sound_win3b, Priority::Low)
        .sound_acmd("sound_win3c", wiifit_sound_win3c, Priority::Low)
        .sound_acmd("sound_win3d", wiifit_sound_win3d, Priority::Low)
        .sound_acmd("sound_win3e", wiifit_sound_win3e, Priority::Low)
        .install();
}
