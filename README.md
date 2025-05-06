[//]: # (# AcornGM - A GameMaker Mod Manager)


## How to actually use features (for now)
Since the modding system is not done, this frontend application is kind of useless now.
To actually test (and please fix) the \[de\]serialization,
**go to [LibGM](https://github.com/BioTomateDE/LibGM)** which has a main function as of now.
I refactored the code for way too long
(It's still not *that* good and readable tbh but it's my first rust project).

You don't have to do the steps below yet since the frontend app is unfinished.


## How to install and use (developer mode)
1. [Install Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) if you haven't already
2. In your terminal, navigate to where you want to install the program (**not directly in your Documents folder!**)
3. Clone this repository: `git clone https://github.com/BioTomateDE/AcornGM`
4. Go into the created folder: `cd AcornGM`
5. Build the program; you can either build in
    - **release**: `cargo build --release`; this might take a while to compile but will be faster while running.
    - or **dev**: `cargo build`; this will take a lot less time compiling but the program might run a bit slower.
6. (optional) Move the built program to your desired location:
    - if you built in release, it will be located in `./target/release/AcornGM.exe`
    - if you built in dev, it will be located in `./target/debug/AcornGM.exe`
7. Run the built program executable
8. Click the **Create Profile** button in the bottom right to create a profile for your desired game
9. Name your profile (typically the name of the game and maybe the version)
10. (optional) Choose an icon for your profile by clicking the image and selecting a new image file
11. Click **Next**
12. Click **Pick File** next to the data file text input and select 
the `data.win` (`game.unx` on Linux) file for your game. It will try to open
to the default Undertale installation folder for convenience, but choose the one for your game.
This is how you can find where your Steam games are located: 
    - Go to your Steam Library
    - Click on your desired game that you want to mod using AcornGM
    - Click the **Gear icon** on the right
    - Click **Properties**
    - In the Popup, click on the **Installed Files** tab
    - Click on **Browse...**
    - The file explorer will open up where the game is located.
    Remember that path (or copy it) and navigate there in the AcornGM data file picker.
13. (it might freeze for a couple seconds) It will try to detect the game and version 
automatically. If it fails to detect it and the game you've selected is either Undertale or Deltarune,
make sure it is vanilla and has not been modified by UndertaleModTool (in that case, 
"verify integrity of game files" in steam to reset the data.win)
14. Click **Next** again
15. Now you can browse, download and apply mods! (not really because i haven't implemented it yet)

## Todo List
### LibGM (Deserializing, Modding)
- **Fix variable reference not found while parsing variables in code** (important)
  - s
- Serialize CODE, VARI
- Implement Spline Sprites
- Replace some raw integer references with `GMRef`s
- Test if deserializing and serializing work properly by using the output data file with the GameMaker runner
- Probably fix lots of issues with the step above
- also test different games and gamemaker versions
- The entire modding system lol
- Variable and function occurrence list optimization (see comment)
- Maybe better texture pages generation? not important though

### AcornGM (Frontend)
- Button bar should align to bottom of window
- Lighter background color on hover for profile and mod list items
- fully implement browser and mod info view (when modding system in place)
- implement modding and connect to browser
- (dynamic window size / saving modified window size for users on smaller screen resolutions?)
- (welcome screen)

### AcornGMBackend (Backend)
- Testing with the new database
- Storing, uploading, editing, deleting, and downloading mods
- Being able to delete your account (GDPR)
- website looks ugly :c

