# Takes real-world D-ATIS's and creates a custom vATIS preset.

## Installation

1. Download the latest [version](https://github.com/EMcNugget/D-ATIS-to-vATIS/releases).
2. Follow the instructions laid out in the installer.

## Setup

1. Go to settings by clicking the cog icon on the bottem left.
2. Specify where vATIS is installed; this path is usually `C:\Users\(user)\AppData\Local\vATIS-4.0`
3. Specify which profile you'd like the program to use, this is case and space sensitive. If you do not specify a profile it will pick the first one it finds that has the facility that you specify.
4. Choose if you would like to save your facility, does not affect vATIS but will auto-fill the facility next time you open the program.
5. Click `Save` and then close the settings page.

## Usage

1. Enter a facility.
2. Ensure that vATIS is closed.
3. Click `Fetch`
4. Then in vATIS select the profile under the name `REAL WORLD`.

## Creating new contractions:

To create a new contraction you have to edit the custom_contractions.json file.

1. Go your install folder/assets.
2. Open ```custom_contractions.json```
3. To add a contraction add the abbreviated versioni as the key and the way it should be said as the value.
4. Save and relaunch the program.
