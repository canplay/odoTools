#include "stdafx.h"
#include "staff.h"

namespace api
{
	void Staff::info(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const
	{
		std::shared_ptr<Json::Value> json = req->getJsonObject();
		if (!json)
		{
			Json::Value ret;
			ret["msg"] = "error";
			ret["status"] = 0;
			return callback(HttpResponse::newHttpJsonResponse(ret));
		}

		auto db = app().getDbClient("jc");
		db->execSqlAsync(
			"SELECT * FROM t_s_base_user WHERE id = ?",
			[=](const orm::Result& r)
		{
			Json::Value ret;
		Json::Value info;

		if (r.size() > 0)
		{
			auto row = r[0];
			info["activitiSync"] = row["activitiSync"].as<std::string>();
			info["browser"] = row["browser"].as<std::string>();
			info["realname"] = row["realname"].as<std::string>();
			info["status"] = row["status"].as<int>();
			info["userkey"] = row["userkey"].as<std::string>();
			info["username"] = row["username"].as<std::string>();
			info["departid"] = row["departid"].as<std::string>();
			info["location"] = row["location"].as<std::string>();

			ret["msg"] = info;
			ret["status"] = 1;
		}
		else
		{
			ret["msg"] = "no data";
			ret["status"] = 0;
		}

		callback(HttpResponse::newHttpJsonResponse(ret));
		},
			[=](const orm::DrogonDbException& e)
		{
			spdlog::warn("select sf_formal_bill error: {}", e.base().what());

		Json::Value ret;
		ret["msg"] = e.base().what();
		ret["status"] = 0;

		callback(HttpResponse::newHttpJsonResponse(ret));
		}, (*json)["id"].asString());
	}
}