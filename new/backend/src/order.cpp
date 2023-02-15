#include "stdafx.h"
#include "order.h"
#include "httplib.h"
#include "utility.h"

namespace api
{
	void Order::count(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const
	{
		std::shared_ptr<Json::Value> json = req->getJsonObject();
		if (!json)
		{
			Json::Value ret;
			ret["msg"] = "error";
			ret["status"] = 0;
			return callback(HttpResponse::newHttpJsonResponse(ret));
		}

		std::string stmt = "SELECT COUNT(id) FROM \"public\".\"repair_order\" WHERE delete = false";

		if ((*json)["plate"].asString() != "")
			stmt = fmt::format("{} AND plate LIKE '{}'", stmt, (*json)["plate"].asString());

		if ((*json)["type"].asString() != "")
			stmt = fmt::format("{} AND type LIKE '{}'", stmt, (*json)["type"].asString());

		if ((*json)["class"].asString() != "")
			stmt = fmt::format("{} AND class LIKE '{}'", stmt, (*json)["class"].asString());

		if ((*json)["phone"].asString() != "")
			stmt = fmt::format("{} AND phone LIKE '{}'", stmt, (*json)["phone"].asString());

		if ((*json)["date_in"].asString() != "")
			stmt = fmt::format("{} AND date_in LIKE '{}'", stmt, (*json)["date_in"].asString());

		if ((*json)["date_settle"].asString() != "")
			stmt = fmt::format("{} AND date_settle LIKE '{}'", stmt, (*json)["date_settle"].asString());

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
			spdlog::warn("select repair_order error: {}", e.base().what());

		Json::Value ret;
		ret["msg"] = e.base().what();
		ret["status"] = 0;

		callback(HttpResponse::newHttpJsonResponse(ret));
		});
	}

	void Order::info(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const
	{
		std::shared_ptr<Json::Value> json = req->getJsonObject();
		if (!json)
		{
			Json::Value ret;
			ret["msg"] = "error";
			ret["status"] = 0;
			return callback(HttpResponse::newHttpJsonResponse(ret));
		}

		std::string stmt = "SELECT * FROM \"public\".\"repair_order\" WHERE delete = false";

		if ((*json)["plate"].asString() != "")
			stmt = fmt::format("{} AND plate LIKE '{}'", stmt, (*json)["plate"].asString());

		if ((*json)["type"].asString() != "")
			stmt = fmt::format("{} AND type LIKE '{}'", stmt, (*json)["type"].asString());

		if ((*json)["class"].asString() != "")
			stmt = fmt::format("{} AND class LIKE '{}'", stmt, (*json)["class"].asString());

		if ((*json)["phone"].asString() != "")
			stmt = fmt::format("{} AND phone LIKE '{}'", stmt, (*json)["phone"].asString());

		if ((*json)["date_in"].asString() != "")
			stmt = fmt::format("{} AND date_in LIKE '{}'", stmt, (*json)["date_in"].asString());

		if ((*json)["date_settle"].asString() != "")
			stmt = fmt::format("{} AND date_settle LIKE '{}'", stmt, (*json)["date_settle"].asString());

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
				info["plate"] = row["plate"].as<std::string>();
				info["type"] = row["type"].as<std::string>();
				info["date_in"] = row["date_in"].as<std::string>();
				info["date_settle"] = row["date_settle"].as<std::string>();
				info["class"] = row["class"].as<std::string>();
				info["vin"] = row["vin"].as<std::string>();
				info["mileage"] = row["mileage"].as<std::string>();
				info["phone"] = row["phone"].as<std::string>();
				info["project"] = row["project"].as<Json::Value>();
				info["parts"] = row["parts"].as<Json::Value>();
				info["cost_manhour"] = row["cost_manhour"].as<Json::Value>();
				info["cost_materials"] = row["cost_materials"].as<Json::Value>();
				info["cost_other"] = row["cost_other"].as<Json::Value>();
				info["amount_total"] = row["amount_total"].as<double>();
				info["amount_receipt"] = row["amount_receipt"].as<double>();
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
			spdlog::warn("select repair_order error: {}", e.base().what());

		Json::Value ret;
		ret["msg"] = e.base().what();
		ret["status"] = 0;

		callback(HttpResponse::newHttpJsonResponse(ret));
		});
	}

	void Order::update(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const
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
			"UPDATE \"public\".\"repair_order\" SET plate = $1, type = $2, date_in = $3, date_settle = $4, class = $5, vin = $6, mileage = $7, phone = $8, project = $9, parts = $10, cost_manhour = $11, cost_materials = $12, cost_other = $13, amount_total = $14, amount_receipt = $15, status = $16, remark = $17, update_date = $18, update_user = $19, update_id = $20, delete = false WHERE id = $21",
			[=](const orm::Result& r)
		{
			Json::Value ret;

		if (r.affectedRows() > 0)
		{
			Json::Value storage = (*json)["parts"];
			auto db = app().getDbClient();
			db->execSqlAsync(
				"UPDATE \"public\".\"repair_storage\" SET num = $1, update_date = $2, update_user = $3, update_id = $4 WHERE id = $5",
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
				spdlog::warn("update repair_order error: {}", e.base().what());

			Json::Value ret;
			ret["msg"] = e.base().what();
			ret["status"] = 0;

			callback(HttpResponse::newHttpJsonResponse(ret));
			}, storage["storage"].asFloat() - storage["num"].asFloat(), timestamp, (*json)["update_user"].asString(), (*json)["update_id"].asString(), storage["id"].asString());
		}
		else
		{
			ret["msg"] = "error";
			ret["status"] = 0;

			callback(HttpResponse::newHttpJsonResponse(ret));
		}
		},
			[=](const orm::DrogonDbException& e)
		{
			spdlog::warn("update repair_order error: {}", e.base().what());

		Json::Value ret;
		ret["msg"] = e.base().what();
		ret["status"] = 0;

		callback(HttpResponse::newHttpJsonResponse(ret));
		}, (*json)["plate"].asString(), (*json)["type"].asString(), (*json)["date_in"].asString(), (*json)["date_settle"].asString(), (*json)["class"].asString(), (*json)["vin"].asString(), (*json)["mileage"].asString(), (*json)["phone"].asString(), (*json)["project"].toStyledString(), (*json)["parts"].toStyledString(), (*json)["cost_manhour"].toStyledString(), (*json)["cost_materials"].toStyledString(), (*json)["cost_other"].toStyledString(), (*json)["amount_total"].asDouble(), (*json)["amount_receipt"].asDouble(), (*json)["status"].asString(), (*json)["remark"].asString(), timestamp, (*json)["update_user"].asString(), (*json)["update_id"].asString(), (*json)["id"].asString());
	}

	Task<Json::Value> Order::insert(const Json::Value json) const
	{
		try
		{
			auto now = std::chrono::system_clock::now();
			time_t time = std::chrono::system_clock::to_time_t(now);
			auto timestamp = fmt::format("{:%Y-%m-%d %H:%M:%S}", fmt::localtime(time));

			auto db = app().getDbClient();
			drogon::orm::Result result;

			Json::Value storage = json["parts"];
			for (auto i = storage.begin(); i != storage.end(); i++)
			{
				result = co_await db->execSqlCoro(
					"UPDATE \"public\".\"repair_storage\" SET num = $1, update_date = $2, update_user = $3, update_id = $4 WHERE id = $5", (*i)["storage"].asInt() - (*i)["num"].asInt(), timestamp, json["update_user"].asString(), json["update_id"].asString(), (*i)["id"].asString());
			}

			Json::Value ret;
			if (result.affectedRows() > 0)
			{
				auto result1 = co_await db->execSqlCoro(
					"INSERT INTO \"public\".\"repair_order\" VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, false)",
					uuid(), json["plate"].asString(), json["type"].asString(), json["date_in"].asString(), json["date_settle"].asString(), json["class"].asString(), json["vin"].asString(), json["mileage"].asString(), json["phone"].asString(), json["project"].toStyledString(), json["parts"].toStyledString(), json["cost_manhour"].toStyledString(), json["cost_materials"].toStyledString(), json["cost_other"].toStyledString(), json["amount_total"].asDouble(), json["amount_receipt"].asDouble(), json["status"].asString(), json["remark"].asString(), timestamp, json["create_user"].asString(), json["create_id"].asString(), timestamp, json["update_user"].asString(), json["update_id"].asString());

				if (result1.affectedRows() > 0)
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
			else
			{
				ret["msg"] = "error";
				ret["status"] = 0;
			}
			co_return ret;
		}
		catch (const orm::DrogonDbException& e)
		{
			spdlog::warn("insert repair_order error: {}", e.base().what());

			Json::Value ret;
			ret["msg"] = e.base().what();
			ret["status"] = 0;
			co_return ret;
		}
	}
}