#include "stdafx.h"
#include "settings.h"
#include "mssql.h"

namespace api
{
	void Settings::url(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const
	{
		auto stmt = fmt::format("SELECT * FROM [SA_BETA_WORLDDB_0002].[PaWebPublic].[settings] WHERE [name] = 'url'");

		Json::Value ret;

		try
		{
			Json::Value info;

			auto r = MsSql::exec(stmt);
			r.next();

			info["id"] = r.get<std::string>("id");
			info["name"] = r.get<std::string>("name");
			info["value"] = r.get<std::string>("value");
			info["create_date"] = r.get<std::string>("create_date");
			info["create_user"] = r.get<std::string>("create_user");
			info["create_id"] = r.get<std::string>("create_id");
			info["update_user"] = r.get<std::string>("update_user");
			info["update_date"] = r.get<std::string>("update_date");
			info["update_id"] = r.get<std::string>("update_id");

			ret["msg"] = info;
			ret["status"] = 1;
		}
		catch (const std::exception& e)
		{
			spdlog::warn("settings url error: {}", e.what());
			ret["msg"] = e.what();
			ret["status"] = 0;
		}

		callback(HttpResponse::newHttpJsonResponse(ret));
	}

	void Settings::version(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const
	{
		auto stmt = fmt::format("SELECT * FROM [SA_BETA_WORLDDB_0002].[PaWebPublic].[settings] WHERE [name] = 'version'");

		Json::Value ret;

		try
		{
			Json::Value info;

			auto r = MsSql::exec(stmt);
			r.next();

			info["id"] = r.get<std::string>("id");
			info["name"] = r.get<std::string>("name");
			info["value"] = r.get<std::string>("value");
			info["create_date"] = r.get<std::string>("create_date");
			info["create_user"] = r.get<std::string>("create_user");
			info["create_id"] = r.get<std::string>("create_id");
			info["update_user"] = r.get<std::string>("update_user");
			info["update_date"] = r.get<std::string>("update_date");
			info["update_id"] = r.get<std::string>("update_id");

			ret["msg"] = info;
			ret["status"] = 1;
		}
		catch (const std::exception& e)
		{
			spdlog::warn("settings url error: {}", e.what());
			ret["msg"] = e.what();
			ret["status"] = 0;
		}

		callback(HttpResponse::newHttpJsonResponse(ret));
	}
}