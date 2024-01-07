#pragma once
#include "../host_core/nativeKernel.h"
#include <mutex>

namespace win
{
	class CriticalSection : public native::ICriticalSection
	{
	public:
		CriticalSection();
		virtual ~CriticalSection();

		virtual void Acquire() override final;
		virtual void Release() override final;

	private:
		CRITICAL_SECTION m_lock;
	};

	class Event : public native::IEvent
	{
	public:
		Event(bool initialState, bool manualReset);
		virtual ~Event();

		virtual bool Set() override final;
		virtual bool Pulse() override final;
		virtual bool Reset() override final;
		virtual native::WaitResult Wait(const uint32 timeout, const bool alertable) override final;
		virtual void* GetNativeHandle() const override final;

	private:
		HANDLE	m_hEvent;
	};

	class Semaphore : public native::ISemaphore
	{
	public:
		Semaphore(uint32 initalCount, uint32 maxCount);
		virtual ~Semaphore();

		virtual uint32 Release(const uint32 count) override final;
		virtual native::WaitResult Wait(const uint32 timeout, const bool alertable) override final;
		virtual void* GetNativeHandle() const override final;

	private:
		HANDLE	m_hSemaphore;
	};

	class Mutant : public native::IMutant
	{
	public:
		Mutant(bool initialOwner);
		virtual ~Mutant();

		virtual bool Release() override final;
		virtual void* GetNativeHandle() const override final;
		virtual native::WaitResult Wait(const uint32 timeout, const bool alertable) override final;

	private:
		HANDLE m_hMutant;
	};

	class Timer : public native::ITimer
	{
	public:
		Timer(bool manualReset);
		virtual ~Timer();

		virtual bool SetOnce(uint64_t nanosecondsToWait, const TCallbackFunc& func) override final;
		virtual bool SetRepeating(uint64_t nanosecondsToWait, uint64_t milisecondPeriod, const TCallbackFunc& func) override final;
		virtual bool Cancel() override final;
		virtual native::WaitResult Wait(const uint32 timeout, const bool alertable) override final;
		virtual void* GetNativeHandle() const override final;

	private:
		std::mutex m_callbackLock;
		std::function<void()> m_callback;

		static void APIENTRY WaitOrTimerCallback(PVOID param, DWORD dwTimerLowValue, DWORD dwTimerHighValue);

		HANDLE m_hTimer;
	};

	class Thread : public native::IThread
	{
	public:
		Thread(native::IRunnable* runnable);
		virtual ~Thread();

		virtual void* GetNativeHandle() const override final;
		virtual native::WaitResult Wait(const uint32 timeout, const bool alertable) override final;

		virtual bool Pause() override final;
		virtual bool Resume() override final;

		virtual int32 SetPriority(const native::ThreadPriority priority) override final;
		virtual uint32 SetAffinity(const uint32 affinity) override final;

		virtual native::WaitResult Sleep(const uint32 ms, const bool alertable) override final;

		virtual void ScheduleAPC() override final;

	private:
		HANDLE m_hThread;
		DWORD m_ThreadID;

		native::IRunnable*	m_runnable;

		static void DeliverAPCs(void* pData);
		static DWORD WINAPI ThreadProc(LPVOID lpParameter);
	};

	class Kernel : public native::IKernel
	{
	public:
		Kernel();
		virtual ~Kernel();

		virtual native::ICriticalSection* CreateCriticalSection() override final;
		virtual native::IEvent* CreateEvent(const bool manualReset, const bool initialState) override final;
		virtual native::ISemaphore* CreateSemaphore(const uint32 initialCount, const uint32 maxCount) override final;
		virtual native::IThread* CreateThread(native::IRunnable* runnable) override final;
		virtual native::ITimer* CreateManualResetTimer() override final;
		virtual native::ITimer* CreateSynchronizationTimer() override final;
		virtual native::IMutant* CreateMutant(bool initiallyOpened) override final;

		virtual native::WaitResult WaitMultiple(const std::vector<native::IKernelObject*>& objects, const bool waitAll, const uint32 timeOut, const bool alertable) override final;
		virtual native::WaitResult SignalAndWait(native::IKernelObject* nativeSignalObject, native::IKernelObject* nativeWaitObject, const uint32 timeOut, const bool alertable) override final;

	};

} // win