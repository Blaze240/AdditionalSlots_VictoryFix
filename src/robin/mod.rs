use {
    smash::{
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::{lua_const::*, L2CAgent, L2CValue},
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::*,
};

unsafe extern "C" fn reflet_sound_win1a(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 40.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_reflet_win01"));
        }
        frame(agent.lua_state_agent, 46.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_fire_02_win01"));
        }
        frame(agent.lua_state_agent, 91.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_special_s02_win01"));
            macros::STOP_SE(agent, Hash40::new("se_reflet_fire_02_win01"));
        }
        frame(agent.lua_state_agent, 125.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_win1"));
            macros::PLAY_SE(agent, Hash40::new("se_common_swing_03"));
        }
    } else {
        frame(agent.lua_state_agent, 46.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_fire_02_win01"));
        }
        frame(agent.lua_state_agent, 48.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_reflet_win01"));
        }
        frame(agent.lua_state_agent, 91.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_special_s02_win01"));
            macros::STOP_SE(agent, Hash40::new("se_reflet_fire_02_win01"));
        }
        frame(agent.lua_state_agent, 125.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_win1"));
            macros::PLAY_SE(agent, Hash40::new("se_common_swing_03"));
        }
    }
}

unsafe extern "C" fn reflet_sound_win1b(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 40.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_reflet_win_lucina"));
        }
        frame(agent.lua_state_agent, 46.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_fire_02_win01"));
        }
        frame(agent.lua_state_agent, 91.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_special_s02_win01"));
            macros::STOP_SE(agent, Hash40::new("se_reflet_fire_02_win01"));
        }
        frame(agent.lua_state_agent, 125.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_win1"));
            macros::PLAY_SE(agent, Hash40::new("se_common_swing_03"));
        }
    } else {
        frame(agent.lua_state_agent, 46.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_fire_02_win01"));
        }
        frame(agent.lua_state_agent, 91.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_special_s02_win01"));
            macros::STOP_SE(agent, Hash40::new("se_reflet_fire_02_win01"));
        }
        frame(agent.lua_state_agent, 125.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_win1"));
            macros::PLAY_SE(agent, Hash40::new("se_common_swing_03"));
        }
    }
}

unsafe extern "C" fn reflet_sound_win2a(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 47.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_reflet_win02"));
        }
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_win2_01"));
        }
        frame(agent.lua_state_agent, 123.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_smash_s01_win02"));
        }
    } else {
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_reflet_win02"));
        }
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_win2_01"));
        }
        frame(agent.lua_state_agent, 123.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_smash_s01_win02"));
        }
    }
}

unsafe extern "C" fn reflet_sound_win3a(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 25.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_reflet_win03"));
        }
        frame(agent.lua_state_agent, 97.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_win1"));
        }
        frame(agent.lua_state_agent, 117.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_win3_01"));
        }
        frame(agent.lua_state_agent, 120.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_smash_s01_win03"));
        }
    } else {
        frame(agent.lua_state_agent, 80.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_reflet_win03"));
        }
        frame(agent.lua_state_agent, 97.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_win1"));
        }
        frame(agent.lua_state_agent, 117.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_win3_01"));
        }
        frame(agent.lua_state_agent, 120.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_smash_s01_win03"));
        }
    }
}

unsafe extern "C" fn reflet_sound_win3b(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 40.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_reflet_win03_chrom"));
        }
        frame(agent.lua_state_agent, 97.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_win1"));
        }
        frame(agent.lua_state_agent, 117.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_win3_01"));
        }
        frame(agent.lua_state_agent, 120.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_smash_s01_win03"));
        }
    } else {
        frame(agent.lua_state_agent, 0.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_reflet_win_lucina"));
        }
        frame(agent.lua_state_agent, 97.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_win1"));
        }
        frame(agent.lua_state_agent, 117.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_win3_01"));
        }
        frame(agent.lua_state_agent, 120.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_smash_s01_win03"));
        }
    }
}
pub fn install() {
    Agent::new("reflet")
        .sound_acmd("sound_win1a", reflet_sound_win1a, Priority::Low)
        .sound_acmd("sound_win1b", reflet_sound_win1b, Priority::Low)
        .sound_acmd("sound_win2a", reflet_sound_win2a, Priority::Low)
        .sound_acmd("sound_win3a", reflet_sound_win3a, Priority::Low)
        .sound_acmd("sound_win3b", reflet_sound_win3b, Priority::Low)
        .install();
}
