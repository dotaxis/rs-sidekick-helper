# Sidekick-Helper
This is a simple project whose goal is to make [Sidekick](https://github.com/Sidekick-Poe/Sidekick) for Path of Exile 2 work in Hyprland.

## Usage
Alt+D will price check an item that is hovered. Alt+Space closes the Sidekick window.
To avoid the terminal window, run the binary via a .desktop file or with `& disown` and you can exit the app with the tray icon.

## Prerequisites
```
xdotool
dotnet-sdk 
aspnet-runtime
npm
nodejs
```

## Setup
1. Sidekick's source must be pulled from their repo.
```
git pull https://github.com/Sidekick-Poe/Sidekick
```
2. Edit the .sln file and comment (#) or remove the two lines
```
#Project("{9A19103F-16F7-4668-BE54-9A1E7A4F7556}") = "Sidekick.Wpf", "src\Sidekick.Wpf\Sidekick.Wpf.csproj", "{0E8A5165-AFEE-42BB-9C96-EA288F613BDE}"
#EndProject
```
3. Make sure your terminal is open in the root directory of the solution i.e `/home/dot/projects/Sidekick/`
4. Run `dotnet build`
5. Clone this repo:
```
git pull https://github.com/dotaxis/rs-sidekick-helper
```
6. Build with tauri:
```
cd rs-sidekick-helper/src-tauri
cargo tauri build --no-bundle
```
7. The binary can then be moved from the `target/release` folder and placed in the root of Sidekick's source, or you can pass the path to that directory using `SIDEKICK_DIR` and run it from anywhere. For example:
```
SIDEKICK_DIR=/home/dot/projects/Sidekick/ ./sidekick-helper
```

### Hyprland Window Rules
In order for this to mimic the overlay that Sidekick has on Windows, we need to set some window rules in Hyprland.
Place these in your hyprland.conf:
```
windowrulev2 = float,initialClass:^(sidekick-helper)$
windowrulev2 = minsize 800 600,initialClass:^(sidekick-helper)$
windowrulev2 = noinitialfocus,initialClass:^(sidekick-helper)$
windowrulev2 = move onscreen cursor 10%,class:^(sidekick-helper)$
bind = ALT, D, exec, xdotool keydown alt key d keyup alt
bind = ALT, Space, exec, xdotool keydown alt key space keyup alt
```
