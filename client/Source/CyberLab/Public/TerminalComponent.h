#pragma once

#include "CoreMinimal.h"
#include "Components/ActorComponent.h"
#include "IWebSocket.h"
#include "TerminalComponent.generated.h"

DECLARE_DYNAMIC_MULTICAST_DELEGATE_OneParam(FOnTerminalOutputReceived, const FString&, Output);

UCLASS( ClassGroup=(Custom), meta=(BlueprintSpawnableComponent) )
class CYBERLAB_API UTerminalComponent : public UActorComponent
{
	GENERATED_BODY()

public:	
	UTerminalComponent();

protected:
	virtual void BeginPlay() override;
	virtual void EndPlay(const EEndPlayReason::Type EndPlayReason) override;

public:	
	UFUNCTION(BlueprintCallable, Category = "CyberLab|Terminal")
	void ConnectToBackend(const FString& ServerURL);

	UFUNCTION(BlueprintCallable, Category = "CyberLab|Terminal")
	void SendCommand(const FString& Command);

	UPROPERTY(BlueprintAssignable, Category = "CyberLab|Terminal")
	FOnTerminalOutputReceived OnOutputReceived;

private:
	TSharedPtr<IWebSocket> Socket;
	void OnConnected();
	void OnConnectionError(const FString& Error);
	void OnClosed(int32 StatusCode, const FString& Reason, bool bWasClean);
	void OnMessageReceived(const FString& Message);
};
