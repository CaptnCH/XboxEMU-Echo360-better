#pragma once

#include "projectAddressHistory.h"

namespace tools
{
	/// general project settings
	struct ProjectSettings
	{
		ProjectSettings();
			
		// custom directory for the file system root
		// if not specified than the project directory is used instead
		std::wstring m_customRoot;
	};

	/// breakpoint list (global)
	class ProjectBreakpointList
	{
	public:
		ProjectBreakpointList();

		/// clear breakpoint list
		void Clear();

		/// set breakpoint on address
		void SetBreakpoint(const uint64 addr, const bool isSet);

		/// check if address contains breakpoint
		const bool HasBreakpoint(const uint64 addr) const;

		/// toggle breakpoint, returns new state
		const bool ToggleBreakpoint(const uint64 addr);

		/// get all breakpoint
		typedef std::vector<uint64_t> TBreakpoints;
		inline const TBreakpoints& GetAllBreakpoints() const { return m_breakpoints; }

		//--

	private:
		TBreakpoints m_breakpoints;
	};

	/// Editable project
	class Project
	{
	public:
		~Project();

		//---

		// get the decoding platform
		inline const platform::Definition* GetPlatform() const { return m_platform; }

		// get project main file
		inline const wxString& GetProjectPath() const { return m_projectPath; }

		// get project directory
		inline const wxString& GetProjectDirectory() const { return m_projectDir; }

		// get loaded project images
		typedef std::vector<std::shared_ptr<ProjectImage>> TProjectImages;
		inline const TProjectImages& GetImages() const { return m_images; }

		// get project settings
		inline ProjectSettings& GetSettings() { return m_settings; }

		// get project breakpoint settings
		inline ProjectBreakpointList& GetBreakpoints() { return m_breakpoints; }
		inline const ProjectBreakpointList& GetBreakpoints() const { return m_breakpoints; }

		//---

		// save project file
		bool Save(ILogOutput& log);

		//! Is the project modified ?
		bool IsModified() const;

		// mark after changes as modified so we will prompt to save the changes
		void MarkAsModified();

		//---

		// add image to project
		void AddImage(const std::shared_ptr<ProjectImage>& image);

		// remove image from project
		void RemoveImage(const std::shared_ptr<ProjectImage>& image);

		// get all startup images
		void GetStartupImages(std::vector<std::shared_ptr<ProjectImage>>& outImages) const;

		//---

		// load existing project
		static std::shared_ptr<Project> LoadProject(ILogOutput& log, const std::wstring& projectPath);

		// create new project
		static std::shared_ptr<Project> CreateProject(ILogOutput& log, const std::wstring& projectPath, const platform::Definition* platform);

		//---

		// get decoding context for given instruction pointer
		decoding::Context* GetDecodingContext(const uint64 ip);

		// get project image for given instruction point
		std::shared_ptr<ProjectImage> FindImageForAddress(const uint64 ip);

	private:
		Project(const platform::Definition* platform, const wxString& projectFilePath);

		// platform
		const platform::Definition* m_platform;

		// path to the project
		wxString m_projectPath;
		wxString m_projectDir;
		
		// loaded project images
		TProjectImages m_images;

		// general project settings
		ProjectSettings m_settings;

		// global breakpoint settings
		ProjectBreakpointList m_breakpoints;

		// local modification flag
		bool m_isModified;

		// cached decoding context
		uint64 m_currentDecodingContextStart;
		uint64 m_currentDecodingContextEnd;
		decoding::Context* m_currentDecodingContext;
	};

} // tools