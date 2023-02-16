#pragma once

namespace api
{
	class MsSql
	{
	public:
		bool init(std::string datasource, std::string username, std::string password);

	private:
		nanodbc::connection odbc;
	};
}
