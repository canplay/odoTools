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
				METHOD_ADD(User::register, "/register", Post);
			METHOD_LIST_END

		public:
			User();

			void register(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;
		};
	}
}
