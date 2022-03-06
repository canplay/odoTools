#pragma once

#include <drogon/HttpController.h>

using namespace drogon;

namespace api
{
	namespace v1
	{
		class Token : public drogon::HttpController<Token>
		{
		public:
			METHOD_LIST_BEGIN
				METHOD_ADD(Token::generate, "/generate", Post);
			METHOD_LIST_END

		public:
			Token();

			void generate(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;

		private:
			std::string sha256(const std::string str);
		};
	}
}
