#pragma once

#include <drogon/HttpController.h>

using namespace drogon;

namespace api
{
	class Settings : public HttpController<Settings>
	{
	public:
		METHOD_LIST_BEGIN
			METHOD_ADD(Settings::url, "/url", Options, Get, "api::LogFilter");
		METHOD_ADD(Settings::version, "/version", Options, Get, "api::LogFilter");
		METHOD_LIST_END

	public:
		void url(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;

		void version(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;
	};
}
