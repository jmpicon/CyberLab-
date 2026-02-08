#include "TerminalComponent.h"
#include "WebSocketsModule.h"

UTerminalComponent::UTerminalComponent()
{
	PrimaryComponentTick.bCanEverTick = false;
}

void UTerminalComponent::BeginPlay()
{
	Super::BeginPlay();
}

void UTerminalComponent::EndPlay(const EEndPlayReason::Type EndPlayReason)
{
	if (Socket.IsValid() && Socket->IsConnected())
	{
		Socket->Close();
	}
	Super::EndPlay(EndPlayReason);
}

void UTerminalComponent::ConnectToBackend(const FString& ServerURL)
{
	if (!FWebSocketsModule::Get().IsModuleLoaded())
	{
		FWebSocketsModule::Get().StartupModule();
	}

	Socket = FWebSocketsModule::Get().CreateWebSocket(ServerURL);

	Socket->OnConnected().AddUObject(this, &UTerminalComponent::OnConnected);
	Socket->OnConnectionError().AddUObject(this, &UTerminalComponent::OnConnectionError);
	Socket->OnClosed().AddUObject(this, &UTerminalComponent::OnClosed);
	Socket->OnMessage().AddUObject(this, &UTerminalComponent::OnMessageReceived);

	Socket->Connect();
}

void UTerminalComponent::SendCommand(const FString& Command)
{
	if (Socket.IsValid() && Socket->IsConnected())
	{
		Socket->Send(Command);
	}
}

void UTerminalComponent::OnConnected()
{
	UE_LOG(LogTemp, Warning, TEXT("Connected to Terminal Backend"));
}

void UTerminalComponent::OnConnectionError(const FString& Error)
{
	UE_LOG(LogTemp, Error, TEXT("Terminal Connection Error: %s"), *Error);
}

void UTerminalComponent::OnClosed(int32 StatusCode, const FString& Reason, bool bWasClean)
{
	UE_LOG(LogTemp, Warning, TEXT("Terminal Connection Closed: %s"), *Reason);
}

void UTerminalComponent::OnMessageReceived(const FString& Message)
{
	OnOutputReceived.Broadcast(Message);
}
