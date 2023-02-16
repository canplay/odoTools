#include "stdafx.h"
#include "user.h"
#include "httplib.h"
#include "jwt.h"
#include "utility.h"
#include "mssql.h"

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

		auto stmt = fmt::format("SELECT * FROM [SA_BETA_WORLDDB_0002].[PaGamePrivate].[TblUserInformation] WHERE _userId LIKE '{},%'", (*json)["username"].asString());

		Json::Value ret;

		try
		{
			Json::Value info;

			auto r = MsSql::exec(stmt);
			r.next();

			if (r.rows() > 1)
			{
				ret["msg"] = "username is exist";
				ret["status"] = 1;
				callback(HttpResponse::newHttpJsonResponse(ret));
			}
			else
			{
				std::random_device rd;

				auto stmt = fmt::format("INSERT INTO [SA_BETA_WORLDDB_0002].[PaGamePrivate].[TblUserInformation] ([_registerDate], [_isValid], [_userId], [_userNickname], [_password], [_paymentPassword], [_authenticKey], [_authenticExpiration], [_webAuthenticKey], [_lastLoginTime], [_lastLogoutTime], [_totalPlayTime], [_lastIp], [_lastServerNo], [_lastWorldNo], [_serviceType], [_failPasswordCount], [_publisherCryptToken], [_membershipType], [_isAdmissionToSpeedServer], [_isPcRoom], [_expirationDate], [_isGuestAccount], [_speedServerExpiration], [_accountExpirationDate], [_surveyHWAndSW], [_isAccessBlanceChannel], [_isPremiumChannelPermission], [_isIgnoreCheckCustomizeOnly], [_preAuthenticKey], [_isAdultWorldUser], [_shutDownTime], [_atField], [_isCompleteTesterSubmit], [_isOtp], [_lastMacAddress], [_allCharacterTotalLevel], [_isAppliedNickNameChange]) VALUES('{}', '1', N'{},{}', N'Adventurer{}', 0xF6ADBC1E77E92C0F1725B83DCFCFA139AD87EABCB262E1E3652B485F427961BDC57382A8A729331B7FF8A7A90FFBFE80ED43EF1060FE5F7716266DC48227AED0, NULL, 0, '{}', 0, '{}', '{}', 0, '127.0.0.1', -110, 12, 12, 0, NULL, 0, 2, '1', NULL, '0', '{}', NULL, '', '1', '0', '0', 0, '1', NULL, '', '0', '0', 0xF728BB816503A79FCCBF76A7AA2F6AF0294BBCCAB8E3140ECAEEAED08E2329A2228983F442A858128C9EA83ED089A12DA0D7076FF52E2F313466E7D42A0130CC, 0, '0')", timestamp, (*json)["username"].asString(), (*json)["password"].asString(), rd(), timestamp, timestamp, timestamp, timestamp);

				auto r = MsSql::exec(stmt);

				if (r.affected_rows() == 1)
				{
					auto stmt = fmt::format("SELECT * FROM [SA_BETA_WORLDDB_0002].[PaGamePrivate].[TblUserInformation] WHERE _userId = '{},{}'", (*json)["username"].asString(), (*json)["password"].asString());

					Json::Value info;

					auto r = MsSql::exec(stmt);
					r.next();
					info["registerDate"] = r.get<std::string>(0);
					info["isValid"] = r.get<std::string>(1);
					info["userNo"] = r.get<int>(2);
					info["userId"] = r.get<std::string>(3);
					info["userNickname"] = r.get<std::string>(4);
					info["lastLoginTime"] = r.get<std::string>(10);
					info["lastLogoutTime"] = r.get<std::string>(11);
					info["totalPlayTime"] = r.get<int>(12);
					info["lastServerNo"] = r.get<int>(14);
					info["lastWorldNo"] = r.get<int>(15);
					info["serviceType"] = r.get<int>(16);
					info["failPasswordCount"] = r.get<int>(17);
					info["membershipType"] = r.get<int>(19);

					jwt jwtGenerated = jwt::generateToken(
						{
							{"id", picojson::value(info["userNo"].asString())},
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

					ret["msg"] = info;
					ret["status"] = 1;
				}
				else
				{
					ret["msg"] = "signup error";
					ret["status"] = 0;
				}
			}
		}
		catch (const std::exception& e)
		{
			spdlog::warn("signup error: {}", e.what());
			ret["msg"] = e.what();
			ret["status"] = 0;
		}

		callback(HttpResponse::newHttpJsonResponse(ret));
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

		auto stmt = fmt::format("SELECT * FROM [SA_BETA_WORLDDB_0002].[PaGamePrivate].[TblUserInformation] WHERE _userId = '{},{}'", (*json)["username"].asString(), (*json)["password"].asString());

		Json::Value ret;

		try
		{
			Json::Value info;

			auto r = MsSql::exec(stmt);
			r.next();
			info["registerDate"] = r.get<std::string>(0);
			info["isValid"] = r.get<std::string>(1);
			info["userNo"] = r.get<int>(2);
			info["userId"] = r.get<std::string>(3);
			info["userNickname"] = r.get<std::string>(4);
			info["lastLoginTime"] = r.get<std::string>(10);
			info["lastLogoutTime"] = r.get<std::string>(11);
			info["totalPlayTime"] = r.get<int>(12);
			info["lastServerNo"] = r.get<int>(14);
			info["lastWorldNo"] = r.get<int>(15);
			info["serviceType"] = r.get<int>(16);
			info["failPasswordCount"] = r.get<int>(17);
			info["membershipType"] = r.get<int>(19);

			jwt jwtGenerated = jwt::generateToken(
				{
					{"id", picojson::value(info["userNo"].asString())},
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

			ret["msg"] = info;
			ret["status"] = 1;
		}
		catch (const std::exception& e)
		{
			spdlog::warn("signup error: {}", e.what());
			ret["msg"] = e.what();
			ret["status"] = 0;
		}

		callback(HttpResponse::newHttpJsonResponse(ret));
	}

	void User::signout(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const
	{}

	void User::info(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback, const std::string& id) const
	{
		auto stmt = fmt::format("SELECT * FROM [SA_BETA_WORLDDB_0002].[PaGamePrivate].[TblUserInformation] WHERE _userNo = {}", id);

		Json::Value ret;

		try
		{
			Json::Value info;

			auto r = MsSql::exec(stmt);
			r.next();
			info["registerDate"] = r.get<std::string>(0);
			info["isValid"] = r.get<std::string>(1);
			info["userNo"] = r.get<int>(2);
			info["userId"] = r.get<std::string>(3);
			info["userNickname"] = r.get<std::string>(4);
			info["lastLoginTime"] = r.get<std::string>(10);
			info["lastLogoutTime"] = r.get<std::string>(11);
			info["totalPlayTime"] = r.get<int>(12);
			info["lastServerNo"] = r.get<int>(14);
			info["lastWorldNo"] = r.get<int>(15);
			info["serviceType"] = r.get<int>(16);
			info["failPasswordCount"] = r.get<int>(17);
			info["membershipType"] = r.get<int>(19);

			ret["msg"] = info;
			ret["status"] = 1;
		}
		catch (const std::exception& e)
		{
			spdlog::warn("info error: {}", e.what());
			ret["msg"] = e.what();
			ret["status"] = 0;
		}

		callback(HttpResponse::newHttpJsonResponse(ret));
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