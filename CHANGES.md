
# Release 0.1.0:

### Added on 22.12.2023

- C: Fixed an issue, when user tried to copy a file to directory with a new name.
- P: Program was rewritten
- PERM: Fixed issues with verbose mode still being enabled while it shouldn't
- RUSH: Added an extremely early shell for CarrotOS

### Added on 23.12.2023

- C: Fixed directory to directory copying
- RUSH: Can understand some built-in commands and execute commands

# Release 0.1.1:

### Added on 28.12.2023

- JUNK: Ability to overwrite files with zeroes or random bytes
- JUNK: Ability to create files with zeroes or random bytes of desired size
- JUNK: Verbose mode

# Release 0.2.0:

### Added on 28.12.2023

- RUSH: Support for keyboard shortcuts
- RUSH: Separation of some code from RUSH to libinput

### Added on 29.12.2023

- L: Less spaghetti code
- C: Heavily rewritten to fix many bugs and ditch spaghetti code
- M: Same as for the previous program since "M" does practically the same job as "C"
- INFO, LF and JUNK changed their names

### Added on 30.12.2023

- RF, RD, LOATHE: Migrate ask() to libinput
- C, M: Ability to ask before actually overwritting files
- LIBDIR: No more unwrapped errors
- L: Ability to sort output alphabetically is back again!

### Added on 31.12.2023

- MD: Ability to show only selected information
- MD: Has an option to enable/disable link resolving
- AD: Ability to disable creating parent directories if they do not exist
- OWN: Use recently added chown added to Rust's STDLIB
- *: Added license to the source tree

### Added on 01.01.2024

- L: Colorful output
- L: When folder is being shown on the list, it's name will always end with "/"
- P: Removed "start", "end" and "rev" switches. There will be added a tool to change command output

# Release 0.2.1:

### Added on 06.01.2024

- C: Fixed a bug that caused moving files to directories impossible if file under destdir doesn't exist
- M: Same patch as in "C". I should probably connect those utilities into one thing...

### Added on 13.01.2024

- AD, AF: Added "ignore" switches. They allow user to ignore "AlreadyExists" error

# Release 0.2.2:

### Added on 14.01.2024

- S: Simple pager (NEW PROGRAM!)

### Added on 15.01.2024

- S: Changed it's name to 'page'

### Added on 18.01.2024

- P: Removed support for counting lines - Use "count -p" command to do it
- PAGE: Add support for piped content
- COUNT: Count number of words or lines from input (NEW PROGRAM!)
- TR: Transform command output (NEW PROGRAM!)

# Release 0.2.3:

### Added on 21.01.2024

 - *: Migrate libraries to another project called "carrot-libs"
 - C: Renamed to CP
 - M: Renamed to MV

# Release 0.2.4:

### Added on 24.03.2024

- RUSH: Migrated to another project

# Release 0.2.5:

### Added on 29.03.2024

- PAGE: Deal with char boundaries

# Release 0.2.6:

### Added on 08.04.2024

- COL: A tool to columnize output (NEW PROGRAM!)
- L: Switch "n" was renamed to "s"
- L: Ability to hide the name of folder that is being listed
- M: Tries to copy and remove file from source if the destination path is on another disk
- CASE: Convert every letter to uppercase/lowercase
- SHRINK: Make wide lines smaller (NEW PROGRAM!)

# Release 0.2.7:

### Added on 11.05.2024

- REX: Tool that executes commands as other users (NEW PROGRAM!) (Very incomplete support)

### Waiting features:

- PAGE: Move code responsible for shrinking wide lines to a function so the code would be more readable
- PERM: ACLs support

# Release 0.3.0:

### Added on 12.05.2024

- USERUTIL: Change system users (NEW PROGRAM!)

### Added on 13.05.2024

- USERUTIL: Supported user description field, ability to remove users

### Waiting features:

- USERUTIL: Ability to set and manage passwords, ability to update existing users
- DISKUTIL: Disk management (NEW PROGRAM!)
- REX: Add more functionality
- DIV: Find divergent lines in files (NEW PROGRAM!)

# Release 0.3.1:

- HELP: View documentation
- SCRIBUS: Generate documents from templates
