using UnrealBuildTool;

public class CyberLab : ModuleRules
{
	public CyberLab(ReadOnlyTargetRules Target) : base(Target)
	{
		PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;
	
		PublicDependencyModuleNames.AddRange(new string[] { 
			"Core", 
			"CoreUObject", 
			"Engine", 
			"InputCore", 
			"WebSockets", 
			"HTTP", 
			"Json", 
			"JsonUtilities",
			"OnlineSubsystem",
			"OnlineSubsystemSteam"
		});

		PrivateDependencyModuleNames.AddRange(new string[] {  });
	}
}
