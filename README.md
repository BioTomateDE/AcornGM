[//]: # (You are viewing this file in its raw form)
[//]: # (A formatted view is available at https://github.com/BioTomateDE/AcornGM)


## How to actually use features (for now)
Since the modding system is not done, this frontend application is kind of useless now.
To actually test (and please fix) the \[de\]serialization,
**go to [LibGM](https://github.com/BioTomateDE/LibGM)** which has a main function as of now.
You don't actually have to do the steps below yet since the frontend app is unfinished.


## How to install on Windows (not yet)
1. Go to the **Releases** section (on the top right of this page).
2. Download the latest .exe file.
3. Double-click the downloaded file to run the program.
> [!NOTE] 
> There might be a warning saying "**Windows protected your PC**". 
This happens because not many people have downloaded that exe yet.
Microsoft is being extra cautious so you don't accidentally run malware.
Since this project is open source, it would be practically impossible to hide malware.
(if you're really paranoid, you can even build from source.)

> [!TIP]
> You can bypass this warning by clicking the small underlined text saying "**More info**"
which will cause a "**Run anyway**" button to appear.


## How to install (developer mode)
1. Install [Rust Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) if you haven't already.
2. In your terminal, navigate to where you want to install the program (using the `cd` command).
3. Clone this repository: `git clone https://github.com/BioTomateDE/AcornGM`.
4. Go into the created folder: `cd AcornGM`.
5. Build the program: `cargo b -r` (equivalent to `cargo build --release`).
6. (optional) Move the built program to your desired location:
   - Unix: `mv ./target/release/AcornGM /your/desired/path`
   - Windows: `move target\release\AcornGM.exe C:\your\desired\path`
7. Run the built program file


## How to use: Creating a Profile
1. Click the **Create Profile** button in the bottom right to create a profile for your desired game.
2. Name your profile (typically the name of the game and maybe the version).
3. (optional) Choose an icon for your profile by clicking the image and selecting a new image file.
4. Click **Next**.
5. Click **Pick File** next to the data file text input and select
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
6. It will try to detect the game and version automatically.
If it fails to detect the version for Undertale or Deltarune,
make sure it is vanilla and has not been modified by UndertaleModTool or similar (in that case, 
"verify integrity of game files" in steam to reset the data.win)
7. Click **Next** again.
8. Now you can browse, download and apply mods! (not really because i haven't implemented it yet)



## Todo List
### LibGM (Deserializing, Modding)
- Implement Spine Sprites
- fix the fucking [de]serializer (hahahahahaha i love yoyogames)
- more gm version detection probably
- implement all other chunks
- test different games and gamemaker versions
- applying mods

### AcornGM (Frontend)
- Button bar should align to bottom of window
- Lighter background color on hover for profile and mod list items
- fully implement browser and mod info view (when modding system in place)
- implement modding and connect to browser
- (dynamic window size / saving modified window size for users on smaller screen resolutions?)
- welcome screen with eula
- fix icon not showing up??? (arch wayland plasma)

### AcornGMBackend (Backend)
- Testing
- Searching for mods
- Being able to delete your account and mods (GDPR)
- website looks ugly :c

## Licence
[View EULA and licence here](https://acorngm.biotomatede.hackclub.app/eula.html).

