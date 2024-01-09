#pragma once

#include "launcherBase.h"

namespace runtime
{
	class CodeTable;
	class RegisterBank;
	class TraceWriter;

	/// executes the code
	class LAUNCHER_API CodeExecutor
	{
	public:
		CodeExecutor(RegisterBank* regs, const CodeTable* code, const uint64 ip);
		~CodeExecutor();

		/// get register bank
		inline RegisterBank& GetRegs() const { return *m_regs;  }

		/// get code being executed
		inline const CodeTable& GetCode() const { return *m_code;  }

		/// get current instruction pointer
		inline const uint64 GetInstructionPointer() const { return m_ip; }

		/// run code loop tracing after every instruction
		bool RunTraced(TraceWriter& trace);

		/// run pure code loop
		bool RunPure();

	private:
		uint64				m_ip;
		const CodeTable*	m_code;
		RegisterBank*		m_regs;
	};

} // runtime