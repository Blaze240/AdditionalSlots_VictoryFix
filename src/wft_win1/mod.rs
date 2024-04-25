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
unsafe extern "C" fn wiifit_sound_win1a(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win02"));
        }
        frame(agent.lua_state_agent, 60.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_h01"));
        }
        frame(agent.lua_state_agent, 124.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    } else {
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win01"));
        }
        frame(agent.lua_state_agent, 60.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_wiifit_smash_h01"));
        }
        frame(agent.lua_state_agent, 124.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    }
}

unsafe extern "C" fn wiifit_sound_win1b(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 0.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win03"));
        }
        frame(agent.lua_state_agent, 60.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_h01"));
        }
        frame(agent.lua_state_agent, 124.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    } else {
        frame(agent.lua_state_agent, 0.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win06"));
        }
        frame(agent.lua_state_agent, 60.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_wiifit_smash_h01"));
        }
        frame(agent.lua_state_agent, 124.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    }
}

unsafe extern "C" fn wiifit_sound_win1c(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 40.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win04"));
        }
        frame(agent.lua_state_agent, 60.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_h01"));
        }
        frame(agent.lua_state_agent, 124.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    } else {
        frame(agent.lua_state_agent, 40.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win10"));
        }
        frame(agent.lua_state_agent, 60.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_wiifit_smash_h01"));
        }
        frame(agent.lua_state_agent, 124.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    }
}

unsafe extern "C" fn wiifit_sound_win1d(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 0.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win07"));
        }
        frame(agent.lua_state_agent, 60.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_h01"));
        }
        frame(agent.lua_state_agent, 124.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    } else {
        frame(agent.lua_state_agent, 0.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win16"));
        }
        frame(agent.lua_state_agent, 60.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_wiifit_smash_h01"));
        }
        frame(agent.lua_state_agent, 124.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    }
}

unsafe extern "C" fn wiifit_sound_win1e(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 30.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win08"));
        }
        frame(agent.lua_state_agent, 60.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_h01"));
        }
        frame(agent.lua_state_agent, 124.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    } else {
        frame(agent.lua_state_agent, 60.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_wiifit_smash_h01"));
        }
        frame(agent.lua_state_agent, 124.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    }
}

unsafe extern "C" fn wiifit_sound_win1f(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 60.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_wiifit_smash_h01"));
        }
        frame(agent.lua_state_agent, 63.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_wiifit_win19"));
        }
        frame(agent.lua_state_agent, 124.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    } else {
        frame(agent.lua_state_agent, 60.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_wiifit_smash_h01"));
        }
        frame(agent.lua_state_agent, 124.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    }
}

pub fn install() {
    Agent::new("wiifit")
        .sound_acmd("sound_win1a", wiifit_sound_win1a, Priority::Low)
        .sound_acmd("sound_win1b", wiifit_sound_win1b, Priority::Low)
        .sound_acmd("sound_win1c", wiifit_sound_win1c, Priority::Low)
        .sound_acmd("sound_win1d", wiifit_sound_win1d, Priority::Low)
        .sound_acmd("sound_win1e", wiifit_sound_win1e, Priority::Low)
        .sound_acmd("sound_win1f", wiifit_sound_win1f, Priority::Low)
        .install();
}
