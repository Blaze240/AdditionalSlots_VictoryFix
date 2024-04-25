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
unsafe extern "C" fn ptrainer_sound_win1afushigisou(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ptrainer_win_pfushigisou"));
        } else {
            frame(agent.lua_state_agent, 60.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pfushigisou"));
            } else {
                frame(agent.lua_state_agent, 60.0);
                if macros::is_excute(agent) {
                    macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pfushigisou"));
                } else {
                    frame(agent.lua_state_agent, 60.0);
                    if macros::is_excute(agent) {
                        macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pfushigisou"));
                    } else {
                        frame(agent.lua_state_agent, 60.0);
                        if macros::is_excute(agent) {
                            macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pfushigisou"));
                        } else {
                            frame(agent.lua_state_agent, 60.0);
                            if macros::is_excute(agent) {
                                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pfushigisou"));
                            }
                        }
                    }
                }
            }
        }
    } else {
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ptrainer_win_pfushigisou"));
        } else {
            frame(agent.lua_state_agent, 60.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pfushigisou"));
            } else {
                frame(agent.lua_state_agent, 60.0);
                if macros::is_excute(agent) {
                    macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pfushigisou"));
                } else {
                    frame(agent.lua_state_agent, 60.0);
                    if macros::is_excute(agent) {
                        macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pfushigisou"));
                    } else {
                        frame(agent.lua_state_agent, 60.0);
                        if macros::is_excute(agent) {
                            macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pfushigisou"));
                        } else {
                            frame(agent.lua_state_agent, 60.0);
                            if macros::is_excute(agent) {
                                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pfushigisou"));
                            }
                        }
                    }
                }
            }
        }
    }
}

unsafe extern "C" fn ptrainer_sound_win1alizardon(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 55.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ptrainer_win_plizardon"));
        } else {
            frame(agent.lua_state_agent, 60.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_plizardon"));
            } else {
                frame(agent.lua_state_agent, 60.0);
                if macros::is_excute(agent) {
                    macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_plizardon"));
                } else {
                    frame(agent.lua_state_agent, 60.0);
                    if macros::is_excute(agent) {
                        macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_plizardon"));
                    } else {
                        frame(agent.lua_state_agent, 60.0);
                        if macros::is_excute(agent) {
                            macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_plizardon"));
                        } else {
                            frame(agent.lua_state_agent, 60.0);
                            if macros::is_excute(agent) {
                                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_plizardon"));
                            }
                        }
                    }
                }
            }
        }
    } else {
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ptrainer_win_plizardon"));
        } else {
            frame(agent.lua_state_agent, 50.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_plizardon"));
            } else {
                frame(agent.lua_state_agent, 50.0);
                if macros::is_excute(agent) {
                    macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_plizardon"));
                } else {
                    frame(agent.lua_state_agent, 50.0);
                    if macros::is_excute(agent) {
                        macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_plizardon"));
                    } else {
                        frame(agent.lua_state_agent, 50.0);
                        if macros::is_excute(agent) {
                            macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_plizardon"));
                        } else {
                            frame(agent.lua_state_agent, 50.0);
                            if macros::is_excute(agent) {
                                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_plizardon"));
                            }
                        }
                    }
                }
            }
        }
    }
}

unsafe extern "C" fn ptrainer_sound_win1azenigame(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ptrainer_win_pzenigame"));
        } else {
            frame(agent.lua_state_agent, 60.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pzenigame"));
            } else {
                frame(agent.lua_state_agent, 60.0);
                if macros::is_excute(agent) {
                    macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pzenigame"));
                } else {
                    frame(agent.lua_state_agent, 60.0);
                    if macros::is_excute(agent) {
                        macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pzenigame"));
                    } else {
                        frame(agent.lua_state_agent, 60.0);
                        if macros::is_excute(agent) {
                            macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pzenigame"));
                        } else {
                            frame(agent.lua_state_agent, 60.0);
                            if macros::is_excute(agent) {
                                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pzenigame"));
                            }
                        }
                    }
                }
            }
        }
    } else {
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ptrainer_win_pzenigame"));
        } else {
            frame(agent.lua_state_agent, 60.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pzenigame"));
            } else {
                frame(agent.lua_state_agent, 60.0);
                if macros::is_excute(agent) {
                    macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pzenigame"));
                } else {
                    frame(agent.lua_state_agent, 60.0);
                    if macros::is_excute(agent) {
                        macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pzenigame"));
                    } else {
                        frame(agent.lua_state_agent, 60.0);
                        if macros::is_excute(agent) {
                            macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pzenigame"));
                        } else {
                            frame(agent.lua_state_agent, 60.0);
                            if macros::is_excute(agent) {
                                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pzenigame"));
                            }
                        }
                    }
                }
            }
        }
    }
}

