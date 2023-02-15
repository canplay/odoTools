#include "stdafx.h"
#include "storage.h"
#include "httplib.h"
#include "utility.h"

namespace api
{
	void Storage::count(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const
	{
		std::shared_ptr<Json::Value> json = req->getJsonObject();
		if (!json)
		{
			Json::Value ret;
			ret["msg"] = "error";
			ret["status"] = 0;
			return callback(HttpResponse::newHttpJsonResponse(ret));
		}

		std::string stmt = "SELECT COUNT(id) FROM \"public\".\"repair_storage\" WHERE delete = false";

		if ((*json)["code"].asString() != "")
			stmt = fmt::format("{} AND code LIKE '{}'", stmt, (*json)["code"].asString());

		if ((*json)["name"].asString() != "")
			stmt = fmt::format("{} AND type LIKE '{}'", stmt, (*json)["name"].asString());

		auto db = app().getDbClient();
		db->execSqlAsync(
			stmt,
			[=](const orm::Result& r)
		{
			Json::Value ret;

		if (r.size() > 0)
		{
			auto row = r[0];
			ret["msg"] = row["count"].as<int>();
			ret["status"] = 1;
		}
		else
		{
			ret["msg"] = 0;
			ret["status"] = 1;
		}

		callback(HttpResponse::newHttpJsonResponse(ret));
		},
			[=](const orm::DrogonDbException& e)
		{
			spdlog::warn("select repair_storage error: {}", e.base().what());

		Json::Value ret;
		ret["msg"] = e.base().what();
		ret["status"] = 0;

		callback(HttpResponse::newHttpJsonResponse(ret));
		});
	}

	void Storage::info(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const
	{
		std::shared_ptr<Json::Value> json = req->getJsonObject();
		if (!json)
		{
			Json::Value ret;
			ret["msg"] = "error";
			ret["status"] = 0;
			return callback(HttpResponse::newHttpJsonResponse(ret));
		}

		std::string stmt = "SELECT * FROM \"public\".\"repair_storage\" WHERE delete = false";

		if ((*json)["code"].asString() != "")
			stmt = fmt::format("{} AND code LIKE '{}'", stmt, (*json)["code"].asString());

		if ((*json)["name"].asString() != "")
			stmt = fmt::format("{} AND type LIKE '{}'", stmt, (*json)["name"].asString());

		if ((*json)["sortBy"].asString() != "")
		{
			stmt = fmt::format("{} ORDER BY {}", stmt, (*json)["sortBy"].asString());

			if ((*json)["descending"].asBool())
				stmt = fmt::format("{} DESC", stmt);
			else
				stmt = fmt::format("{} ASC", stmt);
		}

		stmt = fmt::format("{} LIMIT {} OFFSET {}", stmt, (*json)["maxPage"].asString(), (*json)["curPage"].asString());

		auto db = app().getDbClient();
		db->execSqlAsync(
			stmt,
			[=](const orm::Result& r)
		{
			Json::Value ret;

		if (r.size() > 0)
		{
			Json::Value infos;

			for (size_t i = 0; i < r.size(); i++)
			{
				auto row = r[i];

				Json::Value info;
				info["id"] = row["id"].as<std::string>();
				info["code"] = row["code"].as<std::string>();
				info["name"] = row["name"].as<std::string>();
				info["tag"] = row["tag"].as<std::string>();
				info["num"] = row["num"].as<int>();
				info["unit"] = row["unit"].as<std::string>();
				info["price"] = row["price"].as<double>();
				info["date_in"] = row["date_in"].as<std::string>();
				info["date_out"] = row["date_out"].as<std::string>();
				info["status"] = row["status"].as<std::string>();
				info["remark"] = row["remark"].as<std::string>();
				info["create_date"] = row["create_date"].as<std::string>();
				info["create_user"] = row["create_user"].as<std::string>();
				info["create_id"] = row["create_id"].as<std::string>();
				info["update_date"] = row["update_date"].as<std::string>();
				info["update_user"] = row["update_user"].as<std::string>();
				info["update_id"] = row["update_id"].as<std::string>();

				infos.append(info);
			}

			ret["msg"] = infos;
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
			spdlog::warn("select repair_storage error: {}", e.base().what());

		Json::Value ret;
		ret["msg"] = e.base().what();
		ret["status"] = 0;

		callback(HttpResponse::newHttpJsonResponse(ret));
		});
	}

	void Storage::update(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const
	{
		std::shared_ptr<Json::Value> json = req->getJsonObject();
		if (!json)
		{
			Json::Value ret;
			ret["msg"] = "error";
			ret["status"] = 0;
			return callback(HttpResponse::newHttpJsonResponse(ret));
		}

		if ((*json)["id"].asString() == "")
		{
			auto func = async_func([=]() -> Task<void>
			{
				auto r = co_await insert((*json));
			callback(HttpResponse::newHttpJsonResponse(r));
			});
			func();
			return;
		}

		auto now = std::chrono::system_clock::now();
		time_t time = std::chrono::system_clock::to_time_t(now);
		auto timestamp = fmt::format("{:%Y-%m-%d %H:%M:%S}", fmt::localtime(time));

		auto db = app().getDbClient();
		db->execSqlAsync(
			"UPDATE \"public\".\"repair_storage\" SET code = $1, name = $2, tag = $3, num = $4, unit = $5, price = $6, img = $7, date_in = $8, date_out = $9, status = $10, remark = $11, update_date = $12, update_user = $13, update_id = $14, delete = false WHERE id = $15",
			[=](const orm::Result& r)
		{
			Json::Value ret;

		if (r.affectedRows() > 0)
		{
			auto row = r[0];
			ret["msg"] = "ok";
			ret["status"] = 1;
		}
		else
		{
			ret["msg"] = "error";
			ret["status"] = 0;
		}

		callback(HttpResponse::newHttpJsonResponse(ret));
		},
			[=](const orm::DrogonDbException& e)
		{
			spdlog::warn("update repair_storage error: {}", e.base().what());

		Json::Value ret;
		ret["msg"] = e.base().what();
		ret["status"] = 0;

		callback(HttpResponse::newHttpJsonResponse(ret));
		}, (*json)["code"].asString(), (*json)["name"].asString(), (*json)["tag"].asString(), (*json)["num"].asInt(), (*json)["unit"].asString(), (*json)["price"].asDouble(), (*json)["img"].asString(), (*json)["date_in"].asString(), (*json)["date_out"].asString(), (*json)["status"].asString(), (*json)["remark"].asString(), timestamp, (*json)["update_user"].asString(), (*json)["update_id"].asString(), (*json)["id"].asString());
	}

	Task<Json::Value> Storage::insert(const Json::Value json) const
	{
		try
		{
			auto now = std::chrono::system_clock::now();
			time_t time = std::chrono::system_clock::to_time_t(now);
			auto timestamp = fmt::format("{:%Y-%m-%d %H:%M:%S}", fmt::localtime(time));

			auto db = app().getDbClient();
			auto result = co_await db->execSqlCoro(
				"INSERT INTO \"public\".\"repair_storage\" VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, false)",
				uuid(), json["code"].asString(), json["name"].asString(), json["tag"].asString(), json["num"].asInt(), json["unit"].asString(), json["price"].asDouble(), json["date_in"].asString(), json["date_out"].asString(), json["status"].asString(), json["remark"].asString(), timestamp, json["create_user"].asString(), json["create_id"].asString(), timestamp, json["update_user"].asString(), json["update_id"].asString(), json["img"].asString());

			Json::Value ret;
			if (result.affectedRows() > 0)
			{
				ret["msg"] = "ok";
				ret["status"] = 1;
			}
			else
			{
				ret["msg"] = "error";
				ret["status"] = 0;
			}
			co_return ret;
		}
		catch (const orm::DrogonDbException& e)
		{
			spdlog::warn("insert repair_storage error: {}", e.base().what());

			Json::Value ret;
			ret["msg"] = e.base().what();
			ret["status"] = 0;
			co_return ret;
		}
	}
}