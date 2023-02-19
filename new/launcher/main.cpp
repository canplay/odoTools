#include "httplib.h"

#include <windows.h>
#include <shellapi.h>
#include <shlobj.h>
#include <filesystem>
#include <iostream>

#include <fmt\format.h>
#include <fmt\chrono.h>
#include <spdlog\spdlog.h>
#include <spdlog\async_logger.h>
#include <spdlog\async.h>
#include <spdlog\sinks\stdout_color_sinks.h>
#include <spdlog\sinks\rotating_file_sink.h>

int CreateLinkFile(LPCWSTR szStartAppPath, LPCWSTR szAddCmdLine, LPCOLESTR szDestLnkPath, LPCWSTR szIconPath)
{
	int exit = 0;

	HRESULT hr = CoInitialize(NULL);
	if (SUCCEEDED(hr))
	{
		IShellLink* pShellLink;
		hr = CoCreateInstance(CLSID_ShellLink, NULL, CLSCTX_INPROC_SERVER, IID_IShellLink, (void**)&pShellLink);
		if (SUCCEEDED(hr))
		{
			pShellLink->SetPath(szStartAppPath);
			std::wstring strTmp = szStartAppPath;
			int nStart = strTmp.find_last_of(L"/\\");
			pShellLink->SetWorkingDirectory(strTmp.substr(0, nStart).c_str());
			pShellLink->SetArguments(szAddCmdLine);

			if (szIconPath) pShellLink->SetIconLocation(szIconPath, 0);

			IPersistFile* pPersistFile;
			hr = pShellLink->QueryInterface(IID_IPersistFile, (void**)&pPersistFile);
			if (SUCCEEDED(hr))
			{
				hr = pPersistFile->Save((szDestLnkPath), FALSE);
				if (SUCCEEDED(hr)) exit = 1;
				pPersistFile->Release();
			}
			pShellLink->Release();
		}
		CoUninitialize();
	}
	return exit;
}

int RunGame(std::string username, std::string password)
{
	std::string path;
	char sFile[MAX_PATH];
	GetModuleFileNameA(NULL, sFile, MAX_PATH);
	path = sFile;
	size_t nStart1 = path.find_last_of('\\');
	path = path.substr(0, nStart1);

#ifdef _DEBUG
	spdlog::info("{}\\bin64\\BlackDesert64.exe {},{} {}\\bin64", path, username, password, path);
#endif

	ShellExecuteA(0, "open", fmt::format("{}\\bin64\\BlackDesert64.exe", path.c_str()).c_str(), fmt::format("{},{}", username.c_str(), password.c_str()).c_str(), fmt::format("{}\\bin64", path.c_str()).c_str(), SW_SHOW);

	return 0;
}

int RegisterProtocol()
{
	int exit = 0;

	char szReg[MAX_PATH] = "BDOLauncher";
	HKEY hKey = 0;
	if (RegCreateKeyA(HKEY_CLASSES_ROOT, szReg, &hKey) == ERROR_SUCCESS)
	{
		std::string str = fmt::format("\"{}\\launcher.exe\"", std::filesystem::current_path().string());
		if (RegSetKeyValueA(hKey, 0, "URL Protocol", REG_SZ, str.c_str(), str.length()) == ERROR_SUCCESS)
		{
			str = "BOO Launcher Protocol";
			if (RegSetValueA(hKey, 0, REG_SZ, str.c_str(), str.length()) == ERROR_SUCCESS)
			{
				RegCloseKey(hKey);

				if (RegCreateKeyA(HKEY_CLASSES_ROOT, fmt::format("{}\\DefaultIcon", szReg).c_str(), &hKey) == ERROR_SUCCESS)
				{
					if (RegSetValueA(hKey, 0, REG_SZ, fmt::format("\"{}\\launcher.exe\",0", std::filesystem::current_path().string()).c_str(), str.length()) == ERROR_SUCCESS)
					{
						RegCloseKey(hKey);

						if (RegCreateKeyA(HKEY_CLASSES_ROOT, fmt::format("{}\\shell\\open\\command", szReg).c_str(), &hKey) == ERROR_SUCCESS)
						{
							str = fmt::format("\"{}\\launcher.exe\" \"%1\"", std::filesystem::current_path().string());
							if (RegSetValueA(hKey, 0, REG_SZ, str.c_str(), str.length()) == ERROR_SUCCESS)
							{
								exit = 1;
							}
						}
					}
				}
			}
		}
	}

	RegCloseKey(hKey);

	return exit;
}

int checkVersion(std::string remote, std::string local)
{
	int exit = 0;

	return exit;
}

int main(int argc, LPCSTR argv[])
{
	HANDLE hStdin = GetStdHandle(STD_INPUT_HANDLE);
	DWORD mode;
	GetConsoleMode(hStdin, &mode);
	mode &= ~ENABLE_QUICK_EDIT_MODE;
	mode &= ~ENABLE_INSERT_MODE;
	mode &= ~ENABLE_MOUSE_INPUT;
	SetConsoleMode(hStdin, mode);

	if (!std::filesystem::exists("./log"))
		std::filesystem::create_directory("./log");

	spdlog::init_thread_pool(8192, 1);
	spdlog::flush_on(spdlog::level::warn);
	spdlog::enable_backtrace(32);
	auto out_sink = std::make_shared<spdlog::sinks::stdout_color_sink_mt>();
	auto file_sink = std::make_shared<spdlog::sinks::rotating_file_sink_mt>("./log/log.log", 1048576 * 10, 5);
	out_sink->set_level(spdlog::level::trace);
	file_sink->set_level(spdlog::level::trace);
	std::vector<spdlog::sink_ptr> sinks{ out_sink, file_sink };
	auto logger = std::make_shared<spdlog::async_logger>("", sinks.begin(), sinks.end(), spdlog::thread_pool(), spdlog::async_overflow_policy::block);
	spdlog::set_default_logger(logger);
	spdlog::set_error_handler([](const std::string& msg)
	{
		spdlog::error("{}", msg);
	});

	spdlog::info("========== start launcher ==========");

	if (argc < 2)
	{
		RegisterProtocol();
		ShellExecuteA(NULL, "open", "explorer.exe", "http://1.13.22.82:51530", NULL, SW_SHOW);
		return 0;
	}

	std::string str = argv[1];

	if (strcmp(argv[1], "RunGame") == 0)
		return RunGame(argv[2], argv[3]);
	else if (strcmp(argv[1], "RegisterProtocol") == 0)
		return RegisterProtocol();
	else if (str.find("bdolauncher://") != std::string::npos)
	{
		std::string username, password;

		// bdolauncher://Username,Password

		auto nStart = str.find('/') + 2;
		auto nEnd = str.find(',');

		username = str.substr(nStart, nEnd - nStart);
		nStart = nEnd + 1;
		password = str.substr(nStart, str.length() - nStart - 1);

		RunGame(username, password);
	}
	else
	{
		for (int i = 0; i < argc; i++)
		{
			spdlog::info("arg[{}]: {}", i, argv[i]);
		}
	}

#ifdef _DEBUG
	getchar();
#endif

	spdlog::shutdown();
	spdlog::info("========== stop launcher ==========");

	return 0;
}