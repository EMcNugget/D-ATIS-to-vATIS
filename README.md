# Takes real-world D-ATIS's and creates a custom vATIS preset.

## Features

- Works with any facility that has a D-ATIS.
- Can fetch the D-ATIS for all airports in a profile.
- Adds appropiate contractions so the voice atis reads it correctly.
- Listens for updates to the D-ATIS at an interval of your choosing, notifying you when it changes.
  - The interval can be set to a minimum of 15 minutes and a maximum of 60 minutes. By default, from the H+53 -> H+03 it will check it every 2 minutes as this is when the ATIS is most likely to change. This can be disabled.

## Installation

1. Download the latest [version](https://github.com/EMcNugget/D-ATIS-to-vATIS/releases).
2. Follow the instructions laid out in the installer.
3. **Windows Defender** may flag the program as a virus, this is a false positive. You can either disable Windows Defender or add an exception for the program. However this will not affect the program's functionality.

## Setup

1. Go to settings by clicking the cog icon on the bottem left.
2. You can specify a custom path to the vATIS folder, if you do not specify a path it will use the default path.
3. Specify which profile you'd like the program to use, this is **case** and **space** sensitive. If you do not specify a profile it will pick the first one it finds that has the facility that you specify.
4. Choose if you would like to save your facility, does not affect vATIS but will auto-fill the facility next time you open the program.
5. Click `Save` and then close the settings page.

## Usage

1. Enter a facility.
2. Ensure that vATIS is closed.
3. Click `Fetch`
4. Then in vATIS select the profile under the name `REAL WORLD`.

## Creating new contractions

To create a new contraction you have to edit the custom_contractions.json file.

- ### Notams:

  - 1. Go to your install folder.
  - 2. go to the `assets` folder.
  - 3. Open `custom_contractions.json`
  - 4. To add a contraction add the abbreviated version as the key and the way it should be said as the value.
  - 5. Ensure that you added it to the `notam_contractions` section.
  - 6. Save and relaunch the program.

- ### Airport name:
  - Same as above but edit the `airports` section.

## [Roadmap](https://github.com/users/EMcNugget/projects/12)
