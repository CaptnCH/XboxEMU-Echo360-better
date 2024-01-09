#pragma once

#include "xenonGPU.h"
#include "../host_core/launcherCommandline.h"

namespace xenon
{

	/// Graphics system wrapper (CPU side of rendering)
	class Graphics
	{
	public:
		Graphics(runtime::Symbols& symbols, const launcher::Commandline& commandline);
		~Graphics();

		// setup the ring buffer, this initializes the internal GPU processor
		void InitializeRingBuffer(const void* ptr, const uint32 numPages);

		// set internal interrupt callback
		void SetInterruptCallbackAddr(const uint32 addr, const uint32 userData);

		// command buffer stuff
		void EnableReadPointerWriteBack(const uint32 ptr, const uint32 blockSize);

		// request a trace dump of the whole GPU frame
		void RequestTraceDump();

	private:
		// mapped global stuff
		lib::XenonNativeData	m_nativeVdGlobalDevice;
		lib::XenonNativeData	m_nativeVdGlobalXamDevice;
		lib::XenonNativeData	m_nativeVdGpuClockInMHz;
		lib::XenonNativeData	m_nativeVdHSIOCalibrationLock;

		// GPU "chip" implementation
		CXenonGPU*	m_gpu;
		bool		m_gpuInitialized;

		// IO to/from GPU
		static void WriteGPUWord(const uint64_t ip, const uint64_t addr, const uint32_t size, const void* inPtr);
		static void ReadGPUWord(const uint64_t ip, const uint64_t addr, const uint32_t size, void* outPtr);
	};

} // xenon