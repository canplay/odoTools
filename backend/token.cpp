#include "stdafx.h"
#include "token.h"
#include "utility.h"

using namespace std;
using namespace std::literals::chrono_literals;
using namespace std::chrono;

api::v1::Token::Token()
{
}

void api::v1::Token::generate(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const
{
    system_clock::time_point now = system_clock::now();
    time_t t = system_clock::to_time_t(now);
    string nonce = utility::random128();
    string timestamp = fmt::format("{:%Y%m%d%H%M%S}", fmt::localtime(t));
    string sign = utility::sha256(fmt::format("8a81c1bd7e734154017ed397603602a1{}{}5b50748d6f0542028a489bbaea001d7f", timestamp, nonce));

    Json::Value json;
    json["appId"] = "8a81c1bd7e734154017ed397603602a1";
    json["timestamp"] = timestamp;
    json["nonce"] = nonce;
    json["signMethod"] = "SHA256";
    json["signature"] = sign;

    auto client = HttpClient::newHttpClient("https://api-mop.chinaums.com");
    auto req1 = HttpRequest::newHttpRequest();
    req1->setMethod(drogon::Post);
    req1->setPath("/v1/token/access");
    req1->setContentTypeString("application/json");
    req1->setBody(json.toStyledString());
    auto res1 = client->sendRequest(req1);

    Json::Reader reader;
    if (reader.parse(res1.second->body().data(), json)) {
        if (json.isMember("errCode")) {
            auto res = json["errCode"].asString() == "0000" ? HttpResponse::newHttpJsonResponse(json["accessToken"].asString()) : HttpResponse::newHttpJsonResponse(json);
            callback(res);
        }
        else {
            auto res = HttpResponse::newNotFoundResponse();
            callback(res);
        }
    }
    else {
        auto res = HttpResponse::newNotFoundResponse();
        callback(res);
    }
}
