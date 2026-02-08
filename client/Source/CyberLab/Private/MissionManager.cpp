#include "MissionManager.h"
#include "HttpModule.h"
#include "Interfaces/IHttpResponse.h"

void UMissionManager::StartMission(const FString& MissionID)
{
	FHttpModule* Http = &FHttpModule::Get();
	TSharedRef<IHttpRequest, ESPMode::ThreadSafe> Request = Http->CreateRequest();
	
	Request->SetURL(FString::Printf(TEXT("http://%s/api/mission/start"), *BackendIP));
	Request->SetVerb("POST");
	Request->SetContentAsString(FString::Printf(TEXT("{\"mission_id\": \"%s\"}"), *MissionID));
	Request->SetHeader("Content-Type", "application/json");

	Request->OnProcessRequestComplete().BindLambda([](FHttpRequestPtr Request, FHttpResponsePtr Response, bool bWasSuccessful)
	{
		if (bWasSuccessful && Response.IsValid())
		{
			UE_LOG(LogTemp, Log, TEXT("Mission Started: %s"), *Response->GetContentAsString());
		}
	});

	Request->ProcessRequest();
}

void UMissionManager::FetchMissions()
{
    // Similar to StartMission but for GET /api/missions
}
