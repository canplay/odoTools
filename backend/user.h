#pragma once

#include <drogon/HttpController.h>

using namespace drogon;

namespace api
{
	namespace v1
	{
		class User : public drogon::HttpController<User>
		{
		public:
			METHOD_LIST_BEGIN
				METHOD_ADD(User::reg, "/register", Post);
			METHOD_ADD(User::login, "/login", Post);
			METHOD_LIST_END

		public:
			User();

			void reg(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;
			void login(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;
		};
	}
}
