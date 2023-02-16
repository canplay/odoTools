#include "stdafx.h"
#include "MsSql.h"

#include <nanodbc/nanodbc.h>

namespace api
{
	bool MsSql::init(std::string datasource, std::string username, std::string password)
	{
		auto r = nanodbc::connection(datasource, username, password, 15);
		odbc = r;
		return r.connected();
	}
}