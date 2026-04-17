// // build.rs
fn main() {
//     let mut build = cc::Build::new();
//     build.include("src/apps/doom_c");
//
//     let c_files = [
//         "am_map.c", "doomdef.c", "doomgeneric.c", "doomstat.c", "dstrings.c", "d_event.c",
//         "d_items.c", "d_iwad.c", "d_loop.c", "d_main.c", "d_mode.c", "d_net.c", "f_finale.c",
//         "f_wipe.c", "g_game.c", "hu_lib.c", "hu_stuff.c", "info.c", "i_main.c", "i_net.c",
//         "i_system.c", "i_timer.c", "i_video.c", "m_argv.c", "m_bbox.c", "m_cheat.c",
//         "m_config.c", "m_controls.c", "m_fixed.c", "m_menu.c", "m_misc.c", "m_random.c",
//         "p_ceilng.c", "p_doors.c", "p_enemy.c", "p_floor.c", "p_inter.c", "p_lights.c",
//         "p_map.c", "p_maputl.c", "p_mobj.c", "p_plats.c", "p_pspr.c", "p_saveg.c",
//         "p_setup.c", "p_sight.c", "p_spec.c", "p_switch.c", "p_telept.c", "p_tick.c",
//         "p_user.c", "r_bsp.c", "r_data.c", "r_draw.c", "r_main.c", "r_plane.c", "r_segs.c",
//         "r_sky.c", "r_things.c", "sounds.c", "st_lib.c", "st_stuff.c", "s_sound.c",
//         "tables.c", "v_video.c", "wi_stuff.c", "w_wad.c", "z_zone.c", "w_main.c",
//         "w_file.c", "w_file_stdc.c", "w_checksum.c", "dummy.c", "statdump.c", "i_endoom.c",
//         "i_input.c", "sha1.c",
//     ];
//
//     for file in c_files.iter() {
//         let path = format!("src/apps/doom_c/{}", file);
//         if std::path::Path::new(&path).exists() {
//             build.file(path);
//         }
//     }
//
//     build.define("DOOMGENERIC", None);
//     build.compile("doom");
}
