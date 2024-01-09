#pragma once

#include "../host_core/runtimePlatform.h"

//---------------------------------------------------------------------------

namespace xenon
{
	class Kernel;
	class FileSystem;
	class Graphics;
	class InputSystem;
	class Memory;
	class UserProfileManager;
	class Audio;
	class TraceFile;
	class TimeBase;

	/// Top level wrapper
	class Platform : public runtime::IPlatform
	{
	public:
		inline Kernel& GetKernel() { return *m_kernel; }

		inline FileSystem& GetFileSystem() { return *m_fileSys; }

		inline Graphics& GetGraphics() { return *m_graphics; }

		inline Memory& GetMemory() { return *m_memory;  }

		inline InputSystem& GetInputSystem() const { return *m_inputSys; }

		inline cpu::Interrupts& GetInterruptTable() const { return *m_interruptTable; }

		inline cpu::GeneralIO& GetIOTable() const { return *m_ioTable; }

		inline UserProfileManager& GetUserProfileManager() const { return *m_users; }

		inline Audio& GetAudio() const { return *m_audio; }

		inline TimeBase& GetTimeBase() const { return *m_timeBase; }

		inline runtime::TraceFile* GetTraceFile() const { return m_traceFile; }

	public:
		Platform();
		virtual ~Platform();

		// IPlatform
		virtual const char* GetName() const override final;
		virtual uint32 GetNumCPUs() const override final;
		virtual runtime::IDeviceCPU* GetCPU(const uint32 index) const override final;
		virtual uint32 GetNumDevices() const override final;
		virtual runtime::IDevice* GetDevice(const uint32 index) const override final;

		//---

		virtual bool Initialize(const native::Systems& nativeSystems, const launcher::Commandline& commandline, runtime::Symbols& symbols) override final;
		virtual void Shutdown() override final;

		//---

		virtual int RunImage(const runtime::Image& image) override final;

		//---

		void RequestUserExit();
		void DebugTrace(const char* txt);

		void RaiseIRQL();

	private:
		// subsystems
		Kernel*					m_kernel;		// kernel object container
		FileSystem*				m_fileSys;		// file system
		Graphics*				m_graphics;		// graphics subsystem
		InputSystem*			m_inputSys;		// input system
		Memory*					m_memory;		// memory system
		UserProfileManager*		m_users;		// user profiles
		Audio*					m_audio;		// audio system
		TimeBase*				m_timeBase;		// timer and stuff

		// some runtime data
		lib::XenonNativeData	m_nativeXexExecutableModuleHandle;
		lib::XenonNativeData	m_nativeKeDebugMonitorData;
		lib::XenonNativeData	m_nativeKeCertMonitorData;
		lib::XenonNativeData	m_nativeXboxHardwareInfo;
		lib::XenonNativeData	m_nativeXexExecutableModuleHandlePtr;
		lib::XenonNativeData	m_nativeExLoadedCommandLine;
		lib::XenonNativeData	m_nativeXboxKrnlVersion;
		lib::XenonNativeData	m_nativeKeTimeStampBundle;
		lib::XenonNativeData	m_nativeExThreadObjectType;

		// interrupt table
		cpu::Interrupts*	m_interruptTable;
		cpu::GeneralIO*		m_ioTable;

		// trace file (may be null)
		runtime::TraceFile*		m_traceFile;

		// irq levels

		// output log file
		std::ofstream m_platformLogFile;
		bool m_platformLogFileEnabled;

		// external exit
		bool m_userExitRequested;
	};

	//---------------------------------------------------------------------------

	// bind trace for for memory tagging
	// NOTE: writer is per-thead 
	extern runtime::TraceWriter* BindMemoryTraceWriter(runtime::TraceWriter* writer);

	// create IRQ/APC memory writer
	extern void CreateSystemTraceMemoryWriter();

	// tag a memory write, places information about the write in the trace file for current thread
	extern void TagMemoryWrite(const uint64 addr, const uint32 size, const char* txt, ...);
	extern void TagMemoryWrite(const void* ptr, const uint32 size, const char* txt, ...);

	//---------------------------------------------------------------------------


} // xenon

//---------------------------------------------------------------------------

extern xenon::Platform GPlatform;

//---------------------------------------------------------------------------

#define TAG_MEMORY_WRITE_PTR(data, size) xenon::TagMemoryWrite((uint64)data, size, __FUNCTION__);
#define TAG_MEMORY_WRITE_ADDR(addr, size) xenon::TagMemoryWrite(addr, size, __FUNCTION__);