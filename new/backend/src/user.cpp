#include "stdafx.h"
#include "user.h"
#include "httplib.h"
#include "jwt.h"
#include "utility.h"
#include "mssql.h"

namespace api
{
	void User::count(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const
	{
		auto stmt = fmt::format("SELECT COUNT(_userNo) FROM [SA_BETA_WORLDDB_0002].[PaGamePrivate].[TblUserInformation]");

		Json::Value ret;

		try
		{
			Json::Value info;

			auto r = MsSql::exec(stmt);
			r.next();
			ret["msg"] = r.get<int>(0);
			ret["status"] = 1;
		}
		catch (const std::exception& e)
		{
			spdlog::warn("user count error: {}", e.what());
			ret["msg"] = e.what();
			ret["status"] = 0;
		}

		callback(HttpResponse::newHttpJsonResponse(ret));
	}

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

			if (r.rows() != 0)
			{
				ret["msg"] = "username already exist";
				ret["status"] = 0;
				return callback(HttpResponse::newHttpJsonResponse(ret));
			}
			else
			{
				std::random_device rd;

				auto stmt = fmt::format("INSERT INTO [SA_BETA_WORLDDB_0002].[PaGamePrivate].[TblUserInformation] ([_registerDate], [_isValid], [_userId], [_userNickname], [_password], [_paymentPassword], [_authenticKey], [_authenticExpiration], [_webAuthenticKey], [_lastLoginTime], [_lastLogoutTime], [_totalPlayTime], [_lastIp], [_lastServerNo], [_lastWorldNo], [_serviceType], [_failPasswordCount], [_publisherCryptToken], [_membershipType], [_isAdmissionToSpeedServer], [_isPcRoom], [_expirationDate], [_isGuestAccount], [_speedServerExpiration], [_accountExpirationDate], [_surveyHWAndSW], [_isAccessBlanceChannel], [_isPremiumChannelPermission], [_isIgnoreCheckCustomizeOnly], [_preAuthenticKey], [_isAdultWorldUser], [_shutDownTime], [_atField], [_isCompleteTesterSubmit], [_isOtp], [_lastMacAddress], [_allCharacterTotalLevel], [_isAppliedNickNameChange]) VALUES('{}', '1', N'{},{}', N'Adventurer{}', 0xF6ADBC1E77E92C0F1725B83DCFCFA139AD87EABCB262E1E3652B485F427961BDC57382A8A729331B7FF8A7A90FFBFE80ED43EF1060FE5F7716266DC48227AED0, 0xF6ADBC1E77E92C0F1725B83DCFCFA139AD87EABCB262E1E3652B485F427961BDC57382A8A729331B7FF8A7A90FFBFE80ED43EF1060FE5F7716266DC48227AED0, 0, '{}', 0, '{}', '{}', 0, '127.0.0.1', -110, 12, 12, 0, NULL, 0, 2, '1', NULL, '0', '{}', NULL, '', '1', '0', '0', 0, '1', NULL, '', '0', '0', 0xF728BB816503A79FCCBF76A7AA2F6AF0294BBCCAB8E3140ECAEEAED08E2329A2228983F442A858128C9EA83ED089A12DA0D7076FF52E2F313466E7D42A0130CC, 0, '0')", timestamp, (*json)["username"].asString(), (*json)["password"].asString(), rd(), timestamp, timestamp, timestamp, timestamp);

				auto r = MsSql::exec(stmt);

				if (r.affected_rows() == 1)
				{
					auto stmt = fmt::format("SELECT * FROM [SA_BETA_WORLDDB_0002].[PaGamePrivate].[TblUserInformation] WHERE _userId = '{},{}'", (*json)["username"].asString(), (*json)["password"].asString());

					Json::Value info;

					auto r = MsSql::exec(stmt);
					r.next();
					info["registerDate"] = r.get<std::string>("_registerDate");
					info["valid"] = r.get<std::string>("_isValid");
					info["userNo"] = r.get<int>("_userNo");
					info["userId"] = r.get<std::string>("_userId");
					info["userNickname"] = r.get<std::string>("_userNickname");
					info["lastLoginTime"] = r.get<std::string>("_lastLoginTime");
					info["lastLogoutTime"] = r.get<std::string>("_lastLogoutTime");
					info["totalPlayTime"] = r.get<int>("_totalPlayTime");
					info["membershipType"] = r.get<int>("_membershipType");
					info["pcroom"] = r.get<int>("_isPcRoom");

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

			if (r.rows() != 0)
			{
				info["registerDate"] = r.get<std::string>("_registerDate");
				info["valid"] = r.get<std::string>("_isValid");
				info["userNo"] = r.get<int>("_userNo");
				info["userId"] = r.get<std::string>("_userId");
				info["userNickname"] = r.get<std::string>("_userNickname");
				info["lastLoginTime"] = r.get<std::string>("_lastLoginTime");
				info["lastLogoutTime"] = r.get<std::string>("_lastLogoutTime");
				info["totalPlayTime"] = r.get<int>("_totalPlayTime");
				info["membershipType"] = r.get<int>("_membershipType");
				info["pcroom"] = r.get<int>("_isPcRoom");

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
				ret["msg"] = "username or password error";
				ret["status"] = 0;
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

	void User::signout(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const
	{}

	void User::info(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const
	{
		std::shared_ptr<Json::Value> json = req->getJsonObject();
		if (!json)
		{
			Json::Value ret;
			ret["msg"] = "error";
			ret["status"] = 0;
			return callback(HttpResponse::newHttpJsonResponse(ret));
		}

		std::string stmt = fmt::format("SELECT TOP {} * FROM [SA_BETA_WORLDDB_0002].[PaGamePrivate].[TblUserInformation] WHERE _userNo NOT IN(SELECT TOP {} _userNo FROM [SA_BETA_WORLDDB_0002].[PaGamePrivate].[TblUserInformation]", (*json)["maxPage"].asInt64(), (*json)["curPage"].asInt64());

		if ((*json)["sortBy"].asString() != "")
		{
			stmt = fmt::format("{} ORDER BY [{}] DESC) ORDER BY [{}]", stmt, (*json)["sortBy"].asString(), (*json)["sortBy"].asString());

			if ((*json)["descending"].asBool())
				stmt = fmt::format("{} DESC", stmt);
			else
				stmt = fmt::format("{} ASC", stmt);
		}
		else
			stmt = fmt::format("{})", stmt);

		Json::Value ret;

		try
		{
			Json::Value infos;

			auto r = MsSql::exec(stmt);

			while (r.next())
			{
				Json::Value info;
				info["registerDate"] = r.get<std::string>("_registerDate");
				info["valid"] = r.get<std::string>("_isValid");
				info["userNo"] = r.get<int>("_userNo");
				info["userId"] = r.get<std::string>("_userId");
				info["userNickname"] = r.get<std::string>("_userNickname");
				info["lastLoginTime"] = r.get<std::string>("_lastLoginTime");
				info["lastLogoutTime"] = r.get<std::string>("_lastLogoutTime");
				info["totalPlayTime"] = r.get<int>("_totalPlayTime");
				info["membershipType"] = r.get<int>("_membershipType");
				info["pcroom"] = r.get<int>("_isPcRoom");

				Json::Value characters;
				auto stmt1 = fmt::format("SELECT * FROM [SA_BETA_GAMEDB_0002].[PaGamePrivate].[TblCharacterInformation] WHERE [_userNo] = {}", r.get<int>("_userNo"));
				auto r1 = MsSql::exec(stmt1);
				while (r1.next())
				{
					Json::Value character;
					character["characterName"] = r1.get<std::string>("_characterName");
					character["classType"] = r1.get<int>("_classType");
					character["totalPlayTime"] = r1.get<int>("_totalPlayTime");
					character["positionX"] = r1.get<int>("_currentPositionX");
					character["positionY"] = r1.get<int>("_currentPositionY");
					character["positionZ"] = r1.get<int>("_currentPositionZ");
					character["level"] = r1.get<int>("_level");
					character["experience"] = r1.get<int>("_experience");
					character["skillPointLevel"] = r1.get<int>("_skillPointLevel");
					character["skillPointExperience"] = r1.get<int>("_skillPointExperience");
					character["hp"] = r1.get<int>("_hp");
					character["mp"] = r1.get<int>("_mp");
					character["sp"] = r1.get<int>("_sp");
					character["wp"] = r1.get<int>("_wp");
					character["inventorySlotCount"] = r1.get<int>("_inventorySlotCount");
					character["variedWeight"] = r1.get<int>("_variedWeight");
					character["offenceValue"] = r1.get<int>("_offenceValue");
					character["defenceValue"] = r1.get<int>("_defenceValue");
					character["awakenValue"] = r1.get<int>("_awakenValue");
					characters.append(character);
				}

				info["characters"] = characters;
				infos.append(info);
			}

			ret["msg"] = infos;
			ret["status"] = 1;
		}
		catch (const std::exception& e)
		{
			spdlog::warn("user info error: {}", e.what());
			ret["msg"] = e.what();
			ret["status"] = 0;
		}

		callback(HttpResponse::newHttpJsonResponse(ret));
	}

	void User::infoOne(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback, const std::string& id) const
	{
		auto stmt = fmt::format("SELECT * FROM [SA_BETA_WORLDDB_0002].[PaGamePrivate].[TblUserInformation] WHERE _userNo = {}", id);

		Json::Value ret;

		try
		{
			Json::Value info;

			auto r = MsSql::exec(stmt);
			r.next();
			info["registerDate"] = r.get<std::string>("_registerDate");
			info["valid"] = r.get<std::string>("_isValid");
			info["userNo"] = r.get<int>("_userNo");
			info["userId"] = r.get<std::string>("_userId");
			info["userNickname"] = r.get<std::string>("_userNickname");
			info["lastLoginTime"] = r.get<std::string>("_lastLoginTime");
			info["lastLogoutTime"] = r.get<std::string>("_lastLogoutTime");
			info["totalPlayTime"] = r.get<int>("_totalPlayTime");
			info["membershipType"] = r.get<int>("_membershipType");
			info["pcroom"] = r.get<int>("_isPcRoom");

			ret["msg"] = info;
			ret["status"] = 1;
		}
		catch (const std::exception& e)
		{
			spdlog::warn("user info one error: {}", e.what());
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

		auto now = std::chrono::system_clock::now();
		time_t time = std::chrono::system_clock::to_time_t(now);
		auto timestamp = fmt::format("{:%Y-%m-%d %H:%M:%S}", fmt::localtime(time));

		auto stmt = fmt::format("UPDATE [SA_BETA_WORLDDB_0002].[PaGamePrivate].[TblUserInformation] SET [_isValid] = '1', [_userId] = N'{},{}', [_userNickname] = N'{}', [_isPcRoom] = '{}' WHERE [_userNo] = {};", (*json)["vaild"].asInt(), (*json)["username"].asString(), (*json)["password"].asString(), (*json)["nickname"].asString(), (*json)["pcroom"].asInt(), (*json)["no"].asInt());

		Json::Value ret;

		try
		{
			auto r = MsSql::exec(stmt);

			if (r.affected_rows() == 1)
			{
				ret["msg"] = "ok";
				ret["status"] = 1;
			}
			else
			{
				ret["msg"] = "news add error";
				ret["status"] = 0;
			}
		}
		catch (const std::exception& e)
		{
			spdlog::warn("news add error: {}", e.what());
			ret["msg"] = e.what();
			ret["status"] = 0;
		}

		callback(HttpResponse::newHttpJsonResponse(ret));
	}
}