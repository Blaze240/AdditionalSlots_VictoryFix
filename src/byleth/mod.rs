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
unsafe extern "C" fn master_sound_win1(agent: &mut L2CAgentBase) {
    let x = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if x == 16 || x == 18 || x == 20 || x == 22 || x == 30 || x == 32 {
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win02_02"));
        }
        frame(agent.lua_state_agent, 30.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win01_01"));
        }
        frame(agent.lua_state_agent, 41.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_master_win01"));
        }
        frame(agent.lua_state_agent, 64.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win01_02"));
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win01_03"));
        }
    } else if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2
        == 0
    {
        frame(agent.lua_state_agent, 28.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win01_01"));
        }
        frame(agent.lua_state_agent, 41.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_master_win01"));
        }
    } else {
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win02_02"));
        }
        frame(agent.lua_state_agent, 30.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win01_01"));
        }
        frame(agent.lua_state_agent, 41.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_master_win01"));
        }
        frame(agent.lua_state_agent, 64.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win01_02"));
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win01_03"));
        }
    }
}

unsafe extern "C" fn master_sound_win2(agent: &mut L2CAgentBase) {
    let x = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if x == 16 || x == 18 || x == 20 || x == 22 || x == 30 || x == 32 {
        frame(agent.lua_state_agent, 40.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win02_01"));
        }
        frame(agent.lua_state_agent, 45.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_master_win02"));
        }
        frame(agent.lua_state_agent, 93.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win02_02"));
        }
        frame(agent.lua_state_agent, 125.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win02_03"));
        }
    } else if x % 2 == 0 {
        frame(agent.lua_state_agent, 35.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_master_win02"));
        }
        frame(agent.lua_state_agent, 40.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win02_01"));
        }
        frame(agent.lua_state_agent, 95.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win02_02"));
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_cloth_m"));
        }
        frame(agent.lua_state_agent, 125.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win02_03"));
        }
    } else {
        frame(agent.lua_state_agent, 40.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win02_01"));
        }
        frame(agent.lua_state_agent, 45.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_master_win02"));
        }
        frame(agent.lua_state_agent, 93.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win02_02"));
        }
        frame(agent.lua_state_agent, 125.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win02_03"));
        }
    }
}

unsafe extern "C" fn master_sound_win3(agent: &mut L2CAgentBase) {
    let x = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if x == 16 || x == 18 || x == 20 || x == 22 || x == 30 || x == 32 {
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win03_02"));
        }
        frame(agent.lua_state_agent, 57.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win01_03"));
        }
        frame(agent.lua_state_agent, 59.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_master_win03"));
        }
        frame(agent.lua_state_agent, 98.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win02_03"));
        }
    } else if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2
        == 0
    {
        frame(agent.lua_state_agent, 52.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win03_01"));
        }
        frame(agent.lua_state_agent, 59.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_master_win03"));
        }
        frame(agent.lua_state_agent, 160.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win03_02"));
        }
    } else {
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win03_02"));
        }
        frame(agent.lua_state_agent, 57.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win01_03"));
        }
        frame(agent.lua_state_agent, 59.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_master_win03"));
        }
        frame(agent.lua_state_agent, 98.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win02_03"));
        }
    }
}

pub fn install() {
    Agent::new("master")
        .sound_acmd("sound_win1", master_sound_win1, Priority::Low)
        .sound_acmd("sound_win2", master_sound_win2, Priority::Low)
        .sound_acmd("sound_win3", master_sound_win3, Priority::Low)
        .install();
}
