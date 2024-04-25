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
unsafe extern "C" fn wiifit_sound_win2a(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win01"));
        }
        frame(agent.lua_state_agent, 54.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_attackair_h01"));
        }
        frame(agent.lua_state_agent, 119.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    } else {
        frame(agent.lua_state_agent, 0.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win02"));
        }
        frame(agent.lua_state_agent, 54.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_wiifit_attackair_h01"));
        }
        frame(agent.lua_state_agent, 119.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    }
}

unsafe extern "C" fn wiifit_sound_win2b(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win05"));
        }
        frame(agent.lua_state_agent, 54.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_attackair_h01"));
        }
        frame(agent.lua_state_agent, 119.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    } else {
        frame(agent.lua_state_agent, 0.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win03"));
        }
        frame(agent.lua_state_agent, 54.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_wiifit_attackair_h01"));
        }
        frame(agent.lua_state_agent, 119.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    }
}

unsafe extern "C" fn wiifit_sound_win2c(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 0.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win06"));
        }
        frame(agent.lua_state_agent, 54.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_attackair_h01"));
        }
        frame(agent.lua_state_agent, 119.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    } else {
        frame(agent.lua_state_agent, 0.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win04"));
        }
        frame(agent.lua_state_agent, 54.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_wiifit_attackair_h01"));
        }
        frame(agent.lua_state_agent, 119.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    }
}

unsafe extern "C" fn wiifit_sound_win2d(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 0.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win13"));
        }
        frame(agent.lua_state_agent, 54.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_attackair_h01"));
        }
        frame(agent.lua_state_agent, 119.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    } else {
        frame(agent.lua_state_agent, 0.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win07"));
        }
        frame(agent.lua_state_agent, 54.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_wiifit_attackair_h01"));
        }
        frame(agent.lua_state_agent, 119.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    }
}

unsafe extern "C" fn wiifit_sound_win2e(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 0.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win16"));
        }
        frame(agent.lua_state_agent, 54.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_attackair_h01"));
        }
        frame(agent.lua_state_agent, 119.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    } else {
        frame(agent.lua_state_agent, 0.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win13"));
        }
        frame(agent.lua_state_agent, 54.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_wiifit_attackair_h01"));
        }
        frame(agent.lua_state_agent, 119.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    }
}

unsafe extern "C" fn wiifit_sound_win2f(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 54.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_attackair_h01"));
        }
        frame(agent.lua_state_agent, 119.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    } else {
        frame(agent.lua_state_agent, 0.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win14"));
        }
        frame(agent.lua_state_agent, 54.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_wiifit_attackair_h01"));
        }
        frame(agent.lua_state_agent, 119.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    }
}

unsafe extern "C" fn wiifit_sound_win2g(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 54.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_attackair_h01"));
        }
        frame(agent.lua_state_agent, 119.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    } else {
        frame(agent.lua_state_agent, 0.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win19"));
        }
        frame(agent.lua_state_agent, 54.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_wiifit_attackair_h01"));
        }
        frame(agent.lua_state_agent, 119.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    }
}

pub fn install() {
    Agent::new("wiifit")
        .sound_acmd("sound_win2a", wiifit_sound_win2a, Priority::Low)
        .sound_acmd("sound_win2b", wiifit_sound_win2b, Priority::Low)
        .sound_acmd("sound_win2c", wiifit_sound_win2c, Priority::Low)
        .sound_acmd("sound_win2d", wiifit_sound_win2d, Priority::Low)
        .sound_acmd("sound_win2e", wiifit_sound_win2e, Priority::Low)
        .sound_acmd("sound_win2f", wiifit_sound_win2f, Priority::Low)
        .sound_acmd("sound_win2g", wiifit_sound_win2g, Priority::Low)
        .install();
}
