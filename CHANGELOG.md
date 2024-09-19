# D-ATIS to vATIS

## [1.7.3](#1.7.3) - 9/18/2024

### Patch Changes

- 057a31c: Added custom preset names based on airport operations
- 0b2f179: Added additional error handling
- 00e0793: Updated NOTAM parser to accept a wider range of ATIS's
- 67d823c: Fixed log not rolling over for each instance

## [1.7.2](#1.7.2) - 9/8/2024

### Patch Changes

- Fixed tooltip
- Changed settings styling

## [1.7.1](#1.7.1) - 9/5/2024

### Patch Changes

- aebdb99: Added GUI for adding custom contractions
- a876405: Added ability to suppress new ATIS notification
- 0eafaa6: Fixed ATIS's with parsing errors not displaying a code in the alert message

## [1.7.0](#1.7.0) - 9/2/2024

### Minor Changes

- 1eba84e: Added ability to fetch ATIS for all airports in a profile

### Patch Changes

- 1eba84e: Fixed alerts having duplicate messages (resurgence)
- 1eba84e: Fixed profile dropdown duplicating entries when selecting a profile
- 1eba84e: Added added error handling for fetching
- 9c00d5e: Fixed settings validation CTD

## [1.6.1](#1.6.1) - 8/29/2024

### Patch Changes

- 44e85ef: Added contractions for atis introduction (e.g "ATL DEP INFO" -> "ATL DEPARTURE ")
- ad5569a: Added dropdown for profiles

## [1.6.0](#1.6.0) - 8/11/2024

### Minor Changes

- 0cdfb6b: Added option to listen for D-ATIS updates. Notifies user via alert and sound.

### Patch Changes

- 0cdfb6b: Added option to open vATIS on fetch.
- 5b0bc61: Various styling changes.
- 5b0bc61: Settings validation now correctly orders entries.
- 77a101c: Added versioning CI
- 0cdfb6b: Updated default window size to better fit added settings.
