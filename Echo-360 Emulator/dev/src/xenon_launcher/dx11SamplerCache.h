#pragma once

#include <Windows.h>
#include <d3d11.h>

#include "xenonGPUConstants.h"

class XenonSamplerInfo;

//---------------------------------------------------------------------------

/// Cache for sampler objects
class CDX11SamplerCache
{
public:
	CDX11SamplerCache(ID3D11Device* device);
	~CDX11SamplerCache();

	// find sampler for given definition
	ID3D11SamplerState* GetSamplerState(const XenonSamplerInfo& info);

private:
	typedef std::map< uint32, ID3D11SamplerState* >		TSamplers;

	// cached samplers
	TSamplers		m_samplers;

	// device
	ID3D11Device*	m_device;

	// clear cache
	void Clear();

	// create sampler definition from description
	static ID3D11SamplerState* CreateSamplerState(ID3D11Device* dev, const XenonSamplerInfo& info);
};
