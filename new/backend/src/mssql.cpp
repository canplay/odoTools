#include "stdafx.h"
#include "MsSql.h"

namespace api
{
	static nanodbc::connection odbc;

	bool MsSql::connect(std::string datasource, std::string username, std::string password)
	{
		try
		{
			odbc = nanodbc::connection(datasource, username, password, 15);
			return odbc.connected();
		}
		catch (const std::exception& e)
		{
			spdlog::error(e.what());
			return false;
		}
	}

	void MsSql::close()
	{
		try
		{
			odbc.disconnect();
		}
		catch (const std::exception& e)
		{
			spdlog::error(e.what());
		}
	}

	nanodbc::result MsSql::exec(std::string stmt)
	{
		try
		{
			return nanodbc::execute(odbc, stmt, 1, 15);
		}
		catch (const std::exception& e)
		{
			nanodbc::result r;
			spdlog::error(e.what());
			return r;
		};
	}
}