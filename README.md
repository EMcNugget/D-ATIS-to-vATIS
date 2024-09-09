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

## Setup

1. Go to settings by clicking the cog icon on the bottem left.
2. You can specify a custom path to the vATIS folder, if you do not specify a path it will use the default path.
3. Specify which profile you'd like the program to use. If you do not specify a profile it will pick the first one it finds that has the facility that you specify.
4. Choose if you would like to save your facility, does not affect vATIS but will auto-fill the facility next time you open the program.
5. Click `Save` and then close the settings page.

## Usage

1. Enter a facility.
2. Ensure that vATIS is closed.
3. Click `Fetch`
4. Then in vATIS select the profile under the name `REAL WORLD`.

------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

![Cannot load logo](./src-tauri/icons/Square284x284Logo.png)


