#include "stdafx.h"
#include "user.h"
#include "httplib.h"
#include "jwt.h"
#include "utility.h"

namespace api
{
	void User::signup(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const
	{
		std::shared_ptr<Json::Value> json = req->getJsonObject();
		if (!json)
		{
			Json::Value ret;
			ret["msg"] = "error";
			ret["status"] = 0;
			return callback(HttpResponse::newHttpJsonResponse(ret));
		}

		auto now = std::chrono::system_clock::now();
		time_t time = std::chrono::system_clock::to_time_t(now);
		auto timestamp = fmt::format("{:%Y-%m-%d %H:%M:%S}", fmt::localtime(time));

		Json::Value addition;

		std::string id = uuid();

		auto db = app().getDbClient();
		db->execSqlAsync("INSERT INTO \"public\".\"account\" VALUES ($1, $2, $3, $4, '', '', '', '', '', '', '49d1a8d2-6cc5-47d3-9f27-015e7972c363', $5, $6, 'eebf50f2-95a8-4cab-915e-45d58bf697a1', $5, 'eebf50f2-95a8-4cab-915e-45d58bf697a1', $5, false, 'admin', 'admin', 'b1be135b-a9ed-422c-a18f-2bbe2c56a6b0')",
						 [=](const orm::Result& r)
		{
			Json::Value info;
		info["id"] = id;
		info["username"] = (*json)["username"].asString();
		info["password"] = (*json)["password"].asString();
		info["name"] = (*json)["nickname"].asString();
		info["head"] = "";
		info["phone"] = "";
		info["wechat"] = "";
		info["alipay"] = "";
		info["weibo"] = "";
		info["email"] = "";
		info["permissions"] = "49d1a8d2-6cc5-47d3-9f27-015e7972c363";
		info["options"] = "b1be135b-a9ed-422c-a18f-2bbe2c56a6b0";

		jwt jwtGenerated = jwt::generateToken(
			{
				{"id", picojson::value(id)},
			},
			(*json).isMember("remember") && (*json)["remember"].asBool());
		std::int64_t jwtExpiration = jwtGenerated.getExpiration();
		info["token"] = jwtGenerated.getToken();
		info["expiresIn"] =
			jwtExpiration -
			std::chrono::duration_cast<std::chrono::seconds>(
				std::chrono::system_clock::now().time_since_epoch())
			.count();
		info["expiresAt"] = jwtExpiration;

		Json::Value ret;
		ret["msg"] = info;
		ret["status"] = 1;

		callback(HttpResponse::newHttpJsonResponse(ret));
		},
						 [=](const orm::DrogonDbException& e)
		{
			Json::Value ret;
		ret["msg"] = "signup error";
		ret["status"] = 0;

		callback(HttpResponse::newHttpJsonResponse(ret));
		},
			id, (*json)["username"].asString(), (*json)["password"].asString(), (*json)["nickname"].asString(), timestamp, req->peerAddr().toIpPort());
	}

	void User::signin(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const
	{
		std::shared_ptr<Json::Value> json = req->getJsonObject();
		if (!json)
		{
			Json::Value ret;
			ret["msg"] = "error";
			ret["status"] = 0;
			return callback(HttpResponse::newHttpJsonResponse(ret));
		}

		auto db = app().getDbClient();
		db->execSqlAsync(
			"SELECT * FROM \"public\".\"account\" WHERE username = $1 AND delete = false",
			[=](const orm::Result& r)
		{
			if (r.size() > 0)
			{
				auto row = r[0];

				if (bcryptValidate((*json)["password"].asString(), row["password"].as<std::string>()))
				{
					Json::Value info;
					info["id"] = row["id"].as<std::string>();
					info["username"] = row["username"].as<std::string>();
					info["password"] = row["password"].as<std::string>();
					info["name"] = row["name"].as<std::string>();
					info["head"] = row["head"].as<std::string>();
					info["phone"] = row["phone"].as<std::string>();
					info["wechat"] = row["wechat"].as<std::string>();
					info["alipay"] = row["alipay"].as<std::string>();
					info["weibo"] = row["weibo"].as<std::string>();
					info["email"] = row["email"].as<std::string>();
					info["permissions"] = row["permissions"].as<std::string>();
					info["options"] = row["options"].as<std::string>();

					jwt jwtGenerated = jwt::generateToken(
						{
							{"id", picojson::value(row["id"].as<std::string>())},
						},
						(*json).isMember("remember") && (*json)["remember"].asBool());
					std::int64_t jwtExpiration = jwtGenerated.getExpiration();
					info["token"] = jwtGenerated.getToken();
					info["expiresIn"] =
						jwtExpiration -
						std::chrono::duration_cast<std::chrono::seconds>(
							std::chrono::system_clock::now().time_since_epoch())
						.count();
					info["expiresAt"] = jwtExpiration;

					Json::Value ret;
					ret["msg"] = info;
					ret["status"] = 1;

					callback(HttpResponse::newHttpJsonResponse(ret));
				}
				else
				{
					Json::Value ret;
					ret["msg"] = "password error";
					ret["status"] = 0;

					callback(HttpResponse::newHttpJsonResponse(ret));
				}
			}
			else
			{
				Json::Value ret;
				ret["msg"] = "user not exist";
				ret["status"] = 0;

				callback(HttpResponse::newHttpJsonResponse(ret));
			}
		},
			[=](const orm::DrogonDbException& e)
		{
			spdlog::warn("signin error: {}", e.base().what());
		Json::Value ret;
		ret["msg"] = e.base().what();
		ret["status"] = 0;
		callback(HttpResponse::newHttpJsonResponse(ret));
		},
			(*json)["username"].asString());
	}

	void User::signout(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const
	{}

	void User::info(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback, const std::string& id) const
	{
		auto db = app().getDbClient();
		db->execSqlAsync(
			"SELECT * FROM \"public\".\"account\" WHERE id = $1 AND delete = false",
			[=](const orm::Result& r)
		{
			if (r.size() > 0)
			{
				auto row = r[0];

				Json::Value user;
				user["id"] = row["id"].as<std::string>();
				user["username"] = row["username"].as<std::string>();
				user["password"] = row["password"].as<std::string>();
				user["name"] = row["name"].as<std::string>();
				user["head"] = row["head"].as<std::string>();
				user["phone"] = row["phone"].as<std::string>();
				user["wechat"] = row["wechat"].as<std::string>();
				user["alipay"] = row["alipay"].as<std::string>();
				user["weibo"] = row["weibo"].as<std::string>();
				user["email"] = row["email"].as<std::string>();
				user["permissions"] = row["permissions"].as<std::string>();
				user["options"] = row["options"].as<std::string>();

				auto db = app().getDbClient();
				db->execSqlAsync(
					"SELECT * FROM \"public\".\"permissions\" WHERE id = $1",
					[=](const orm::Result& r)
				{
					if (r.size() > 0)
					{
						auto row1 = r[0];
						Json::Value permissions;
						permissions["id"] = row1["id"].as<std::string>();
						permissions["name"] = row1["name"].as<std::string>();
						permissions["access"] = row1["access"].as<Json::Value>();

						auto db = app().getDbClient();
						db->execSqlAsync(
							"SELECT * FROM \"public\".\"options\" WHERE id = $1",
							[=](const orm::Result& r)
						{
							if (r.size() > 0)
							{
								auto row2 = r[0];

								Json::Value options;
								options["id"] = row2["id"].as<std::string>();
								options["name"] = row2["name"].as<std::string>();
								options["options"] = row2["options"].as<Json::Value>();
								options["level"] = row2["level"].as<INT64>();

								Json::Value info;
								info["user"] = user;
								info["permissions"] = permissions;
								info["options"] = options;

								Json::Value ret;
								ret["msg"] = info;
								ret["status"] = 1;

								callback(HttpResponse::newHttpJsonResponse(ret));
							}
							else
							{
								Json::Value ret;
								ret["msg"] = "no options";
								ret["status"] = 0;

								callback(HttpResponse::newHttpJsonResponse(ret));
							}
						},
							[=](const orm::DrogonDbException& e)
						{
							spdlog::warn("select options error: {}", e.base().what());

						Json::Value ret;
						ret["msg"] = e.base().what();
						ret["status"] = 0;

						callback(HttpResponse::newHttpJsonResponse(ret));
						}, row["options"].as<std::string>());
					}
				},
					[=](const orm::DrogonDbException& e)
				{
					spdlog::warn("select permissions error: {}", e.base().what());

				Json::Value ret;
				ret["msg"] = e.base().what();
				ret["status"] = 0;

				callback(HttpResponse::newHttpJsonResponse(ret));
				}, row["permissions"].as<std::string>());
			}
		},
			[=](const orm::DrogonDbException& e)
		{
			spdlog::warn("select user info error: {}", e.base().what());

		Json::Value ret;
		ret["msg"] = e.base().what();
		ret["status"] = 0;

		callback(HttpResponse::newHttpJsonResponse(ret));
		}, id);
	}

	void User::update(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const
	{
		std::shared_ptr<Json::Value> json = req->getJsonObject();
		if (!json)
		{
			Json::Value ret;
			ret["msg"] = "error";
			ret["status"] = 0;
			return callback(HttpResponse::newHttpJsonResponse(ret));
		}
	}
}