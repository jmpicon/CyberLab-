# SteamWorks SDK Integration Guide for CyberLab

To officially launch on Steam, CyberLab needs to integrate the SteamWorks SDK. This allows for features like Achievements, Steam Cloud, and DRM.

## 1. Backend (Rust) - Optional but Recommended
Use the `steamworks` crate to initialize the API.

```rust
// Add to Cargo.toml: steamworks = "0.11"
use steamworks::Client;

fn init_steam() {
    let (client, single) = Client::init_app(YOUR_APP_ID).expect("Steam must be running");
    // client is now your handle to Steam features
}
```

## 2. Client (Unreal Engine 5) - Mandatory
Unreal Engine has built-in support for Steam.

### Configuration (`DefaultEngine.ini`)
Add the following to your `Config/DefaultEngine.ini`:
```ini
[/Script/Engine.GameEngine]
+NetDriverDefinitions=(DefName="GameNetDriver",DriverClassName="OnlineSubsystemSteam.SteamNetDriver",DriverClassNameFallback="OnlineSubsystemUtils.IpNetDriver")

[OnlineSubsystem]
DefaultPlatformService=Steam

[OnlineSubsystemSteam]
bEnabled=true
SteamAppId=YOUR_APP_ID_HERE
bVACEnabled=0
```

### Build Details (`CyberLab.Build.cs`)
Ensure the OnlineSubsystemSteam module is included:
```csharp
PublicDependencyModuleNames.AddRange(new string[] { "Core", "CoreUObject", "Engine", "InputCore", "OnlineSubsystem", "OnlineSubsystemSteam" });
```

## 3. SteamPipe Upload Process
We have provided `steam/app_build.vdf`. To upload:
1. Download the [Steam SDK ContentBuilder](https://partner.steamgames.com/downloads/steamworks_sdk.zip).
2. Place the builder in your tools folder.
3. Run the following command:
```bash
./builder/steamcmd.exe +login <your_user> +run_app_build ../steam/app_build.vdf +quit
```

## 4. Next Steps for Developer
1. Pay the Steam Direct fee to get your **AppID**.
2. Replace `YOUR_APP_ID_HERE` in all config files.
3. Upload the first build to the `branch: beta` for testing.
