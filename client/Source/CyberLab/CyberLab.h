#pragma once

#include "CoreMinimal.h"

class FCyberLabModule : public IModuleInterface
{
public:
	virtual void StartupModule() override;
	virtual void ShutdownModule() override;
};
