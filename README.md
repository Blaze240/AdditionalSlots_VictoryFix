# Additional Slots Victory Fix (Smashline 2)

Skyline plugin modifying certain characters' audio on victory screens when utilizing added slots to play as intended, as without this plugin they will remain mute.

The following characters are fixed in this plugin:
* Ike
* Pokemon Trainer
* Wii Fit Trainer
* Robin
* Corrin
* Byleth

## Technical Details
The issue stems from the win scripts of all the affected characters performing hardcoded checks for the base slots exclusively in order to have both voice lines and sound effects play at the proper times, unlike the rest of the characters who merely have the scripts written to play their audio with no checks at all. Therefore, any added slots for those affected characters will not have any audio playing as a result, as there's simply no lines of code to account for the slots.

This plugin essentially rewrites the scripts to only include a slot number check to apply the proper audio timings for certain characters; this also includes multiple voice line variations (examples include Robin and Wii Fit Trainer). This will enable added slots to play audio while base slots are relatively unchanged.

Keep in mind that this plugin specifically hardcodes the same costume pattern as the base game (alternating odd/even for gender/costume variations). As a result, any modification to the costume order (for example stacking multiple female at once and vice versa) for either base or added slots will result in audio being mistimed and out of synch with the victory animation playing at that time. There are means to correct this in the code, which I will detail in the Wiki page.


## Prerequesites
* [Skyline](https://github.com/skyline-dev/skyline/releases)
* [NRO_Hook](https://github.com/ultimate-research/nro-hook-plugin/releases)
* [Smashline 2](https://github.com/HDR-Development/smashline/releases)

## Installation
Place the "libadditional_slots_victoryfix" plugin in the Skyline plugins location ```sd:/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/```.
