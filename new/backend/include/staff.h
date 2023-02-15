#pragma once

#include <drogon/HttpController.h>

using namespace drogon;

namespace api
{
	class Staff : public HttpController<Staff>
	{
	public:
		METHOD_LIST_BEGIN
			METHOD_ADD(Staff::info, "/info", Options, Post, "api::JwtFilter");
		METHOD_LIST_END

	public:
		void info(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;
	};
}