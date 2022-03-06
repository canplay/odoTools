#include "stdafx.h"
#include "user.h"
#include "utility.h"

using namespace std;
using namespace std::literals::chrono_literals;
using namespace std::chrono;

struct _token
{
	string token = "";
	time_t time = 0;
} g_token;

api::v1::User::User()
{
}

void api::v1::User::register(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const
{
	time_t t = system_clock::to_time_t(system_clock::now());
	auto tm_Prev = fmt::localtime(g_token.time);
	auto tm_Now = fmt::localtime(t);
	auto time = fmt::format("{:%Y-%m-%d %H:%M:%S}", fmt::localtime(t));

	LOG_INFO << tm_Now.tm_min - tm_Prev.tm_min;
	if (tm_Now.tm_min - tm_Prev.tm_min >= 40 || g_token.time == 0) {
		get_token();
	}

	Json::Reader reader;
	Json::Value reqJson;
	if (reader.parse(req->body().data(), reqJson)) {
		Json::Value json;
		json["billDate"] = time;
		json["billDesc"] = reqJson["desc"].asString();
		json["billNo"] = reqJson["no"].asString();
		json["instMid"] = "QRPAYDEFAULT";
		json["mid"] = "898532680998463";
		json["requestTimestamp"] = time;
		json["tid"] = "53265456";
		json["totalAmount"] = reqJson["amount"].asFloat() * 100;
	}

	auto res = HttpResponse::newNotFoundResponse();
	callback(res);
}