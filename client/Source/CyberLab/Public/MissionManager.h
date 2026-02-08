#pragma once

#include "CoreMinimal.h"
#include "UObject/NoExportTypes.h"
#include "MissionManager.generated.h"

USTRUCT(BlueprintType)
struct FMissionData
{
	GENERATED_BODY()

	UPROPERTY(BlueprintReadOnly)
	FString ID;

	UPROPERTY(BlueprintReadOnly)
	FString Title;

	UPROPERTY(BlueprintReadOnly)
	FString Description;
};

UCLASS(Blueprintable)
class CYBERLAB_API UMissionManager : public UObject
{
	GENERATED_BODY()

public:
	UFUNCTION(BlueprintCallable, Category = "CyberLab|Missions")
	void StartMission(const FString& MissionID);

	UFUNCTION(BlueprintCallable, Category = "CyberLab|Missions")
	void FetchMissions();

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "CyberLab|Config")
	FString BackendIP = "127.0.0.1:3000";
};
