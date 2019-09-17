//---------------------------------------------------------------------------//
// Copyright (c) 2017-2019 Ismael Gutiérrez González. All rights reserved.
// 
// This file is part of the Rusted PackFile Manager (RPFM) project,
// which can be found here: https://github.com/Frodo45127/rpfm.
// 
// This file is licensed under the MIT license, which can be found here:
// https://github.com/Frodo45127/rpfm/blob/master/LICENSE.
//---------------------------------------------------------------------------//

/*!
Module with all the code to setup the tips (in the `StatusBar`) for the actions in `AppUI`.
!*/

use crate::QString;
use crate::app_ui::AppUI;

/// This function sets the status bar tip for all the actions in the provided `AppUI`.
pub fn set_tips(app_ui: &AppUI) {

    //-----------------------------------------------//
    // `PackFile` menu tips.
    //-----------------------------------------------//
    unsafe { app_ui.packfile_new_packfile.as_mut().unwrap().set_status_tip(&QString::from_std_str("Creates a new PackFile and open it. Remember to save it later if you want to keep it!")); }
    unsafe { app_ui.packfile_open_packfile.as_mut().unwrap().set_status_tip(&QString::from_std_str("Open an existing PackFile, or multiple existing PackFiles into one.")); }
    unsafe { app_ui.packfile_save_packfile.as_mut().unwrap().set_status_tip(&QString::from_std_str("Save the changes made in the currently open PackFile to disk.")); }
    unsafe { app_ui.packfile_save_packfile_as.as_mut().unwrap().set_status_tip(&QString::from_std_str("Save the currently open PackFile as a new PackFile, instead of overwriting the original one.")); }
    unsafe { app_ui.packfile_load_all_ca_packfiles.as_mut().unwrap().set_status_tip(&QString::from_std_str("Try to load every PackedFile from every vanilla PackFile of the selected game into RPFM at the same time, using lazy-loading to load the PackedFiles. Keep in mind that if you try to save it, your PC may die.")); }
    unsafe { app_ui.packfile_preferences.as_mut().unwrap().set_status_tip(&QString::from_std_str("Open the Preferences/Settings dialog.")); }
    unsafe { app_ui.packfile_quit.as_mut().unwrap().set_status_tip(&QString::from_std_str("Exit the Program.")); }
    
    unsafe { app_ui.change_packfile_type_boot.as_mut().unwrap().set_status_tip(&QString::from_std_str("Changes the PackFile's Type to Boot. You should never use it.")); }
    unsafe { app_ui.change_packfile_type_release.as_mut().unwrap().set_status_tip(&QString::from_std_str("Changes the PackFile's Type to Release. You should never use it.")); }
    unsafe { app_ui.change_packfile_type_patch.as_mut().unwrap().set_status_tip(&QString::from_std_str("Changes the PackFile's Type to Patch. You should never use it.")); }
    unsafe { app_ui.change_packfile_type_mod.as_mut().unwrap().set_status_tip(&QString::from_std_str("Changes the PackFile's Type to Mod. You should use this for mods that should show up in the Mod Manager.")); }
    unsafe { app_ui.change_packfile_type_movie.as_mut().unwrap().set_status_tip(&QString::from_std_str("Changes the PackFile's Type to Movie. You should use this for mods that'll always be active, and will not show up in the Mod Manager.")); }
    unsafe { app_ui.change_packfile_type_other.as_mut().unwrap().set_status_tip(&QString::from_std_str("Changes the PackFile's Type to Other. This is for PackFiles without write support, so you should never use it.")); }

    unsafe { app_ui.change_packfile_type_data_is_encrypted.as_mut().unwrap().set_status_tip(&QString::from_std_str("If checked, the data of the PackedFiles in this PackFile is encrypted. Saving this kind of PackFiles is NOT SUPPORTED.")); }
    unsafe { app_ui.change_packfile_type_index_includes_timestamp.as_mut().unwrap().set_status_tip(&QString::from_std_str("If checked, the PackedFile Index of this PackFile includes the 'Last Modified' date of every PackedFile. Note that PackFiles with this enabled WILL NOT SHOW UP as mods in the official launcher.")); }
    unsafe { app_ui.change_packfile_type_index_is_encrypted.as_mut().unwrap().set_status_tip(&QString::from_std_str("If checked, the PackedFile Index of this PackFile is encrypted. Saving this kind of PackFiles is NOT SUPPORTED.")); }
    unsafe { app_ui.change_packfile_type_header_is_extended.as_mut().unwrap().set_status_tip(&QString::from_std_str("If checked, the header of this PackFile is extended by 20 bytes. Only seen in Arena PackFiles with encryption. Saving this kind of PackFiles is NOT SUPPORTED.")); }
    
    unsafe { app_ui.change_packfile_type_data_is_compressed.as_mut().unwrap().set_status_tip(&QString::from_std_str("If checked, the data of each PackedFile in the open PackFile will be compressed on save. If you want to decompress a PackFile, disable this, then save it.")); }

    //-----------------------------------------------//
    // `Game Selected` menu tips.
    //-----------------------------------------------//
    unsafe { app_ui.game_selected_open_game_data_folder.as_mut().unwrap().set_status_tip(&QString::from_std_str("Tries to open the currently selected game's Data folder (if exists) in the default file manager.")); }
    unsafe { app_ui.game_selected_open_game_assembly_kit_folder.as_mut().unwrap().set_status_tip(&QString::from_std_str("Tries to open the currently selected game's Assembly Kit folder (if exists) in the default file manager.")); }
    
    unsafe { app_ui.game_selected_three_kingdoms.as_mut().unwrap().set_status_tip(&QString::from_std_str("Sets 'TW:Three Kingdoms' as 'Game Selected'.")); }
    unsafe { app_ui.game_selected_warhammer_2.as_mut().unwrap().set_status_tip(&QString::from_std_str("Sets 'TW:Warhammer 2' as 'Game Selected'.")); }
    unsafe { app_ui.game_selected_warhammer.as_mut().unwrap().set_status_tip(&QString::from_std_str("Sets 'TW:Warhammer' as 'Game Selected'.")); }
    unsafe { app_ui.game_selected_thrones_of_britannia.as_mut().unwrap().set_status_tip(&QString::from_std_str("Sets 'TW: Thrones of Britannia' as 'Game Selected'.")); }
    unsafe { app_ui.game_selected_attila.as_mut().unwrap().set_status_tip(&QString::from_std_str("Sets 'TW:Attila' as 'Game Selected'.")); }
    unsafe { app_ui.game_selected_rome_2.as_mut().unwrap().set_status_tip(&QString::from_std_str("Sets 'TW:Rome 2' as 'Game Selected'.")); }
    unsafe { app_ui.game_selected_shogun_2.as_mut().unwrap().set_status_tip(&QString::from_std_str("Sets 'TW:Shogun 2' as 'Game Selected'.")); }
    unsafe { app_ui.game_selected_napoleon.as_mut().unwrap().set_status_tip(&QString::from_std_str("Sets 'TW:Napoleon' as 'Game Selected'.")); }
    unsafe { app_ui.game_selected_empire.as_mut().unwrap().set_status_tip(&QString::from_std_str("Sets 'TW:Empire' as 'Game Selected'.")); }
    unsafe { app_ui.game_selected_arena.as_mut().unwrap().set_status_tip(&QString::from_std_str("Sets 'TW:Arena' as 'Game Selected'.")); }

    //-----------------------------------------------//
    // `Special Stuff` menu tips.
    //-----------------------------------------------//
    let generate_pak_file = QString::from_std_str("Generates a PAK File (Processed Assembly Kit File) for the game selected, to help with dependency checking.");
    let optimize_packfile = QString::from_std_str("Check and remove any data in DB Tables and Locs (Locs only for english users) that is unchanged from the base game. That means your mod will only contain the stuff you change, avoiding incompatibilities with other mods.");
    let patch_siege_ai_tip = QString::from_std_str("Patch & Clean an exported map's PackFile. It fixes the Siege AI (if it has it) and remove useless xml files that bloat the PackFile, reducing his size.");
    unsafe { app_ui.special_stuff_three_k_generate_pak_file.as_mut().unwrap().set_status_tip(&generate_pak_file); }
    unsafe { app_ui.special_stuff_three_k_optimize_packfile.as_mut().unwrap().set_status_tip(&optimize_packfile); }
    unsafe { app_ui.special_stuff_wh2_generate_pak_file.as_mut().unwrap().set_status_tip(&generate_pak_file); }
    unsafe { app_ui.special_stuff_wh2_optimize_packfile.as_mut().unwrap().set_status_tip(&optimize_packfile); }
    unsafe { app_ui.special_stuff_wh2_patch_siege_ai.as_mut().unwrap().set_status_tip(&patch_siege_ai_tip); }
    unsafe { app_ui.special_stuff_wh_generate_pak_file.as_mut().unwrap().set_status_tip(&generate_pak_file); }
    unsafe { app_ui.special_stuff_wh_optimize_packfile.as_mut().unwrap().set_status_tip(&optimize_packfile); }
    unsafe { app_ui.special_stuff_wh_patch_siege_ai.as_mut().unwrap().set_status_tip(&patch_siege_ai_tip); }
    unsafe { app_ui.special_stuff_tob_generate_pak_file.as_mut().unwrap().set_status_tip(&generate_pak_file); }
    unsafe { app_ui.special_stuff_tob_optimize_packfile.as_mut().unwrap().set_status_tip(&optimize_packfile); }
    unsafe { app_ui.special_stuff_att_generate_pak_file.as_mut().unwrap().set_status_tip(&generate_pak_file); }
    unsafe { app_ui.special_stuff_att_optimize_packfile.as_mut().unwrap().set_status_tip(&optimize_packfile); }
    unsafe { app_ui.special_stuff_rom2_generate_pak_file.as_mut().unwrap().set_status_tip(&generate_pak_file); }
    unsafe { app_ui.special_stuff_rom2_optimize_packfile.as_mut().unwrap().set_status_tip(&optimize_packfile); }
    unsafe { app_ui.special_stuff_sho2_generate_pak_file.as_mut().unwrap().set_status_tip(&generate_pak_file); }
    unsafe { app_ui.special_stuff_sho2_optimize_packfile.as_mut().unwrap().set_status_tip(&optimize_packfile); }
    unsafe { app_ui.special_stuff_nap_optimize_packfile.as_mut().unwrap().set_status_tip(&optimize_packfile); }
    unsafe { app_ui.special_stuff_emp_optimize_packfile.as_mut().unwrap().set_status_tip(&optimize_packfile); }

    //-----------------------------------------------//
    // `About` menu tips.
    //-----------------------------------------------//
    unsafe { app_ui.about_about_qt.as_mut().unwrap().set_status_tip(&QString::from_std_str("Info about Qt, the UI Toolkit used to make this program.")); }
    unsafe { app_ui.about_about_rpfm.as_mut().unwrap().set_status_tip(&QString::from_std_str("Info about RPFM.")); }
    unsafe { app_ui.about_open_manual.as_mut().unwrap().set_status_tip(&QString::from_std_str("Open RPFM's Manual in a PDF Reader.")); }
    unsafe { app_ui.about_patreon_link.as_mut().unwrap().set_status_tip(&QString::from_std_str("Open RPFM's Patreon page. Even if you are not interested in becoming a Patron, check it out. I post info about the next updates and in-dev features from time to time.")); }
    unsafe { app_ui.about_check_updates.as_mut().unwrap().set_status_tip(&QString::from_std_str("Checks if there is any update available for RPFM.")); }
    unsafe { app_ui.about_check_schema_updates.as_mut().unwrap().set_status_tip(&QString::from_std_str("Checks if there is any update available for the schemas. This is what you have to use after a game's patch.")); }
}