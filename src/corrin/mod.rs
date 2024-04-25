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

unsafe extern "C" fn kamui_sound_win1a(agent: &mut L2CAgentBase) {
    let x = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if x == 8 || x == 10 {
        frame(agent.lua_state_agent, 53.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win01"));
        }
        frame(agent.lua_state_agent, 79.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win01_02"));
        }
        frame(agent.lua_state_agent, 120.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_kamui_win01"));
        }
        frame(agent.lua_state_agent, 127.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_kamui_win01_03"));
        }
    } else if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2
        == 0
    {
        frame(agent.lua_state_agent, 53.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win01"));
        }
        frame(agent.lua_state_agent, 79.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win01_02"));
        }
        frame(agent.lua_state_agent, 90.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_kamui_win01"));
        }
        frame(agent.lua_state_agent, 127.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win01_03"));
        }
    } else {
        frame(agent.lua_state_agent, 53.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win01"));
        }
        frame(agent.lua_state_agent, 79.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win01_02"));
        }
        frame(agent.lua_state_agent, 120.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_kamui_win01"));
        }
        frame(agent.lua_state_agent, 127.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_kamui_win01_03"));
        }
    }
}

unsafe extern "C" fn kamui_sound_win2a(agent: &mut L2CAgentBase) {
    let x = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if x == 8 || x == 10 {
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_kamui_win02"));
        }
        frame(agent.lua_state_agent, 46.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_kamui_win02_02"));
        }
        frame(agent.lua_state_agent, 70.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win02_03"));
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_kamui_win02"));
        }
        frame(agent.lua_state_agent, 127.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win02_04"));
        }
    } else if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2
        == 0
    {
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_kamui_win02"));
        }
        frame(agent.lua_state_agent, 46.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_kamui_win02_02"));
        }
        frame(agent.lua_state_agent, 70.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win02_03"));
        }
        frame(agent.lua_state_agent, 85.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_kamui_win02"));
        }
        frame(agent.lua_state_agent, 127.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win02_04"));
        }
    } else {
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_kamui_win02"));
        }
        frame(agent.lua_state_agent, 46.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_kamui_win02_02"));
        }
        frame(agent.lua_state_agent, 70.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win02_03"));
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_kamui_win02"));
        }
        frame(agent.lua_state_agent, 127.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win02_04"));
        }
    }
}

unsafe extern "C" fn kamui_sound_win3a(agent: &mut L2CAgentBase) {
    let x = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if x == 8 || x == 10 {
        frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_appeal_h01"));
        }
        frame(agent.lua_state_agent, 63.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_kamui_win03"));
        }
        frame(agent.lua_state_agent, 71.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win03"));
        }
        frame(agent.lua_state_agent, 112.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_kamui_win03b"));
        }
    } else if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2
        == 0
    {
        frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_appeal_h01"));
        }
        frame(agent.lua_state_agent, 63.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_kamui_win03"));
        }
        frame(agent.lua_state_agent, 71.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win03"));
        }
        frame(agent.lua_state_agent, 112.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_kamui_win03b"));
        }
    } else {
        frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_appeal_h01"));
        }
        frame(agent.lua_state_agent, 63.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_kamui_win03"));
        }
        frame(agent.lua_state_agent, 71.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_kamui_win03"));
        }
        frame(agent.lua_state_agent, 112.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_kamui_win03b"));
        }
    }
}

pub fn install() {
    Agent::new("kamui")
        .sound_acmd("sound_win1a", kamui_sound_win1a, Priority::Low)
        .sound_acmd("sound_win2a", kamui_sound_win2a, Priority::Low)
        .sound_acmd("sound_win3a", kamui_sound_win3a, Priority::Low)
        .install();
}