unsafe extern "C" fn ptrainer_sound_win1bfushigisou(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 43.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ptrainer_win"));
        } else {
            frame(agent.lua_state_agent, 70.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
            } else {
                frame(agent.lua_state_agent, 70.0);
                if macros::is_excute(agent) {
                    macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
                } else {
                    frame(agent.lua_state_agent, 70.0);
                    if macros::is_excute(agent) {
                        macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
                    } else {
                        frame(agent.lua_state_agent, 70.0);
                        if macros::is_excute(agent) {
                            macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
                        } else {
                            frame(agent.lua_state_agent, 70.0);
                            if macros::is_excute(agent) {
                                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
                            }
                        }
                    }
                }
            }
        }
    } else {
        frame(agent.lua_state_agent, 53.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ptrainer_win"));
        } else {
            frame(agent.lua_state_agent, 70.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
            } else {
                frame(agent.lua_state_agent, 70.0);
                if macros::is_excute(agent) {
                    macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
                } else {
                    frame(agent.lua_state_agent, 70.0);
                    if macros::is_excute(agent) {
                        macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
                    } else {
                        frame(agent.lua_state_agent, 70.0);
                        if macros::is_excute(agent) {
                            macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
                        } else {
                            frame(agent.lua_state_agent, 70.0);
                            if macros::is_excute(agent) {
                                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
                            }
                        }
                    }
                }
            }
        }
    }
}

unsafe extern "C" fn ptrainer_sound_win1blizardon(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 43.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ptrainer_win"));
        } else {
            frame(agent.lua_state_agent, 65.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
            } else {
                frame(agent.lua_state_agent, 65.0);
                if macros::is_excute(agent) {
                    macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
                } else {
                    frame(agent.lua_state_agent, 65.0);
                    if macros::is_excute(agent) {
                        macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
                    } else {
                        frame(agent.lua_state_agent, 65.0);
                        if macros::is_excute(agent) {
                            macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
                        } else {
                            frame(agent.lua_state_agent, 65.0);
                            if macros::is_excute(agent) {
                                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
                            }
                        }
                    }
                }
            }
        }
    } else {
        frame(agent.lua_state_agent, 60.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ptrainer_win"));
        } else {
            frame(agent.lua_state_agent, 70.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
            } else {
                frame(agent.lua_state_agent, 70.0);
                if macros::is_excute(agent) {
                    macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
                } else {
                    frame(agent.lua_state_agent, 70.0);
                    if macros::is_excute(agent) {
                        macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
                    } else {
                        frame(agent.lua_state_agent, 70.0);
                        if macros::is_excute(agent) {
                            macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
                        } else {
                            frame(agent.lua_state_agent, 70.0);
                            if macros::is_excute(agent) {
                                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
                            }
                        }
                    }
                }
            }
        }
    }
}

unsafe extern "C" fn ptrainer_sound_win1bzenigame(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 43.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ptrainer_win"));
        } else {
            frame(agent.lua_state_agent, 70.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
            } else {
                frame(agent.lua_state_agent, 70.0);
                if macros::is_excute(agent) {
                    macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
                } else {
                    frame(agent.lua_state_agent, 70.0);
                    if macros::is_excute(agent) {
                        macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
                    } else {
                        frame(agent.lua_state_agent, 70.0);
                        if macros::is_excute(agent) {
                            macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
                        } else {
                            frame(agent.lua_state_agent, 70.0);
                            if macros::is_excute(agent) {
                                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
                            }
                        }
                    }
                }
            }
        }
    } else {
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ptrainer_win"));
        } else {
            frame(agent.lua_state_agent, 70.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
            } else {
                frame(agent.lua_state_agent, 70.0);
                if macros::is_excute(agent) {
                    macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
                } else {
                    frame(agent.lua_state_agent, 70.0);
                    if macros::is_excute(agent) {
                        macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
                    } else {
                        frame(agent.lua_state_agent, 70.0);
                        if macros::is_excute(agent) {
                            macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
                        } else {
                            frame(agent.lua_state_agent, 70.0);
                            if macros::is_excute(agent) {
                                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win"));
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn install() {
    Agent::new("ptrainer")
        .sound_acmd(
            "sound_win1afushigisou",
            ptrainer_sound_win1afushigisou,
            Priority::Low,
        )
        .sound_acmd(
            "sound_win1alizardon",
            ptrainer_sound_win1alizardon,
            Priority::Low,
        )
        .sound_acmd(
            "sound_win1azenigame",
            ptrainer_sound_win1azenigame,
            Priority::Low,
        )
        .sound_acmd(
            "sound_win1afushigisou",
            ptrainer_sound_win1bfushigisou,
            Priority::Low,
        )
        .sound_acmd(
            "sound_win1afushigisou",
            ptrainer_sound_win1blizardon,
            Priority::Low,
        )
        .sound_acmd(
            "sound_win1afushigisou",
            ptrainer_sound_win1bzenigame,
            Priority::Low,
        )
        .install();
}
