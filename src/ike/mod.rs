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

unsafe extern "C" fn ike_sound_win1(agent: &mut L2CAgentBase) {
    let x = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if x % 2 == 0 {
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ike_win01"));
        }
        frame(agent.lua_state_agent, 38.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ike_special_s03"));
        }
        frame(agent.lua_state_agent, 71.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ike_special_s03"));
        }
        frame(agent.lua_state_agent, 122.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ike_squat"));
        }
    } else {
        frame(agent.lua_state_agent, 38.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ike_special_s03"));
        }
        frame(agent.lua_state_agent, 45.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ike_win01"));
        }
        frame(agent.lua_state_agent, 71.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ike_special_s03"));
        }
        frame(agent.lua_state_agent, 122.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ike_squat"));
        }
    }
}
unsafe extern "C" fn ike_sound_win2(agent: &mut L2CAgentBase) {
    let x = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if x % 2 == 0 {
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ike_swing_l"));
        }
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ike_swordgroundhit"));
        }
        frame(agent.lua_state_agent, 53.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ike_win02"));
        }
    } else {
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ike_swing_l"));
        }
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ike_swordgroundhit"));
        }
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ike_win02"));
        }
    }
}
unsafe extern "C" fn ike_sound_win3(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 30.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ike_win03"));
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_ike_special_h01"));
        }
        frame(agent.lua_state_agent, 78.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_ike_jump01"));
        }
        frame(agent.lua_state_agent, 89.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_ike_special_h04"));
        }
        frame(agent.lua_state_agent, 124.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ike_special_h06"));
        }
    } else {
        frame(agent.lua_state_agent, 30.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_ike_special_h01"));
        }
        frame(agent.lua_state_agent, 40.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ike_win03"));
        }
        frame(agent.lua_state_agent, 78.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_ike_jump01"));
        }
        frame(agent.lua_state_agent, 89.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_ike_special_h04"));
        }
        frame(agent.lua_state_agent, 124.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ike_special_h06"));
        }
    }
}
pub fn install() {
    Agent::new("ike")
        .sound_acmd("sound_win1", ike_sound_win1,Priority::Low)
        .sound_acmd("sound_win2", ike_sound_win2,Priority::Low)
        .sound_acmd("sound_win3", ike_sound_win3,Priority::Low)
        .install();
}
